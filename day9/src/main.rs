use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

type Move = (Dir, i128);

fn input() -> Vec<Move> {
    std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let mut parts = line.split(' ');

            let dir = match parts.next()? {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                _ => return None,
            };

            let dist = parts.next()?.parse().ok()?;

            Some((dir, dist))
        })
        .collect()
}

fn advance(x: i128, y: i128, dir: Dir) -> (i128, i128) {
    match dir {
        Dir::U => (x, y + 1),
        Dir::D => (x, y - 1),
        Dir::L => (x - 1, y),
        Dir::R => (x + 1, y),
    }
}

fn tail(tx: i128, ty: i128, hx: i128, hy: i128) -> (i128, i128) {
    if (tx - hx).abs() <= 1 && (ty - hy).abs() <= 1 {
        (tx, ty)
    } else if tx == hx {
        let y = if ty > hy { ty - 1 } else { ty + 1 };
        (tx, y)
    } else if ty == hy {
        let x = if tx > hx { tx - 1 } else { tx + 1 };
        (x, ty)
    } else {
        let x = if tx > hx { tx - 1 } else { tx + 1 };
        let y = if ty > hy { ty - 1 } else { ty + 1 };
        (x, y)
    }
}

fn part1(input: &[Move]) -> usize {
    let positions: HashSet<_> = input
        .iter()
        .scan(((0, 0), (0, 0)), |((tx, ty), (hx, hy)), (dir, dist)| {
            Some(
                std::iter::repeat_with(|| {
                    (*hx, *hy) = advance(*hx, *hy, *dir);
                    (*tx, *ty) = tail(*tx, *ty, *hx, *hy);

                    (*tx, *ty)
                })
                .take(*dist as usize)
                .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect();

    positions.len()
}

fn part2(input: &[Move]) -> usize {
    let positions: HashSet<_> = input
        .iter()
        .scan([(0, 0); 10], |rope, (dir, dist)| {
            Some(
                std::iter::repeat_with(|| {
                    (rope[0].0, rope[0].1) = advance(rope[0].0, rope[0].1, *dir);
                    for idx in 1..10 {
                        (rope[idx].0, rope[idx].1) =
                            tail(rope[idx].0, rope[idx].1, rope[idx - 1].0, rope[idx - 1].1);
                    }

                    rope[9]
                })
                .take(*dist as usize)
                .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect();

    positions.len()
}

fn main() {
    let input = input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
