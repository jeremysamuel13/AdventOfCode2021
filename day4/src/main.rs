use std::collections::HashMap;

const FILEPATH: &str = "src/input.txt";
const ROWCOLLEN: u32 = 5;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Plane {
    Column,
    Row,
}

#[derive(Clone)]
struct Board {
    board: Vec<Vec<(u32, bool)>>,
    totals: HashMap<(Plane, usize), u32>,
    done: bool,
}

impl Board {
    fn from(s: Vec<String>) -> Self {
        Self {
            board: s
                .iter()
                .map(|y| {
                    y.split_ascii_whitespace()
                        .filter_map(|z| {
                            if let Ok(res) = z.parse::<u32>() {
                                return Some((res, false));
                            }
                            None
                        })
                        .collect::<Vec<(u32, bool)>>()
                })
                .collect::<Vec<Vec<(u32, bool)>>>(),
            totals: HashMap::new(),
            done: false,
        }
    }

    //returns true if board is done, false if not
    fn mark(&mut self, targ: u32) -> bool {
        'outer: for (ridx, r) in self.board.iter_mut().enumerate() {
            for (cidx, (val, checked)) in r.iter_mut().enumerate() {
                if targ == *val && !*checked {
                    *checked = true;
                    self.totals.insert(
                        (Plane::Row, ridx),
                        *self.totals.get(&(Plane::Row, ridx)).unwrap_or(&0) + 1,
                    );
                    self.totals.insert(
                        (Plane::Column, cidx),
                        *self.totals.get(&(Plane::Column, cidx)).unwrap_or(&0) + 1,
                    );

                    if *self.totals.get(&(Plane::Row, ridx)).unwrap() == ROWCOLLEN
                        || *self.totals.get(&(Plane::Column, cidx)).unwrap() == ROWCOLLEN
                    {
                        self.done = true;
                        return true;
                    }

                    break 'outer;
                }
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut res = 0;
        for r in &self.board {
            for (val, marked) in r {
                if !*marked {
                    res += val;
                }
            }
        }

        res
    }
}

fn main() {
    let data: Vec<String> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let order: Vec<u32> = data[0]
        .clone()
        .split(",")
        .filter_map(|x| {
            if let Ok(res) = x.parse::<u32>() {
                return Some(res);
            }

            return None;
        })
        .collect();

    let boards: Vec<Board> = data[2..]
        .to_vec()
        .split(|x| x.trim().is_empty())
        .map(|x| Board::from(x.to_vec()))
        .collect();

    println!("Answer to Q1: {}", q1(&order, boards.clone()));
    println!("Answer to Q2: {}", q2(&order, boards.clone()));
}

fn q1(order: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    for num in order {
        for b in boards.iter_mut() {
            if b.mark(*num) {
                //this is winner
                return b.unmarked_sum() * num;
            }
        }
    }

    0
}

fn q2(order: &Vec<u32>, boards: Vec<Board>) -> u32 {
    let mut boards = boards;
    let mut notdone = Vec::new();
    for num in order {
        let blen = boards.len();
        let mut counter = 0;
        for b in boards.iter_mut() {
            if b.mark(*num) {
                counter += 1;
                if counter == blen {
                    return b.unmarked_sum() * num;
                }
            } else {
                notdone.push(b.clone());
            }
        }

        boards = notdone.clone();
        notdone = Vec::new();
    }

    0
}
