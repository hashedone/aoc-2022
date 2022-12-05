use std::io::BufRead;

use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char as char_, digit1, multispace0, multispace1};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, terminated, tuple};
use nom::{IResult, Parser};

type Stack = Vec<char>;

#[derive(Clone, Debug)]
struct Command {
    count: usize,
    source: usize,
    target: usize,
}

#[derive(Clone, Debug)]
struct Input {
    stacks: Vec<Stack>,
    program: Vec<Command>,
}

fn some_crate_parser(input: &str) -> IResult<&str, Option<char>> {
    let (i, c) = delimited(char_('['), take(1usize), char_(']'))(input)?;
    Ok((i, c.chars().next()))
}

fn crate_(input: &str) -> IResult<&str, Option<char>> {
    let none_parser = tag("   ").map(|_| None);

    alt((some_crate_parser, none_parser))(input)
}

fn crates_lines(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    many0(terminated(separated_list0(char_(' '), crate_), char_('\n')))(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<Stack>> {
    let description_line = delimited(
        multispace0,
        separated_list0(multispace1, digit1),
        multispace0,
    );

    let (i, lines) = terminated(crates_lines, description_line)(input)?;

    let cnt = match lines.get(0) {
        Some(v) => v.len(),
        None => return Ok((i, vec![])),
    };

    let mut stack = lines
        .into_iter()
        .fold(vec![vec![]; cnt], |mut stacks, line| {
            for (stack, item) in stacks.iter_mut().zip(line) {
                if let Some(c) = item {
                    stack.push(c);
                }
            }

            stacks
        });

    for s in stack.iter_mut() {
        s.reverse();
    }

    Ok((i, stack))
}

fn command(input: &str) -> IResult<&str, Command> {
    let (i, (_, count, _, source, _, target)) = tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ))(input)?;

    let count: usize = count.parse().unwrap();
    let source: usize = source.parse().unwrap();
    let target: usize = target.parse().unwrap();

    Ok((
        i,
        Command {
            count,
            source: source - 1,
            target: target - 1,
        },
    ))
}

fn program(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(char_('\n'), command)(input)
}

fn input() -> Result<Input> {
    let s: Vec<_> = std::io::stdin().lock().lines().collect::<Result<_, _>>()?;
    let s = s.join("\n");

    let (_, (stacks, program)) = tuple((stacks, program))(&s).map_err(|err| err.to_owned())?;

    Ok(Input { stacks, program })
}

fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for command in &input.program {
        let idx = stacks[command.source].len() - command.count;
        let mut moved = stacks[command.source].split_off(idx);
        moved.reverse();
        stacks[command.target].append(&mut moved);
    }

    stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect()
}

fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for command in &input.program {
        let idx = stacks[command.source].len() - command.count;
        let mut moved = stacks[command.source].split_off(idx);
        stacks[command.target].append(&mut moved);
    }

    stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect()
}

fn main() -> Result<()> {
    let input = input()?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_parsing() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
"#;

        let (i, lines) = crates_lines(input).unwrap();
        assert_eq!(i, "");
        assert_eq!(
            lines,
            vec![
                vec![None, Some('D'), None],
                vec![Some('N'), Some('C'), None],
                vec![Some('Z'), Some('M'), Some('P')],
            ]
        );
    }

    #[test]
    fn stacks_parsing() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

"#;

        let (i, stacks) = stacks(input).unwrap();

        assert_eq!(i, "");
        assert_eq!(
            stacks,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
    }
}
