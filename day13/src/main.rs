#![feature(iter_array_chunks)]

use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u128),
    List(Vec<Packet>),
}

fn cmp_lists(l: &[Packet], r: &[Packet]) -> std::cmp::Ordering {
    l.iter()
        .zip(r)
        .map(|(l, r)| l.cmp(r))
        .find(|o| *o != std::cmp::Ordering::Equal)
        .unwrap_or_else(|| l.len().cmp(&r.len()))
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::*;
        match (self, other) {
            (Number(a), Number(b)) => a.partial_cmp(b),
            (List(a), List(b)) => Some(cmp_lists(a, b)),
            (a @ Number(_), List(b)) => Some(cmp_lists(&[a.clone()], b)),
            (List(a), b @ Number(_)) => Some(cmp_lists(a, &[b.clone()])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_num(input: &str) -> IResult<&str, Packet> {
    map_res(digit1, |s: &str| s.parse::<u128>().map(Packet::Number))(input)
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    let item = alt((parse_num, parse_list));
    let items = separated_list0(tag(","), item).map(Packet::List);
    delimited(tag("["), items, tag("]"))(input)
}

fn packet_list(input: &str) -> IResult<&str, Vec<[Packet; 2]>> {
    let pair = tuple((parse_list, tag("\n"), parse_list)).map(|(a, _, b)| [a, b]);
    separated_list1(tag("\n\n"), pair)(input)
}

fn input() -> Vec<[Packet; 2]> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (input, output) = packet_list(input.trim())
        .map_err(|err| err.to_owned())
        .unwrap();

    if !input.is_empty() {
        panic!("Input illformed, tail not parsed: {input}");
    }

    output
}

fn part1(input: &[[Packet; 2]]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, [a, b])| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: Vec<[Packet; 2]>) -> usize {
    let delim1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let delim2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

    let mut data: Vec<_> = input
        .into_iter()
        .flatten()
        .chain([delim1.clone(), delim2.clone()])
        .collect();

    data.sort_unstable();

    let delim1pos = data.binary_search(&delim1).unwrap() + 1;
    let delim2pos = data.binary_search(&delim2).unwrap() + 1;

    delim1pos * delim2pos
}

fn main() {
    let input = input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}
