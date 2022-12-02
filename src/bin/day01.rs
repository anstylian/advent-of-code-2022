fn main() {
    let numbers: Vec<_> = include_str!("../../input/day01/input.txt")
        .lines()
        .map(|n| {
            if n.is_empty() {
                -1
            } else {
                n.parse::<i32>().unwrap()
            }
        })
        .collect();

    let sol1 = solve_part1(&numbers);
    println!("Part one answer: {}", sol1);

    let sol2 = solve_part2(&numbers);
    println!("Part two answer: {}", sol2);
}

fn solve_part1(numbers: &[i32]) -> i32 {
    let mut cal = vec![];
    let mut sum = 0;
    for &c in numbers {
        if c == -1 {
            cal.push(sum);
            sum = 0;
        } else {
            sum += c;
        }
    }

    *cal.iter().max().unwrap()
}

fn solve_part2(numbers: &[i32]) -> i32 {
    let mut cal = vec![];
    let mut sum = 0;
    for &c in numbers {
        if c == -1 {
            cal.push(sum);
            sum = 0;
        } else {
            sum += c;
        }
    }

    cal.sort_by(|a, b| b.partial_cmp(a).unwrap());
    cal[0..3].iter().sum()
}
