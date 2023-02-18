fn main() {
    let input: Vec<Vec<u32>> = include_str!("../../input/day08/input.txt")
        .lines()
        .map(|l| l.chars().map(|h| h.to_digit(10).unwrap()).collect())
        .collect();

    let t: Trees = input.into();
    println!("part 1: {}", t.find_visible_from_outside());
    println!("part 2: {:?}", t.highest_scenic_score());
}

struct Trees(Vec<Tree>);

impl Trees {
    fn find_visible_from_outside(&self) -> usize {
        self.0
            .iter()
            .filter(|t| t.cell.iter().any(|&(_, v)| v))
            .count()
    }

    fn highest_scenic_score(&self) -> Option<usize> {
        self.0
            .iter()
            .map(|t| t.cell.iter().map(|(n, _)| n).product())
            .max()
    }
}

#[derive(Debug)]
struct Tree {
    cell: [(usize, bool); 4],
}

impl Tree {
    fn new(input: &Vec<Vec<u32>>, i: usize, j: usize) -> Self {
        let current_height = input[i][j];
        let rows = input.len() as i32;
        let cols = input[0].len() as i32;

        let mut cell = [(0, false); 4];

        for (idx, &xy) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().enumerate() {
            let mut ni = i;
            let mut nj = j;
            let mut count = 0;
            loop {
                if !is_valid(rows, cols, ni as i32, nj as i32, xy) {
                    cell[idx] = (count, true);
                    break;
                }

                ni = (ni as i32 + xy.0) as usize;
                nj = (nj as i32 + xy.1) as usize;

                if input[ni][nj] >= current_height {
                    cell[idx] = (count + 1, false);
                    break;
                }
                count += 1;
            }
        }

        Tree { cell }
    }
}

fn is_valid(rows: i32, cols: i32, i: i32, j: i32, xy: (i32, i32)) -> bool {
    let ii = i + xy.0;
    let ij = j + xy.1;

    ii < rows && ii >= 0 && ij < cols && ij >= 0
}

impl From<Vec<Vec<u32>>> for Trees {
    fn from(value: Vec<Vec<u32>>) -> Self {
        Trees(
            (0..value.len())
                .flat_map(|i| {
                    (0..value[i].len())
                        .map(|j| Tree::new(&value, i, j))
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    }
}
