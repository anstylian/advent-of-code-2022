#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let line: Vec<_> = include_str!("../../input/day03/input.txt")
        .lines()
        .collect();

    let r1 = solve_part1(&line);
    println!("{:?}", r1);

    let r2 = solve_part2(&line);
    println!("{:?}", r2);
}

fn solve_part1(lines: &Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|l| -> u32 {
            let half_len = l.len() / 2;
            let (f, s) = l.split_at(half_len);
            let hs_first: HashSet<char> = HashSet::from_iter(f.chars().into_iter());
            let hs_second: HashSet<char> = HashSet::from_iter(s.chars().into_iter());
            hs_first
                .intersection(&hs_second)
                .map(|&c| char_to_value(c))
                .sum()
        })
        .sum()
}

fn char_to_value(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        0 // THIS is an error
    }
}

fn solve_part2(lines: &Vec<&str>) -> u32 {
    lines
        .iter()
        .array_chunks()
        .filter_map(|[a, b, c]| {
            let hs_first: HashSet<char> = HashSet::from_iter(a.chars().into_iter());
            let hs_second: HashSet<char> = HashSet::from_iter(b.chars().into_iter());
            let hs_third: HashSet<char> = HashSet::from_iter(c.chars().into_iter());
            HashSet::from_iter(
                hs_first
                    .intersection(&hs_second)
                    .map(|&c| c)
                    .collect::<Vec<char>>(),
            )
            .intersection(&hs_third)
            .map(|&c| c)
            .collect::<Vec<char>>()
            .first()
            .map(|&c| char_to_value(c))
        })
        .sum()
}
