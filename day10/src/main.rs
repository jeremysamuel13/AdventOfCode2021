const FILEPATH: &str = "src/input.txt";
type DataType = Line;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Bracket {
        match c {
            '{' | '[' | '(' | '<' => Bracket::Open(c),
            '}' => Bracket::Close('{'),
            ']' => Bracket::Close('['),
            ')' => Bracket::Close('('),
            '>' => Bracket::Close('<'),
            _ => panic!("Unexpected character"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum State {
    Incomplete(u128),
    Corrupted(i32),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Line {
    chars: Vec<Bracket>,
    state: State,
}

impl Line {
    fn from(line: &str) -> Self {
        let chars = line.chars().map(|x| Bracket::from_char(x)).collect();
        let state = Line::get_state(&chars);
        Self { chars, state }
    }

    fn get_state(chars: &Vec<Bracket>) -> State {
        let mut bal = vec![];

        for c in chars {
            match c {
                Bracket::Open(char_bracket) => {
                    bal.push(char_bracket);
                }
                Bracket::Close(char_close_bracket) => {
                    if bal.pop() != Some(char_close_bracket) {
                        match char_close_bracket {
                            '(' => return State::Corrupted(3),
                            '[' => return State::Corrupted(57),
                            '{' => return State::Corrupted(1197),
                            '<' => return State::Corrupted(25137),
                            _ => panic!("Unexpected character"),
                        };
                    }
                }
            }
        }

        if !bal.is_empty() {
            return State::Incomplete(bal.iter().rev().fold(0, |res, b| {
                5 * res
                    + match b {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Unexpected character"),
                    }
            }));
        };

        //then valid
        State::Incomplete(0)
    }
}

fn main() {
    let data: Vec<DataType> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split('\n')
        .map(DataType::from)
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &[DataType]) -> i32 {
    data.iter()
        .map(|x| {
            if let State::Corrupted(val) = x.state {
                return val;
            }

            0
        })
        .sum()
}

fn q2(data: &[DataType]) -> u128 {
    let mut data: Vec<u128> = data
        .iter()
        .filter_map(|x| {
            if let State::Incomplete(val) = x.state {
                return Some(val);
            }

            None
        })
        .collect();

    data.sort();

    //println!("{:?}", data);

    data[(data.len()) / 2]
}
