use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

// 21-82,22-81
#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_numbers = separated_pair(parse_numbers, char('-'), parse_numbers);

        map(parse_two_numbers, |(start, end)| Range { start, end })(input)
    }

    fn is_fully_contained(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn contains_item(&self, other: u32) -> bool {
        self.start <= other && other <= self.end
    }
}

#[derive(Debug)]
struct Entry(Range, Range);

impl Entry {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_numbers = separated_pair(Range::parse, char(','), Range::parse);

        map(parse_two_numbers, |(r1, r2)| Entry { 0: r1, 1: r2 })(input)
    }

    fn is_fully_contained(&self) -> bool {
        self.0.is_fully_contained(&self.1) || self.1.is_fully_contained(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.is_fully_contained()
            || self.0.contains_item(self.1.start)
            || self.0.contains_item(self.1.end)
            || self.1.contains_item(self.0.start)
            || self.1.contains_item(self.0.end)
    }
}

fn main() {
    let input = include_str!("../../input/day04/input.txt");
    let (_remaining_input, entries) = separated_list1(line_ending, Entry::parse)(input).unwrap();

    let r1 = solve_part1(&entries);
    println!("r1: {}", r1);

    let r2 = solve_part2(&entries);
    println!("r2: {}", r2);
}

fn solve_part1(entries: &Vec<Entry>) -> u32 {
    entries.iter().filter(|e| e.is_fully_contained()).count() as u32
}

fn solve_part2(entries: &Vec<Entry>) -> u32 {
    entries.iter().filter(|e| e.overlaps()).count() as u32
}
