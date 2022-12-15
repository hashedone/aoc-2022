#![feature(iter_array_chunks)]

use derivative::Derivative;

type OpFn = dyn Fn(u128) -> u128;

#[derive(Derivative)]
#[derivative(Debug)]
struct MonkeyDesc {
    #[derivative(Debug = "ignore")]
    op: Box<OpFn>,
    test: u128,
    tbranch: usize,
    fbranch: usize,
}

#[derive(Debug, Clone)]
struct MonkeyState {
    items: Vec<u128>,
    inspections: usize,
}

fn parse_starting_items(line: &str) -> Vec<u128> {
    let items = match line.split_once(':') {
        Some((_, items)) => items.trim(),
        None => return vec![],
    };

    items
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn parse_op(line: &str) -> Option<Box<dyn Fn(u128) -> u128>> {
    let (_, op) = line.split_once("old")?;

    let (op, val) = op.trim().split_once(' ')?;

    let res = match (op.trim(), val) {
        ("+", "old") => Box::new(|x| x * 2u128) as Box<OpFn>,
        ("*", "old") => Box::new(|x| x * x),
        ("+", val) => {
            let val: u128 = val.parse().ok()?;
            Box::new(move |x| x + val)
        }
        ("*", val) => {
            let val: u128 = val.parse().ok()?;
            Box::new(move |x| x * val)
        }
        _ => return None,
    };

    Some(res)
}

fn parse_test(line: &str) -> Option<u128> {
    let (_, test) = line.trim().rsplit_once(' ')?;
    test.parse().ok()
}

fn parse_branch(line: &str) -> Option<usize> {
    let (_, test) = line.trim().rsplit_once(' ')?;
    test.parse().ok()
}

fn input() -> (Vec<MonkeyDesc>, Vec<MonkeyState>) {
    std::io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .array_chunks()
        .filter_map(|[_monkey, starting, op, test, tbranch, fbranch]| {
            let items = parse_starting_items(&starting);
            let op = parse_op(&op)?;
            let test = parse_test(&test)?;
            let tbranch = parse_branch(&tbranch)?;
            let fbranch = parse_branch(&fbranch)?;

            let desc = MonkeyDesc {
                op,
                test,
                tbranch,
                fbranch,
            };

            let state = MonkeyState {
                items,
                inspections: 0,
            };

            Some((desc, state))
        })
        .unzip()
}

fn part1(descs: &[MonkeyDesc], states: Vec<MonkeyState>) -> usize {
    let mut inspections: Vec<_> = std::iter::successors(Some(states), |prev| {
        let mut state = prev.clone();

        for idx in 0..prev.len() {
            let monkey = &mut state[idx];
            let desc = &descs[idx];

            monkey.inspections += monkey.items.len();

            for item in &mut monkey.items {
                *item = (descs[idx].op)(*item) / 3;
            }

            let (t, f): (Vec<_>, Vec<_>) =
                monkey.items.drain(..).partition(|x| *x % desc.test == 0);

            state[desc.tbranch].items.extend(t);
            state[desc.fbranch].items.extend(f);
        }

        Some(state)
    })
    .nth(20)
    .unwrap_or_default()
    .into_iter()
    .map(|s| s.inspections)
    .collect();

    inspections.select_nth_unstable_by(1, |a, b| b.cmp(a));

    inspections[..2].iter().product()
}

fn part2(descs: &[MonkeyDesc], states: Vec<MonkeyState>) -> usize {
    let m = descs.iter().map(|d| d.test).product::<u128>();

    let mut inspections: Vec<_> = std::iter::successors(Some(states), |prev| {
        let mut state = prev.clone();

        for idx in 0..prev.len() {
            let monkey = &mut state[idx];
            let desc = &descs[idx];

            monkey.inspections += monkey.items.len();

            for item in &mut monkey.items {
                *item = (descs[idx].op)(*item) % m;
            }

            let (t, f): (Vec<_>, Vec<_>) =
                monkey.items.drain(..).partition(|x| *x % desc.test == 0);

            state[desc.tbranch].items.extend(t);
            state[desc.fbranch].items.extend(f);
        }

        Some(state)
    })
    .nth(10000)
    .unwrap_or_default()
    .into_iter()
    .map(|s| s.inspections)
    .collect();

    inspections.select_nth_unstable_by(1, |a, b| b.cmp(a));

    inspections[..2].iter().product()
}

fn main() {
    let (desc, states) = input();

    println!("Part 1: {}", part1(&desc, states.clone()));
    println!("Part 2: {}", part2(&desc, states));
}
