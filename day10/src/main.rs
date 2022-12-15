enum Instruction {
    Addx(i128),
    Noop,
}

fn input() -> Vec<Instruction> {
    std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next()? {
                "addx" => {
                    let v: i128 = parts.next()?.parse().ok()?;
                    Some(Instruction::Addx(v))
                }
                "noop" => Some(Instruction::Noop),
                _ => None,
            }
        })
        .collect()
}

fn execute(program: &[Instruction]) -> impl Iterator<Item = i128> + '_ {
    use Instruction::*;

    program
        .iter()
        .scan(1, |x, instr| match instr {
            Addx(v) => {
                let old = *x;
                *x += v;
                Some(vec![old; 2])
            }
            Noop => Some(vec![*x]),
        })
        .flatten()
}

fn part1(program: &[Instruction]) -> i128 {
    execute(program)
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| ((i + 1) as i128) * x)
        .sum()
}

fn part2(program: &[Instruction]) -> Vec<String> {
    let display: Vec<_> = execute(program)
        .enumerate()
        .map(|(i, x)| match ((i % 40) as i128 - x).abs() <= 1 {
            true => '#',
            false => '.',
        })
        .collect();

    display
        .chunks(40)
        .map(|line| line.iter().collect())
        .collect()
}

fn main() {
    let input = input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    for line in part2(&input) {
        println!("{}", line);
    }
}
