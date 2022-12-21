use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let data: HashSet<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let vs: Vec<i64> = l.trim().split(',').map(|n| n.parse().unwrap()).collect();
            (vs[0], vs[1], vs[2])
        })
        .collect();

    let hidden: usize = data
        .iter()
        .map(|(x, y, z)| {
            let (x, y, z) = (*x, *y, *z);

            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            .into_iter()
            .filter(|&(x, y, z)| data.contains(&(x, y, z)))
            .count()
        })
        .sum();

    println!("Part 1: {}", data.len() * 6 - hidden);

    let minx = *data.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let miny = *data.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let minz = *data.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let maxx = *data.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let maxy = *data.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let maxz = *data.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut visited = data.clone();
    let mut queue: Vec<(i64, i64, i64)> = vec![];
    let mut cnt = 0;

    queue.push((minx, miny, minz));

    while let Some((x, y, z)) = queue.pop() {
        if !visited.insert((x, y, z)) {
            continue;
        }

        let nb: Vec<_> = [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
        .into_iter()
        .filter(|(x, y, z)| {
            (minx..=maxx).contains(x) && (miny..=maxy).contains(y) && (minz..=maxz).contains(z)
        })
        .collect();

        cnt += nb
            .iter()
            .copied()
            .filter(|&(x, y, z)| data.contains(&(x, y, z)))
            .count();

        queue.extend(nb.into_iter());
    }

    println!("Part 2: {cnt}");
}
