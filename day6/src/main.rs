use anyhow::Result;

fn input() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn solution(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .position(|w| (0..w.len()).all(|i| !w[i + 1..].contains(&w[i])))
        .map(|i| i + n)
        .unwrap_or(0)
}

fn main() {
    let s = input().unwrap();
    println!("Part 1: {}", solution(&s, 4));
    println!("Part 2: {}", solution(&s, 14));
}
