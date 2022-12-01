use anyhow::Result;
use std::io::BufRead;

fn input() -> Result<Vec<u128>> {
    let (mut max, buf) =
        std::io::stdin()
            .lock()
            .lines()
            .fold(Ok((vec![], 0)), |buf, line| -> Result<_> {
                let (mut callories, buf) = buf?;
                let line = line?;

                if line.is_empty() {
                    callories.push(buf);
                    Ok((callories, 0))
                } else {
                    let cals: u128 = line.parse().unwrap();
                    Ok((callories, buf + cals))
                }
            })?;

    max.push(buf);
    max.sort();
    Ok(max)
}

fn part1(input: &[u128]) -> u128 {
    *input.iter().last().unwrap()
}

fn part2(input: &[u128]) -> u128 {
    input.iter().rev().take(3).sum()
}

fn main() -> Result<()> {
    let input = input()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
