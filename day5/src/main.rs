use std::{cmp::min_by, collections::HashMap, hash::Hash};

const FILEPATH: &str = "src/input.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Dir {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Dir {
    fn from(from: (u32, u32), to: (u32, u32)) -> Self {
        if from.0 == to.0 {
            return Dir::Horizontal;
        } else if from.1 == to.1 {
            return Dir::Vertical;
        }

        Dir::Diagonal
    }
}

#[derive(Clone, Copy, Debug)]
struct Vent {
    from: (u32, u32),
    to: (u32, u32),
    dir: Dir,
}

impl Vent {
    fn from(from: (u32, u32), to: (u32, u32)) -> Self {
        Self {
            from,
            to,
            dir: Dir::from(from, to),
        }
    }

    fn max(&self) -> u32 {
        use std::cmp::max;

        return max(self.from.0, max(self.from.1, max(self.to.0, self.to.1)));
    }

    fn get_points(&self) -> Vec<(usize, usize)> {
        match self.dir {
            Dir::Horizontal => {
                let mi = std::cmp::min(self.from.1, self.to.1);
                let ma = std::cmp::max(self.from.1, self.to.1);

                return (mi..=ma)
                    .map(|x| (self.from.0 as usize, x as usize))
                    .collect();
            }

            Dir::Vertical => {
                let mi = std::cmp::min(self.from.0, self.to.0);
                let ma = std::cmp::max(self.from.0, self.to.0);
                return (mi..=ma)
                    .map(|x| (x as usize, self.from.1 as usize))
                    .collect();
            }
            Dir::Diagonal => {
                let mut res = Vec::new();

                // min/max by height
                let min = min_by(self.from, self.to, |x, y| x.1.cmp(&y.1));
                let (maxx, maxy) = min_by(self.from, self.to, |x, y| x.1.cmp(&y.1).reverse());

                let horoffset = (min.0 < maxx).then(|| 1).unwrap_or(-1);

                let mut x: i32 = min.0 as i32;
                let mut y: i32 = min.1 as i32;

                while (x, y) != (maxx as i32, maxy as i32) {
                    res.push((x as usize, y as usize));

                    x += horoffset;
                    y += 1;
                }

                res.push((maxx as usize, maxy as usize));

                return res;
            }
        };
    }
}

fn main() {
    let data: Vec<Vent> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|x| {
            let b = x
                .split(" -> ")
                .map(|y| {
                    let a = y
                        .split(",")
                        .map(|z| z.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    (a[0], a[1])
                })
                .collect::<Vec<(u32, u32)>>();

            Vent::from(b[0], b[1])
        })
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Vec<Vent>) -> usize {
    let vents: Vec<Vent> = data
        .iter()
        .copied()
        .filter(|x| x.dir != Dir::Diagonal)
        .collect();

    q2(&vents)
}

fn q2(data: &Vec<Vent>) -> usize {
    let mut grid = HashMap::new();
    for ele in data {
        for (x, y) in ele.get_points() {
            *grid.entry((x, y)).or_insert(0) += 1;
        }
    }

    grid.values().filter(|x| **x >= 2).count()
}
