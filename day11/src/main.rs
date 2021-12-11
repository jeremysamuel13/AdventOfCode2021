const FILEPATH: &str = "src/input.txt";

#[derive(Clone, PartialEq, Eq, Debug)]
struct Problem {
    inp: Vec<Vec<(u8, i128)>>,
    rl: usize,
    cl: usize,
}

impl Problem {
    fn from(input: String) -> Self {
        //println!("{:?}",);
        let inp: Vec<Vec<(u8, i128)>> = input
            .trim()
            .split('\n')
            .map(|x| {
                x.chars()
                    .map(|x| (x.to_digit(10).unwrap() as u8, -1))
                    .collect()
            })
            .collect();

        let rl = inp.len();
        let cl = inp[0].len();

        Self { inp, rl, cl }
    }

    fn flash_step(&mut self, res: &mut u128, q: u128) {
        for i in 0..self.rl {
            for j in 0..self.cl {
                self.inp[i][j].0 += 1;

                if self.inp[i][j].0 > 9 && self.inp[i][j].1 != (q as i128) {
                    self.flash(i, j, res, q);
                }
            }
        }

        for a in self.inp.iter_mut() {
            for val in a.iter_mut() {
                if val.0 > 9 {
                    val.0 = 0;
                }
            }
        }
    }

    fn flashes_nsteps(&mut self, n: u128) -> u128 {
        let mut res = 0;

        for q in 0..n {
            self.flash_step(&mut res, q)
        }

        res
    }

    fn steps_to_synchronize(&mut self) -> u128 {
        let mut steps = 0;
        let mut q = 0;

        while self
            .inp
            .iter()
            .map(|x| x.iter().filter(|(_, curr)| *curr != q as i128).count())
            .sum::<usize>()
            != 0
        {
            steps += 1;
            q += 1;
            self.flash_step(&mut 0, q);
        }

        steps
    }

    fn flash(&mut self, i: usize, j: usize, res: &mut u128, currstep: u128) {
        //println!("{},{}", i, j);
        let v = &mut self.inp[i][j];
        if v.1 as u128 == currstep {
            return;
        }

        *res += 1;
        v.1 = currstep as i128;

        let ind = vec![
            (i - 1, j),
            (i - 1, j - 1),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];

        let mut next = vec![];

        for (a, b) in ind {
            if let Some(x) = self.inp.get_mut(a) {
                if let Some(val) = x.get_mut(b) {
                    val.0 += 1;

                    if val.0 > 9 {
                        next.push((a, b));
                    }
                }
            }
        }

        for (a, b) in next {
            self.flash(a, b, res, currstep);
        }
    }
}

fn main() {
    let data: Problem =
        Problem::from(std::fs::read_to_string(FILEPATH).expect("Unable to read file"));

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Problem) -> u128 {
    let mut data = data.clone();
    data.flashes_nsteps(100)
}

fn q2(data: &Problem) -> u128 {
    let mut data = data.clone();
    data.steps_to_synchronize()
}
