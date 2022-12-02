fn main() {
    let line: Vec<_> = include_str!("../../input/day02/input.txt")
        .lines()
        .flat_map(|l| match l.split_once(' ') {
            Some((x, y)) => Some((x, y)),
            _ => None,
        })
        .collect();

    let sol1 = solve_part1(&line);
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&line);
    println!("Part two answer: {}", sol2);
}

fn points(v: &str) -> i32 {
    match v {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}
fn result(b: &str, a: &str) -> i32 {
    match (b, a) {
        ("X", "C") | ("Y", "A") | ("Z", "B") => 6,
        ("X", "A") | ("Y", "B") | ("Z", "C") => 3,
        _ => 0,
    }
}

fn solve_part1(line: &Vec<(&str, &str)>) -> i32 {
    line.iter().map(|(a, b)| points(b) + result(b, a)).sum()
}

fn mmove(outcome: &str, opponent: &str) -> i32 {
    match (opponent, outcome) {
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,
        ("A", "Y") => 1 + 3,
        ("B", "Y") => 2 + 3,
        ("C", "Y") => 3 + 3,
        ("A", "Z") => 2 + 6,
        ("B", "Z") => 3 + 6,
        ("C", "Z") => 1 + 6,
        _ => 0,
    }
}

fn solve_part2(line: &Vec<(&str, &str)>) -> i32 {
    line.iter().map(|(a, b)| mmove(b, a)).sum()
}
