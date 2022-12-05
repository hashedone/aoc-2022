use anyhow::{anyhow, Result};
use std::io::BufRead;
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u128>;

fn parse_range(rng: &str) -> Result<Assignment> {
    let mut parts = rng.split('-');
    let start = parts.next().ok_or_else(|| anyhow!(""))?.parse()?;
    let end = parts.next().ok_or_else(|| anyhow!(""))?.parse()?;
    Ok(start..=end)
}

fn input() -> Vec<[Assignment; 2]> {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| -> Result<_> {
            let mut parts = line.split(',');

            let fst = parts.next().ok_or_else(|| anyhow!(""))?;
            let fst = parse_range(fst)?;

            let snd = parts.next().ok_or_else(|| anyhow!(""))?;
            let snd = parse_range(snd)?;

            Ok([fst, snd])
        })
        .filter_map(Result::ok)
        .collect()
}

fn part1(input: &[[Assignment; 2]]) -> usize {
    input
        .iter()
        .filter(|[fst, snd]| {
            let c0 = fst.contains(snd.start()) && fst.contains(snd.end());
            let c1 = snd.contains(fst.start()) && snd.contains(fst.end());
            c0 || c1
        })
        .count()
}

fn part2(input: &[[Assignment; 2]]) -> usize {
    input
        .iter()
        .filter(|[fst, snd]| {
            let c0 = fst.contains(snd.start()) || fst.contains(snd.end());
            let c1 = snd.contains(fst.start()) || snd.contains(fst.end());
            c0 || c1
        })
        .count()
}

fn main() {
    let input = input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
