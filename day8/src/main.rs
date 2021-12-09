use std::collections::HashSet;

const FILEPATH: &str = "src/input.txt";
type DataType = Problem;

#[derive(Clone, PartialEq, Eq)]
struct Problem {
    set: Vec<SevenSegment>,
    output: Vec<SevenSegment>,
    mappings: Vec<Option<SevenSegment>>,
}

impl Problem {
    fn from(prob: &str) -> Self {
        let prob: Vec<&str> = prob.split('|').collect();

        Self {
            set: prob[0]
                .split_ascii_whitespace()
                .map(SevenSegment::from)
                .collect(),
            output: prob[1]
                .split_ascii_whitespace()
                .map(SevenSegment::from)
                .collect(),
            mappings: vec![None; 10],
        }
    }

    fn find_mappings(&mut self) {
        //1 4 7 and 8 are easy to map
        self.set.retain(|x| match x.len() {
            2 => {
                self.mappings[1] = Some(x.clone());
                false
            }

            4 => {
                self.mappings[4] = Some(x.clone());
                false
            }

            3 => {
                self.mappings[7] = Some(x.clone());
                false
            }

            7 => {
                self.mappings[8] = Some(x.clone());
                false
            }

            _ => true,
        });

        //can figure out 9 from 4 and 7 and 3 from 1
        self.set.retain(|x| {
            if x.len() == 6
                && self.mappings[4]
                    .as_ref()
                    .unwrap()
                    .enabled
                    .difference(&x.enabled)
                    .count()
                    == 0
                && self.mappings[7]
                    .as_ref()
                    .unwrap()
                    .enabled
                    .difference(&x.enabled)
                    .count()
                    == 0
            {
                self.mappings[9] = Some(x.clone());
                return false;
            } else if x.len() == 5
                && self.mappings[1]
                    .as_ref()
                    .unwrap()
                    .enabled
                    .difference(&x.enabled)
                    .count()
                    == 0
            {
                self.mappings[3] = Some(x.clone());
                return false;
            }

            true
        });

        //can figure out two from the set diff between 9 and 2 (cannot be 0)
        self.set.retain(|x| {
            if x.len() == 5
                && x.enabled
                    .difference(&self.mappings[9].as_ref().unwrap().enabled)
                    .count()
                    != 0
            {
                self.mappings[2] = Some(x.clone());
                return false;
            }
            true
        });

        //only other 5 length pattern
        self.set.retain(|x| {
            if x.len() == 5 {
                self.mappings[5] = Some(x.clone());
                return false;
            }

            true
        });

        //can figure out six because it contains 5 and isnt 9
        self.set.retain(|x| {
            if x.len() == 6
                && *x != *self.mappings[9].as_ref().unwrap()
                && self.mappings[5]
                    .as_ref()
                    .unwrap()
                    .enabled
                    .difference(&x.enabled)
                    .count()
                    == 0
            {
                self.mappings[6] = Some(x.clone());
                return false;
            }

            true
        });

        self.mappings[0] = Some(self.set[0].clone());
    }

    fn get_output(&mut self) -> u32 {
        self.find_mappings();

        self.output
            .iter()
            .map(|x| x.get_mapping(&self.mappings))
            .collect::<Vec<String>>()
            .join("")
            .as_str()
            .parse()
            .unwrap()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct SevenSegment {
    enabled: HashSet<char>,
}

impl SevenSegment {
    fn from(str: &str) -> Self {
        Self {
            enabled: str.chars().collect(),
        }
    }

    fn len(&self) -> usize {
        self.enabled.len()
    }

    fn get_mapping(&self, mappings: &[Option<Self>]) -> String {
        mappings
            .iter()
            .enumerate()
            .find(|(_, mapping)| mapping.is_some() && *self == *mapping.as_ref().unwrap())
            .unwrap()
            .0
            .to_string()
    }
}

fn main() {
    let data: Vec<DataType> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split('\n')
        .map(Problem::from)
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &[DataType]) -> usize {
    //size of 2 3 4 and 7
    data.iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum()
}

fn q2(data: &[DataType]) -> u32 {
    data.iter().cloned().map(|mut x| x.get_output()).sum()
}
