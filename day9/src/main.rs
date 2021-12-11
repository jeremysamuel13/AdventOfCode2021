use std::collections::BinaryHeap;

const FILEPATH: &str = "src/input.txt";
const EMPTYCELL: Cell = Cell {
    val: 10,
    marked: false,
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cell {
    val: i32,
    marked: bool,
}

impl Cell {
    fn from(c: char) -> Self {
        Self {
            val: c.to_string().parse::<i32>().unwrap(),
            marked: false,
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Problem {
    inp: Vec<Vec<Cell>>,
    rl: usize,
    cl: usize,
}

impl Problem {
    fn from(input: String) -> Self {
        //println!("{:?}",);
        let inp: Vec<Vec<Cell>> = input
            .to_string()
            .trim()
            .split('\n')
            .map(|x| x.chars().map(Cell::from).collect())
            .collect();

        let rl = inp.len();
        let cl = inp[0].len();

        Self { inp, rl, cl }
    }

    fn lowest_point_sum(&self) -> i32 {
        let mut res = 0;

        for i in 0..self.rl {
            for j in 0..self.cl {
                let val = &self.inp[i][j];
                let temp = vec![];

                let adj = vec![
                    self.inp
                        .get(i)
                        .unwrap_or(&temp)
                        .get(j - 1)
                        .unwrap_or(&EMPTYCELL),
                    self.inp
                        .get(i)
                        .unwrap_or(&temp)
                        .get(j + 1)
                        .unwrap_or(&EMPTYCELL),
                    self.inp
                        .get(i - 1)
                        .unwrap_or(&temp)
                        .get(j)
                        .unwrap_or(&EMPTYCELL),
                    self.inp
                        .get(i + 1)
                        .unwrap_or(&temp)
                        .get(j)
                        .unwrap_or(&EMPTYCELL),
                ];

                if val < adj[0] && val < adj[1] && val < adj[2] && val < adj[3] {
                    res += val.val + 1;
                }
            }
        }

        res
    }

    fn get_basins_len(&mut self, i: usize, j: usize) -> usize {
        let val = &mut self.inp[i][j];

        if val.marked || val.val == 9 {
            return 0;
        }

        val.marked = true;

        let mut res = 1;

        if i > 0 {
            res += self.get_basins_len(i - 1, j);
        }

        if i + 1 < self.rl {
            res += self.get_basins_len(i + 1, j);
        }

        if j > 0 {
            res += self.get_basins_len(i, j - 1);
        }

        if j + 1 < self.cl {
            res += self.get_basins_len(i, j + 1);
        }

        res
    }

    fn get_basins(&mut self) -> BinaryHeap<usize> {
        let mut basins = BinaryHeap::new();

        for i in 0..self.inp.len() {
            for j in 0..self.inp[0].len() {
                if !self.inp[i][j].marked {
                    basins.push(self.get_basins_len(i, j));
                }
            }
        }

        basins
    }
}

fn main() {
    let data: Problem =
        Problem::from(std::fs::read_to_string(FILEPATH).expect("Unable to read file"));

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Problem) -> i32 {
    data.lowest_point_sum()
}

fn q2(data: &Problem) -> usize {
    let mut data = data.clone();
    let mut basins = data.get_basins();

    basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
}
