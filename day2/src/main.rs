use anyhow::{anyhow, bail, Result};
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(self) -> u128 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome(self, enemy: Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match (self, enemy) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn deduce(self, expected: Outcome) -> Self {
        use Outcome::*;
        use Shape::*;

        match (self, expected) {
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Hint {
    X,
    Y,
    Z,
}

impl Hint {
    fn shape(self) -> Shape {
        use Hint::*;
        use Shape::*;

        match self {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        }
    }

    fn outcome(self) -> Outcome {
        use Hint::*;
        use Outcome::*;

        match self {
            X => Lose,
            Y => Draw,
            Z => Win,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Round {
    enemy: Shape,
    me: Hint,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn points(self) -> u128 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

fn input() -> Result<Vec<Round>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| -> Result<_> {
            use Hint::*;
            use Shape::*;

            let line = line?;
            let mut split = line.split(' ');
            let player1 = match split.next().ok_or_else(|| anyhow!("No enemy play"))? {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => bail!("Invalid enemy play"),
            };

            let player2 = match split.next().ok_or_else(|| anyhow!("No my play"))? {
                "X" => X,
                "Y" => Y,
                "Z" => Z,
                _ => bail!("Invalid my play"),
            };

            if split.next().is_some() {
                bail!("Unexpected input");
            }

            Ok(Round {
                enemy: player1,
                me: player2,
            })
        })
        .collect()
}

fn part1(input: &[Round]) -> u128 {
    input
        .iter()
        .map(|round| round.me.shape().outcome(round.enemy).points() + round.me.shape().points())
        .sum()
}

fn part2(input: &[Round]) -> u128 {
    input
        .iter()
        .map(|round| {
            let outcome = round.me.outcome();
            round.enemy.deduce(round.me.outcome()).points() + outcome.points()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = input()?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}
