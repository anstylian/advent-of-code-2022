use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
    NumberNone,
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(left), Packet::Number(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
            (Packet::Number(left), Packet::List(right)) => {
                ([Packet::Number(*left)].as_slice()).cmp(&right)
            }
            (Packet::List(left), Packet::Number(right)) => {
                left.as_slice().cmp(&[Packet::Number(*right)])
            }
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut stack = vec![];
        let mut n = None;

        for (i, c) in line.chars().enumerate() {
            match c {
                '[' => {
                    stack.push(Packet::NumberNone);
                }
                ']' => {
                    let mut tmp = vec![];
                    while let Some(c) = stack.pop() {
                        if c == Packet::NumberNone {
                            let mut tt = vec![];
                            for i in tmp.into_iter().rev() {
                                tt.push(i);
                            }
                            let t = Packet::List(tt);
                            stack.push(t);
                            break;
                        }
                        tmp.push(c);
                    }
                }
                ',' => {
                    if let Some(_) = n {
                        stack.push(Packet::Number(0));
                        n = None;
                    }
                }
                c => {
                    if (i + 1) < line.len() && line.as_bytes()[i + 1] == '0' as u8 {
                        n = Some(10);
                    } else if let Some(nn) = n {
                        stack.push(Packet::Number(nn + c.to_digit(10).unwrap()));
                        n = None;
                    } else {
                        stack.push(Packet::Number(c.to_digit(10).unwrap()));
                    }
                }
            }
        }

        if stack.len() != 1 {
            unimplemented!();
        }

        Ok(stack.pop().unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    first: Packet,
    second: Packet,
}

impl TryFrom<&[&str]> for Pair {
    type Error = ();

    fn try_from(lines: &[&str]) -> Result<Self, Self::Error> {
        let first = Packet::try_from(lines[0])?;
        let second = Packet::try_from(lines[1])?;
        Ok(Pair { first, second })
    }
}

fn main() {
    let lines: Vec<&str> = include_str!("../../input/day13/input.txt")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut pairs = vec![];
    for l2 in lines.chunks(2) {
        let pair = Pair::try_from(l2).unwrap();
        pairs.push(pair);
    }

    let res: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            if pair.first < pair.second {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();

    println!("Result 1: {}", res);

    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    ];

    let mut packets = pairs
        .into_iter()
        .map(|p| [p.first, p.second])
        .flatten()
        .chain(divider_packets)
        .collect::<Vec<Packet>>();

    packets.sort();
    let p1 = packets
        .iter()
        .position(|x| *x == Packet::List(vec![Packet::List(vec![Packet::Number(2)])]));
    let p2 = packets
        .iter()
        .position(|x| *x == Packet::List(vec![Packet::List(vec![Packet::Number(6)])]));

    match (p1, p2) {
        (Some(p1), Some(p2)) => {
            println!("Result 2: {}", (p1 + 1) * (p2 + 1));
        }
        _ => unreachable!()
    }
}
