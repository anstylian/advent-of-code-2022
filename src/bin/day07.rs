#[derive(Debug)]
enum Cmd {
    Directory(u32),
    File(u32),
    Back,
    Skip,
    Ls,
}

impl TryFrom<&str> for Cmd {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        Ok(match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "cd", ".."] => Cmd::Back,
            ["$", "cd", _name] => Cmd::Directory(0),
            ["$", "ls"] => Cmd::Ls,
            ["dir", _name] => Cmd::Skip,
            [raw_size, _name] => {
                Cmd::File(raw_size.parse::<u32>().unwrap())
            }
            _ => unreachable!()
        })
    }
}

fn main() {
    let input: Vec<_> = include_str!("../../input/day07/input.txt")
        .lines()
        .into_iter()
        .filter_map(|l| { Cmd::try_from(l).ok()})
        .collect();

    let mut it: std::slice::Iter<'_, Cmd> = input.iter();
    let (_, files) = dir_sizes(&mut it);

    let r1 = solve_part1(&files);
    println!("r1: {}", r1);

    let r2 = solve_part2(&files);
    println!("r2: {}", r2);
}

fn solve_part1(files: &Vec<Cmd>) -> u32 {
    files.iter().filter_map(|d| match d {
        Cmd::Directory(s) if *s <= 100000 => Some(s),
        _ => None,
    }).sum()
}

fn solve_part2(files: &Vec<Cmd>) -> u32 {
    const TOTAL_SPACE: u32 = 70_000_000;
    const SPACE_NEEDED: u32 = 30_000_000;

    let iter = files.iter().filter_map(|d| match d {
        Cmd::Directory(s) => Some(*s),
        _ => None});
    let max = iter.clone().max().unwrap_or(0);

    let curr_free = TOTAL_SPACE.saturating_sub(max);
    let need_to_free = SPACE_NEEDED.saturating_sub(curr_free);

    files.iter().filter_map(|d| match d {
        Cmd::Directory(s) if *s >= need_to_free => Some(*s),
        _ => None,
    }).min().unwrap_or(0)
}

fn dir_sizes(it: &mut std::slice::Iter<'_, Cmd>) -> (u32, Vec<Cmd>) {
    let mut current_size = 0;
    let mut res = vec![];
    while let Some(entry) = it.next() {
        match entry {
            Cmd::File(size) => {
                res.push(Cmd::File(*size));
                current_size += size;
            },
            Cmd::Directory(_) => {
                let (size, sub_entries) = dir_sizes(it);
                current_size += size;
                res.push(Cmd::Directory(size));
                res.extend(sub_entries);
            }
            Cmd::Back => {
                return (current_size, res)
            }
            _ => {}
        }
    }

    (current_size, res)
}
