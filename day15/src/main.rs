type Pos = (i128, i128);
type Entry = (Pos, Pos);

fn input() -> Vec<Entry> {
    std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let (sensor, beacon) = line.trim().split_once(':').unwrap();

            let (x, y) = sensor.trim().split_once(',').unwrap();
            let (_, x) = x.trim().split_once('=').unwrap();
            let x: i128 = x.trim().parse().unwrap();
            let (_, y) = y.trim().split_once('=').unwrap();
            let y: i128 = y.trim().parse().unwrap();
            let sensor = (x, y);

            let (x, y) = beacon.trim().split_once(',').unwrap();
            let (_, x) = x.trim().split_once('=').unwrap();
            let x: i128 = x.trim().parse().unwrap();
            let (_, y) = y.trim().split_once('=').unwrap();
            let y: i128 = y.trim().parse().unwrap();
            let beacon = (x, y);

            (sensor, beacon)
        })
        .collect()
}

fn dist((x1, y1): Pos, (x2, y2): Pos) -> i128 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(entries: &[Entry]) -> usize {
    let range = (entries[0].0 .0, entries[0].0 .0);

    let (x0, x1) = entries
        .iter()
        .copied()
        .fold(range, |(x0, x1), ((sx, sy), beacon)| {
            let d = dist((sx, sy), beacon);
            (x0.min(sx - d), x1.max(sx + d))
        });

    const Y: i128 = 2000000;
    let line = vec![false; (x1 - x0 + 1) as usize];

    let idx = |x: i128| (x - x0) as usize;

    let line = entries
        .iter()
        .copied()
        .fold(line, |mut line, ((sx, sy), beacon)| {
            let d = dist((sx, sy), beacon);
            let a = d - (sy - Y).abs();

            for x in (sx - a)..=(sx + a) {
                line[idx(x)] = true;
            }

            line
        });

    let line = entries.iter().filter(|(_, (_, y))| *y == Y).copied().fold(
        line,
        |mut line, (_, (x, _))| {
            line[idx(x)] = false;
            line
        },
    );

    line.iter().filter(|&&b| b).count()
}

fn part2(entries: &[Entry]) -> i128 {
    const A: i128 = 4000000;

    let mut sonars: Vec<_> = entries
        .iter()
        .map(|((x, y), beacon)| ((*x, *y), dist((*x, *y), *beacon)))
        .collect();

    sonars.sort_by_key(|&(_, d)| -(d as i64));
    let (x, y) = sonars
        .iter()
        .flat_map(|((x, y), d)| {
            let d = d + 1;

            let xmin = (x - d).max(0);
            let ymin = (y - d).max(0);

            (xmin..=(x - d + y - ymin).max(A)).map(move |px| (px, y - d + x - px))
        })
        .filter(|(x, y)| (0..=A).contains(x) && (0..=A).contains(y))
        .find(|(x, y)| {
            sonars
                .iter()
                .all(|((sx, sy), d)| dist((*x, *y), (*sx, *sy)) > *d)
        })
        .unwrap();

    const M: i128 = 4000000;
    M * x + y
}

fn main() {
    let input = input();
    //    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
