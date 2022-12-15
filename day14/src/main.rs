use std::io::BufRead;

fn input() -> Vec<Vec<(usize, usize)>> {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.trim()
                .split("->")
                .filter_map(|pos| {
                    let (x, y) = pos.split_once(',')?;
                    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
                })
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<(usize, usize)>]) -> usize {
    let maxx = input
        .iter()
        .flat_map(|v| v.iter())
        .max_by_key(|(x, _)| *x)
        .unwrap()
        .0
        .max(500);

    let maxy = input
        .iter()
        .flat_map(|v| v.iter())
        .max_by_key(|(_, y)| *y)
        .unwrap()
        .1;

    let mut surface = vec![false; (maxx + 1) * (maxy + 1)];

    let idx = |x: usize, y: usize| y * (maxx + 1) + x;

    for line in input {
        for segment in line.windows(2) {
            let (x0, y0) = segment[0];
            let (x1, y1) = segment[1];

            if x0 == x1 {
                for y in y0.min(y1)..=y0.max(y1) {
                    surface[idx(x0, y)] = true;
                }
            } else {
                for x in x0.min(x1)..=x0.max(x1) {
                    surface[idx(x, y0)] = true;
                }
            }
        }
    }

    std::iter::from_fn(|| {
        let (x, y) = std::iter::successors(Some((500, 0)), |(x, y)| {
            if *y > maxy {
                None
            } else if *y == maxy {
                (*x, *y + 1).into()
            } else if !surface[idx(*x, *y + 1)] {
                Some((*x, *y + 1))
            } else if *x == 0 {
                Some((*x, maxy + 1))
            } else if !surface[idx(*x - 1, *y + 1)] {
                Some((*x - 1, *y))
            } else if *x > maxx {
                Some((*x, maxy + 1))
            } else if !surface[idx(*x + 1, *y + 1)] {
                Some((*x + 1, *y))
            } else {
                None
            }
        })
        .last()
        .unwrap();

        if y > maxy {
            None
        } else {
            surface[idx(x, y)] = true;

            Some(())
        }
    })
    .count()
}

fn part2(input: &[Vec<(usize, usize)>]) -> usize {
    let maxy = input
        .iter()
        .flat_map(|v| v.iter())
        .max_by_key(|(_, y)| *y)
        .unwrap()
        .1
        + 1;

    let maxx = input
        .iter()
        .flat_map(|v| v.iter())
        .max_by_key(|(x, _)| *x)
        .unwrap()
        .0
        .max(500 + maxy + 1);

    let mut surface = vec![false; (maxx + 1) * (maxy + 1)];

    let idx = |x: usize, y: usize| y * (maxx + 1) + x;

    for line in input {
        for segment in line.windows(2) {
            let (x0, y0) = segment[0];
            let (x1, y1) = segment[1];

            if x0 == x1 {
                for y in y0.min(y1)..=y0.max(y1) {
                    surface[idx(x0, y)] = true;
                }
            } else {
                for x in x0.min(x1)..=x0.max(x1) {
                    surface[idx(x, y0)] = true;
                }
            }
        }
    }

    std::iter::from_fn(|| {
        let (x, y) = std::iter::successors(Some((500, 0)), |(x, y)| {
            if *y == maxy {
                None
            } else if !surface[idx(*x, *y + 1)] {
                Some((*x, *y + 1))
            } else if !surface[idx(*x - 1, *y + 1)] {
                Some((*x - 1, *y))
            } else if !surface[idx(*x + 1, *y + 1)] {
                Some((*x + 1, *y))
            } else {
                None
            }
        })
        .last()
        .unwrap();

        if (x, y) == (500, 0) {
            None
        } else {
            surface[idx(x, y)] = true;

            Some(())
        }
    })
    .count()
        + 1
}

fn main() {
    let input = input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
