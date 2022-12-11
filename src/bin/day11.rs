#[derive(Default, Debug, Clone)]
enum Op {
    Mul(usize),
    Add(usize),
    MulSelf,
    AddSelf,
    #[default]
    NA,
}

impl Op {
    fn run(&self, nn: usize) -> usize {
        match self {
            Op::Mul(n) => n * nn,
            Op::MulSelf => nn * nn,
            Op::Add(n) => *n + nn,
            Op::AddSelf => nn + nn,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct DivisibleBy {
    num: usize,
    on_true: usize,
    on_false: usize,
}

impl DivisibleBy {
    fn next_monkey(&self, n: usize) -> usize {
        if n % self.num == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Op,
    divisible_by: DivisibleBy,
    inspections: usize,
}

impl Monkey {
    // return monkey, item
    fn round_part(&mut self, stress_relief: usize, worry_factor: usize) -> Vec<(usize, usize)> {
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|v| {
                let worry_level = self.operation.run(v) / stress_relief % worry_factor;
                let send_to = self.divisible_by.next_monkey(worry_level);
                (send_to, worry_level)
            })
            .collect()
    }
}

impl TryFrom<&str> for Monkey {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut iter = line.split("|");

        iter.next();

        let items = iter
            .next()
            .and_then(|l| l.strip_prefix("Starting items: "))
            .map(|l| l.split(", ").map(|v| v.parse::<usize>().unwrap()).collect())
            .unwrap();

        let op: Op = iter
            .next()
            .and_then(|l| l.strip_prefix("Operation: new ="))
            .and_then(|op| {
                Some(match op.split_whitespace().collect::<Vec<_>>()[..] {
                    ["old", "*", "old"] => Op::MulSelf,
                    ["old", "+", "old"] => Op::AddSelf,
                    ["old", "*", n] => Op::Mul(n.parse::<usize>().unwrap()),
                    ["old", "+", n] => Op::Add(n.parse::<usize>().unwrap()),
                    _ => unreachable!(),
                })
            })
            .unwrap();

        let num = iter
            .next()
            .and_then(|l| l.strip_prefix("Test: divisible by "))
            .and_then(|n| n.parse::<usize>().ok())
            .unwrap();

        let t = iter
            .next()
            .and_then(|l| l.strip_prefix("If true: throw to monkey "))
            .and_then(|n| n.parse::<usize>().ok())
            .unwrap();

        let f = iter
            .next()
            .and_then(|l| l.strip_prefix("If false: throw to monkey "))
            .and_then(|n| n.parse::<usize>().ok())
            .unwrap();

        Ok(Monkey {
            items,
            operation: op,
            divisible_by: DivisibleBy {
                num,
                on_true: t,
                on_false: f,
            },
            inspections: 0,
        })
    }
}

fn main() {
    let input: Vec<_> = include_str!("../../input/day11/input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect::<Vec<_>>();

    let input: Vec<_> = input.chunks(6).map(|c| c.join("|")).collect();

    let input: Vec<Monkey> = input
        .iter()
        .filter_map(|l| Monkey::try_from(l.as_str()).ok())
        .collect();

    let r1 = solver(input.clone(), 20, 3);
    println!("{:?}", r1);

    let r2 = solver(input, 10_000, 1);
    println!("{:?}", r2);
}

fn solver(mut input: Vec<Monkey>, iter: u32, stress_relief: usize) -> u32 {
    let worry_factor = input.iter().map(|m| m.divisible_by.num).product();
    for _ in 0..iter {
        for i in 0..input.len() {
            let moves = input[i].round_part(stress_relief, worry_factor);
            for m in moves {
                input[m.0 as usize].items.push(m.1);
            }
        }
    }

    let mut inspections = input
        .iter()
        .map(|m| m.inspections as u32)
        .collect::<Vec<_>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}
