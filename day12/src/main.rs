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

    fn add(&mut self, a: String, b: String) {
        self.l
            .entry(a.clone())
            .or_insert_with(Vec::new)
            .push(b.clone());

        self.l.entry(b).or_insert_with(Vec::new).push(a);
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
    let data = std::fs::read_to_string(FILEPATH).expect("Unable to read file");
    let data: Vec<&str> = data.trim().split('\n').collect();
    let mut al = AdjacencyList::new();
    for a in data.iter().map(|x| x.split('-').collect::<Vec<&str>>()) {
        al.add(a[0].to_string(), a[1].to_string());
    }

    println!("Q1 answer is: {}", q1(&al));
    println!("Q2 answer is: {}", q2(&al));
}

fn q1(al: &AdjacencyList) -> usize {
    al.search(START.to_string(), HashSet::new())
}

fn q2(al: &AdjacencyList) -> usize {
    al.search_twice(START.to_string(), HashSet::new(), false)
}
