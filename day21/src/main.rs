use std::collections::HashMap;
use std::io::BufRead;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Op<Idx> {
    Num(i64),
    Add(Idx, Idx),
    Sub(Idx, Idx),
    Mul(Idx, Idx),
    Div(Idx, Idx),
    Deref,
}

impl<Idx> Op<Idx> {
    fn map<T>(self, f: impl Fn(Idx) -> T) -> Op<T> {
        use Op::*;

        match self {
            Num(n) => Num(n),
            Add(a0, a1) => Add(f(a0), f(a1)),
            Sub(a0, a1) => Sub(f(a0), f(a1)),
            Mul(a0, a1) => Mul(f(a0), f(a1)),
            Div(a0, a1) => Div(f(a0), f(a1)),
            Deref => Deref,
        }
    }
}

fn eval(tree: &[Op<usize>], root: usize) -> i64 {
    let mut values = vec![];
    let mut stack = vec![root];

    while let Some(idx) = stack.last().copied() {
        //        println!("{stack:?}");
        //        println!("{:?}", monkeys[idx]);
        //        println!("{values:?}");
        let mut perform = |idx: usize, i0: usize, i1: usize, f: fn(i64, i64) -> i64| {
            let (n0, n1) = (
                values.get(i0).and_then(Option::as_ref).copied(),
                values.get(i1).and_then(Option::as_ref).copied(),
            );

            if let (Some(n0), Some(n1)) = (n0, n1) {
                if values.len() <= idx {
                    values.resize(idx + 1, None);
                }
                values[idx] = Some(f(n0, n1));
                stack.pop();
            }

            if n0.is_none() {
                stack.push(i0);
            }

            if n1.is_none() {
                stack.push(i1);
            }
        };

        match &tree[idx] {
            Op::Num(n) => {
                if values.len() <= idx {
                    values.resize(idx + 1, None);
                }
                values[idx] = Some(*n);
                stack.pop();
            }
            Op::Add(i0, i1) => perform(idx, *i0, *i1, |a, b| a + b),
            Op::Sub(i0, i1) => perform(idx, *i0, *i1, |a, b| a - b),
            Op::Mul(i0, i1) => perform(idx, *i0, *i1, |a, b| a * b),
            Op::Div(i0, i1) => perform(idx, *i0, *i1, |a, b| {
                assert_eq!(a % b, 0);
                a / b
            }),
            _ => panic!("Deref cannot be evaluated - calculation loop? {idx}"),
        }
    }

    values[root].unwrap()
}

fn main() {
    let t0 = Instant::now();
    let mut monkeys: HashMap<String, Op<String>> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (idx, args) = line.trim().split_once(':').unwrap();
            let idx = idx.to_owned();
            let mut args = args.split_whitespace();
            let arg0 = args.next().unwrap();

            if let Ok(arg) = arg0.parse::<i64>() {
                return (idx, Op::Num(arg));
            }

            let op = args.next().unwrap();
            let arg1 = args.next().unwrap();

            let arg0 = arg0.to_owned();
            let arg1 = arg1.to_owned();
            match op {
                "+" => (idx, Op::Add(arg0, arg1)),
                "-" => (idx, Op::Sub(arg0, arg1)),
                "*" => (idx, Op::Mul(arg0, arg1)),
                _ => (idx, Op::Div(arg0, arg1)),
            }
        })
        .collect();

    let t1 = t0.elapsed();
    let mut names: Vec<_> = monkeys.keys().cloned().collect();
    names.sort();

    let monkeys: Vec<_> = names
        .iter()
        .map(|name| {
            monkeys
                .remove(name)
                .unwrap()
                .map(|name| names.binary_search(&name).unwrap())
        })
        .collect();

    let root = names.binary_search(&"root".to_owned()).unwrap();
    let humn = names.binary_search(&"humn".to_owned()).unwrap();

    let t2 = t0.elapsed();
    let p1 = eval(&monkeys, root);

    let t3 = t0.elapsed();
    let mut transformed = vec![None; monkeys.len()];

    let (i0, i1) = match monkeys[root] {
        Op::Add(i0, i1) => (i0, i1),
        Op::Sub(i0, i1) => (i0, i1),
        Op::Mul(i0, i1) => (i0, i1),
        Op::Div(i0, i1) => (i0, i1),
        _ => unreachable!(),
    };
    let mut stack = vec![i0, i1];

    while let Some(idx) = stack.last().copied() {
        //        println!("{stack:?}");
        //        println!("{:?}", monkeys[idx]);
        //        println!("{values:?}");
        let mut perform =
            |idx: usize,
             i0: usize,
             i1: usize,
             f: fn(i64, i64) -> i64,
             lt: fn(/* parent */ usize, /* i1 */ usize) -> Op<usize>,
             rt: fn(/* parent */ usize, /* i0 */ usize) -> Op<usize>| {
                // Two cases for `humn` being one of the arguments - in such case we can just
                // fix its tree - it shoul never be evaluated
                if i0 == humn {
                    transformed[idx] = Some(Op::Deref);
                    transformed[i1] = Some(Op::Num(eval(&monkeys, i1)));
                    //                    transformed[i1] = Some(monkeys[i1].clone());
                    transformed[i0] = Some(lt(idx, i1));
                    stack.pop();
                    return;
                }

                if i1 == humn {
                    transformed[idx] = Some(Op::Deref);
                    transformed[i0] = Some(Op::Num(eval(&monkeys, i0)));
                    //                    transformed[i0] = Some(monkeys[i0].clone());
                    transformed[i1] = Some(rt(idx, i0));
                    stack.pop();
                    return;
                }

                let (n0, n1) = (
                    transformed.get(i0).and_then(Option::as_ref).cloned(),
                    transformed.get(i1).and_then(Option::as_ref).cloned(),
                );

                match (n0, n1) {
                    // If both sides are calculated, we can just calculate
                    (Some(Op::Num(n0)), Some(Op::Num(n1))) => {
                        transformed[idx] = Some(Op::Num(f(n0, n1)));
                        stack.pop();
                    }
                    // Very special case - human lives on both expression sides
                    (Some(Op::Deref), Some(Op::Deref)) => panic!("double deref {idx} {i0} {i1}"),
                    // If one side is calculated, and the other side is `Deref`, we can calculate
                    // the value of the `Deref` side in terms of the other side, but our node
                    // becomes `Deref`
                    (Some(Op::Deref), Some(_)) => {
                        transformed[idx] = Some(Op::Deref);
                        transformed[i0] = Some(lt(idx, i1));
                        stack.pop();
                    }
                    (Some(_), Some(Op::Deref)) => {
                        transformed[idx] = Some(Op::Deref);
                        transformed[i1] = Some(rt(idx, i0));
                        stack.pop();
                    }
                    // I don't think it is ever a case, but if we have tree build up of some other
                    // trees, we will forward it - maybe there is some strange case, but it looks
                    // like it means a deref loop
                    (Some(_), Some(_)) => {
                        transformed[idx] = Some(monkeys[idx].clone());
                        stack.pop();
                    }
                    // Cases for calculating subnodes
                    (None, Some(_)) => {
                        stack.push(i0);
                    }
                    (Some(_), None) => {
                        stack.push(i1);
                    }
                    (None, None) => {
                        stack.push(i0);
                        stack.push(i1);
                    }
                }
            };

        match &monkeys[idx] {
            Op::Num(n) => {
                transformed[idx] = Some(Op::Num(*n));
                stack.pop();
            }
            Op::Add(i0, i1) => perform(idx, *i0, *i1, |a, b| a + b, Op::Sub, Op::Sub),
            Op::Sub(i0, i1) => {
                perform(idx, *i0, *i1, |a, b| a - b, Op::Add, |p, i0| Op::Sub(i0, p))
            }
            Op::Mul(i0, i1) => perform(idx, *i0, *i1, |a, b| a * b, Op::Div, Op::Div),
            Op::Div(i0, i1) => {
                perform(idx, *i0, *i1, |a, b| a / b, Op::Mul, |p, i0| Op::Div(i0, p))
            }
            _ => unreachable!(),
        }
    }

    match (transformed[i0].clone(), transformed[i1].clone()) {
        (Some(Op::Deref), Some(tree)) => transformed[i0] = Some(tree),
        (Some(tree), Some(Op::Deref)) => transformed[i1] = Some(tree),
        _ => unreachable!(),
    };

    let transformed: Vec<_> = transformed
        .into_iter()
        .map(|op| op.unwrap_or(Op::Deref))
        .collect();

    let p2 = eval(&transformed, humn);
    let t4 = t0.elapsed();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!(
        "Times: input {t1:?}, preprocess {:?}, p1 {:?}, p2 {:?}, total {t4:?}",
        t2 - t1,
        t3 - t2,
        t4 - t3
    );

    //    println!("{humn} {root}");
    //    for d in transformed.iter().enumerate() {
    //        println!("{d:?}");
    //    }
    //    println!("Monkeys:");
    //    for d in monkeys.iter().enumerate() {
    //        println!("{d:?}");
    //    }
}
