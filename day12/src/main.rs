use std::collections::{HashMap, HashSet};
const FILEPATH: &str = "src/input.txt";
const START: &str = "start";
const END: &str = "end";

struct AdjacencyList {
    l: HashMap<String, Vec<String>>,
}

impl AdjacencyList {
    fn new() -> Self {
        Self { l: HashMap::new() }
    }

    fn add(&mut self, a: &str, b: &str) {
        self.l
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push(b.to_string());

        self.l
            .entry(b.to_string())
            .or_insert_with(Vec::new)
            .push(a.to_string());
    }

    fn search(&self, curr: String, mut visited: HashSet<String>) -> usize {
        if curr == *END {
            return 1;
        }

        visited.insert(curr.clone());

        self.l[&curr]
            .iter()
            .filter(|curr| !(curr.chars().all(char::is_lowercase) && visited.contains(*curr)))
            .fold(0, |acc, x| {
                acc + self.search(x.to_string(), visited.clone())
            })
    }

    fn search_twice(&self, curr: String, mut visited: HashSet<String>, mut twice: bool) -> usize {
        if curr == *END {
            return 1;
        }

        if visited.contains(&curr) && curr.chars().all(char::is_lowercase) && !twice {
            twice = true;
        }

        visited.insert(curr.clone());

        self.l[&curr]
            .iter()
            .filter(|curr| {
                !(visited.contains(*curr)
                    && (**curr == *START || (curr.chars().all(char::is_lowercase) && twice)))
            })
            .fold(0, |acc, x| {
                acc + self.search_twice(x.to_string(), visited.clone(), twice)
            })
    }
}

fn main() {
    let mut al = AdjacencyList::new();
    std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .lines()
        .map(|x| x.split_once('-').unwrap())
        .for_each(|(a, b)| al.add(a, b));

    println!("Q1 answer is: {}", q1(&al));
    println!("Q2 answer is: {}", q2(&al));
}

fn q1(al: &AdjacencyList) -> usize {
    al.search(START.to_string(), HashSet::new())
}

fn q2(al: &AdjacencyList) -> usize {
    al.search_twice(START.to_string(), HashSet::new(), false)
}
