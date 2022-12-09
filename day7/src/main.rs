use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{bail, Result};
use either::Either;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::{fold_many0, many0, many1};
use nom::sequence::{delimited, preceded, tuple};
use nom::{Finish, IResult, Parser};

#[derive(Debug)]
struct FileEntry {
    name: String,
    size: u128,
}

#[derive(Debug)]
enum Command {
    Cd {
        dir: String,
    },
    CdUp,
    CdRoot,
    Ls {
        dirs: Vec<String>,
        files: Vec<FileEntry>,
    },
}

#[derive(Debug)]
struct Input {
    commands: Vec<Command>,
}

fn fsname(input: &str) -> IResult<&str, String> {
    many1(alt((alpha1, tag("."))))
        .map(|s| s.concat())
        .parse(input)
}

fn cd(input: &str) -> IResult<&str, Command> {
    delimited(tag("$ cd "), fsname, tag("\n"))
        .map(|dir| Command::Cd { dir })
        .parse(input)
}

fn cdup(input: &str) -> IResult<&str, Command> {
    tag("$ cd ..\n").map(|_| Command::CdUp).parse(input)
}

fn cdroot(input: &str) -> IResult<&str, Command> {
    tag("$ cd /\n").map(|_| Command::CdRoot).parse(input)
}

fn ls_dir_entry(input: &str) -> IResult<&str, Either<String, FileEntry>> {
    delimited(tag("dir "), fsname, tag("\n"))
        .map(Either::Left)
        .parse(input)
}

fn ls_file_entry(input: &str) -> IResult<&str, Either<String, FileEntry>> {
    map_res(
        tuple((digit1, tag(" "), fsname, tag("\n"))),
        |(size, _, name, _)| -> Result<_> {
            Ok(Either::Right(FileEntry {
                name,
                size: size.parse()?,
            }))
        },
    )
    .parse(input)
}

fn ls_result(input: &str) -> IResult<&str, (Vec<String>, Vec<FileEntry>)> {
    fold_many0(
        alt((ls_dir_entry, ls_file_entry)),
        || (vec![], vec![]),
        |(mut dirs, mut files), entry| {
            match entry {
                Either::Left(dir) => dirs.push(dir),
                Either::Right(file) => files.push(file),
            }
            (dirs, files)
        },
    )(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ ls\n"), ls_result)
        .map(|(dirs, files)| Command::Ls { dirs, files })
        .parse(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    many0(alt((ls, cdup, cdroot, cd)))
        .map(|commands| Input { commands })
        .parse(input)
}

fn input() -> Result<Input> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let (input, output) = input_parser(&input)
        .map_err(|err| err.to_owned())
        .finish()?;

    if !input.is_empty() {
        anyhow::bail!("input not fully consumed");
    }

    Ok(output)
}

fn build_file_stats(input: Input) -> HashMap<PathBuf, u128> {
    let mut paths: HashMap<PathBuf, u128> = HashMap::new();
    let mut current_path = PathBuf::new();

    for command in &input.commands {
        match command {
            Command::Cd { dir } => {
                current_path.push(dir);
                paths.entry(current_path.clone()).or_default();
            }
            Command::CdUp => {
                current_path.pop();
            }
            Command::CdRoot => {
                current_path = PathBuf::from("/");
                paths.entry(current_path.clone()).or_default();
            }
            Command::Ls { dirs, files } => {
                for dir in dirs {
                    paths.insert(current_path.join(dir), 0);
                }

                for file in files {
                    let mut p = current_path.join(&file.name);

                    while p.pop() {
                        *paths.get_mut(&p).unwrap() += file.size;
                    }
                }
            }
        }
    }

    paths
}

fn part1(stats: &HashMap<PathBuf, u128>) -> u128 {
    stats.values().filter(|size| size <= &&100000).sum()
}

fn part2(stats: &HashMap<PathBuf, u128>) -> Result<u128> {
    let used = stats
        .get(&PathBuf::from("/"))
        .ok_or_else(|| anyhow::anyhow!("no root"))?;

    let needed = 40_000_000;
    if used <= &needed {
        bail!("Already enough space");
    }

    let needed = used - needed;

    stats
        .iter()
        .filter(|(_, size)| size >= &&needed)
        .min_by_key(|(_, size)| *size)
        .map(|(_, size)| *size)
        .ok_or_else(|| anyhow::anyhow!("No path found"))
}

fn main() -> Result<()> {
    let input = input()?;
    let stats = build_file_stats(input);
    println!("Part 1: {}", part1(&stats));
    println!("Part 2: {}", part2(&stats)?);

    Ok(())
}
