use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::io::BufRead;

#[derive(Debug)]
struct Map {
    map: Vec<u8>,
    width: usize,
    start: usize,
    end: usize,
}

impl Map {
    fn neighbors(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        [
            Some(pos.wrapping_sub(self.width)).filter(move |_| pos >= self.width),
            Some(pos + self.width).filter(move |p| *p < self.map.len()),
            Some(pos.wrapping_sub(1)).filter(move |_| pos % self.width != 0),
            Some(pos + 1).filter(move |p| p % self.width != 0),
        ]
        .into_iter()
        .flatten()
        .filter(move |p| self.map[*p] <= self.map[pos] + 1)
    }
}

#[derive(PartialEq, Eq)]
struct D(usize, usize);

impl PartialOrd for D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.0, other.0) {
            (usize::MAX, _) => self.1.cmp(&other.1),
            (_, usize::MAX) => Ordering::Less,
            _ => (self.0, self.1).cmp(&(other.0, other.1)),
        }
        .into()
    }
}

impl Ord for D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn input() -> Option<Map> {
    let mut lines = std::io::stdin().lock().lines().peekable();
    let width = lines.peek()?.as_ref().ok()?.len();

    let mut map: Vec<_> = lines
        .filter_map(|l| l.ok())
        .flat_map(|m| m.into_bytes())
        .collect();

    let start = map.iter().position(|&c| c == b'S')?;
    let end = map.iter().position(|&c| c == b'E')?;

    for h in &mut map {
        *h = match *h {
            b'S' => 0,
            b'E' => b'z' - b'a',
            h => h - b'a',
        };
    }

    Map {
        map,
        width,
        start,
        end,
    }
    .into()
}

fn part1(map: &Map) -> usize {
    let mut distance = vec![usize::MAX; map.map.len()];
    distance[map.start] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, map.start)));

    while let Some(Reverse((d, pos))) = queue.pop() {
        for n in map.neighbors(pos) {
            if distance[n] > d + 1 {
                distance[n] = d + 1;
                queue.push(Reverse((d + 1, n)));
            }
        }
    }

    distance[map.end]
}

fn part2(map: &Map) -> usize {
    let mut distance: Vec<_> = map
        .map
        .iter()
        .map(|&h| match h {
            0 => 0,
            _ => usize::MAX,
        })
        .collect();

    let mut queue: BinaryHeap<_> = distance
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d == 0)
        .map(|(i, _)| Reverse((0, i)))
        .collect();

    while let Some(Reverse((d, pos))) = queue.pop() {
        for n in map.neighbors(pos) {
            if distance[n] > d + 1 {
                distance[n] = d + 1;
                queue.push(Reverse((d + 1, n)));
            }
        }
    }

    distance[map.end]
}

fn part12(map: &Map) -> (usize, usize) {
    let mut distance: Vec<_> = map
        .map
        .iter()
        .map(|&h| match h {
            0 => (usize::MAX, 0),
            _ => (usize::MAX, usize::MAX),
        })
        .collect();

    distance[map.start] = (0, 0);

    //    let mut queue = BinaryHeap::new();
    //    queue.push(Reverse((0, 0, map.start)));

    let mut queue: BinaryHeap<_> = distance
        .iter()
        .enumerate()
        .filter(|&(_, &(d0, _))| d0 == 0)
        .map(|(i, (d0, ds))| Reverse((D(*d0, *ds), i)))
        .collect();

    while let Some(Reverse((D(ds, d0), pos))) = queue.pop() {
        for n in map.neighbors(pos) {
            let mut p = false;

            if ds != usize::MAX && distance[n].0 > ds + 1 {
                distance[n].0 = ds + 1;
                p = true;
            }

            if distance[n].1 > d0 + 1 {
                distance[n].1 = d0 + 1;
                p = true;
            }

            if p {
                queue.push(Reverse((D(distance[n].0, distance[n].1), n)));
            }
        }
    }

    distance[map.end]
}

fn main() {
    let input = input().unwrap();
    let t = std::time::Instant::now();
    let p1 = part1(&input);
    let t1 = t.elapsed();
    let p2 = part2(&input);
    let t2 = t.elapsed();
    let (p22, p12) = part12(&input);
    let t3 = t.elapsed();
    println!("Part1: {p1}, time: {t1:?}");
    println!("Part2: {p2}, time: {:?}", t2 - t1);
    println!("Combined: {p12}, {p22}, time: {:?}", t3 - t2);
    println!("P1 + P2 time: {t2:?}");
    println!("Total time: {t3:?}");
}
