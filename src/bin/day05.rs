#[derive(Debug)]
struct Action {
    count: usize,
    from: usize,
    to: usize,
}

impl Action {
    // move 4 from 9 to 1
    fn new(line: &str) -> Self {
        let t: String = line
            .chars()
            .filter(|c| c.is_digit(10) || c.is_whitespace())
            .collect();

        let mut iter = t.split_whitespace();

        Action {
            count: iter.next().unwrap().parse::<usize>().unwrap(),
            from: iter.next().unwrap().parse::<usize>().unwrap(),
            to: iter.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    s: Vec<Vec<char>>,
}

impl Stacks {
    fn push(&mut self, idx: usize, c: Option<char>) {
        if let Some(c) = c {
            if c.is_whitespace() {
                return;
            }
            if self.s.len() <= idx {
                for _ in self.s.len()..(idx + 1) {
                    self.s.push(Vec::new());
                }
            }
            if self.s[idx].is_empty() {
                self.s[idx].push(c);
            } else {
                self.s[idx].insert(0, c);
            }
        }
    }

    fn mmove(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            if let Some(item) = self.s[from-1].pop() {
                self.s[to-1].push(item);
            }
        }
    }

    fn mmove_chuncks(&mut self, count: usize, from: usize, to: usize) {
            let start_range = self.s[from-1].len()-count;

            let mut tmp = self.s[from-1].drain(start_range..).collect();
            self.s[to-1].append(&mut tmp);
    }

    fn tops(&self) -> Vec<char> {
        self.s.iter().map(|s| s[s.len()-1]).collect()
    }
}

fn parse_line(line: &str, stack: &mut Stacks, actions: &mut Vec<Action>) {
    if line.is_empty() {
        return;
    }
    match &line[..1] {
        "[" => {
            for i in (1..line.len()).step_by(4) {
                stack.push((i + 1) / 4, line.chars().nth(i));
            }
        }
        "m" => {
            actions.push(Action::new(&line[1..]));
        }
        _ => {}
    }
}

fn main() {
    let mut stacks = Stacks { s: vec![] };
    let mut actions = vec![];
    include_str!("../../input/day05/input.txt")
        .lines()
        .for_each(|l| parse_line(l, &mut stacks, &mut actions));

    let r1 = solve_part1(stacks.clone(), &actions);
    println!("r1: {}", r1);

    let r2 = solve_part2(stacks, &actions);
    println!("r2: {}", r2);
}

fn solve_part1(mut stacks: Stacks, actions: &Vec<Action>) -> String {

    for action in actions {
        stacks.mmove(action.count, action.from, action.to);
    }

    stacks.tops().into_iter().collect()
}

fn solve_part2(mut stacks: Stacks, actions: &Vec<Action>) -> String {
    for action in actions {
        stacks.mmove_chuncks(action.count, action.from, action.to);
    }

    stacks.tops().into_iter().collect()
}

