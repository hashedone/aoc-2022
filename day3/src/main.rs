use std::io::BufRead;

type Ransac = Vec<u8>;

fn input() -> Vec<Ransac> {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(String::into_bytes)
        .collect()
}

fn prio(item: u8) -> u128 {
    if (b'a'..=b'z').contains(&item) {
        (item - b'a' + 1) as u128
    } else {
        (item - b'A' + 27) as u128
    }
}

fn part1(input: &[Ransac]) -> u128 {
    let mut input = input.to_vec();

    input
        .iter_mut()
        .filter_map(|ransack0| {
            let mut ransack1 = ransack0.split_off(ransack0.len() / 2);
            ransack1.sort();

            ransack0
                .iter()
                .find(|fst| ransack1.binary_search(fst).is_ok())
                .copied()
        })
        .map(prio)
        .sum()
}

fn part2(input: &[Ransac]) -> u128 {
    let mut input = input.to_vec();

    input
        .chunks_mut(3)
        .filter_map(|ransack| {
            ransack[1].sort();
            ransack[2].sort();

            ransack[0]
                .iter()
                .find(|fst| {
                    ransack[1].binary_search(fst).is_ok() && ransack[2].binary_search(fst).is_ok()
                })
                .copied()
        })
        .map(prio)
        .sum()
}

fn main() {
    let input = input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
