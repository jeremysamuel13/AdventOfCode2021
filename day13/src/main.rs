use std::collections::HashSet;
const FILEPATH: &str = "src/input.txt";

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn from((axis, val): (&str, &str)) -> Option<Self> {
        match axis {
            "x" => Some(Fold::X(val.parse().unwrap())),
            "y" => Some(Fold::Y(val.parse().unwrap())),
            _ => None,
        }
    }
}

fn main() {
    let data: Vec<String> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();

    let s = data.iter().position(|x| x.trim().is_empty()).unwrap();

    let coords = data[0..s]
        .iter()
        .flat_map(|line| {
            line.split_once(",")
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        })
        .collect();

    let fold: Vec<Fold> = data[s + 1..]
        .iter()
        .flat_map(|line| {
            Fold::from(
                line.split_ascii_whitespace().collect::<Vec<&str>>()[2]
                    .split_once("=")
                    .unwrap(),
            )
        })
        .collect();

    println!("Q1 answer is: {}", q1(&coords, &fold));
    println!("Q2 answer is: {}", q2(&coords, &fold));
}

fn q1(set: &HashSet<(usize, usize)>, fold: &[Fold]) -> usize {
    match fold[0] {
        Fold::X(val) => set
            .iter()
            .map(|(x, y)| (if x > &val { 2 * val - x } else { *x }, *y))
            .collect::<HashSet<(usize, usize)>>(),
        Fold::Y(val) => set
            .iter()
            .map(|(x, y)| (*x, if y > &val { 2 * val - y } else { *y }))
            .collect::<HashSet<(usize, usize)>>(),
    }
    .len()
}

fn q2(set: &HashSet<(usize, usize)>, fold: &[Fold]) -> String {
    let set = fold.iter().fold(set.clone(), |set, f| match *f {
        Fold::X(val) => set
            .into_iter()
            .map(|(x, y)| (if x > val { 2 * val - x } else { x }, y))
            .collect(),
        Fold::Y(val) => set
            .into_iter()
            .map(|(x, y)| (x, if y > val { 2 * val - y } else { y }))
            .collect(),
    });

    (0..=set.iter().max_by_key(|(_, y)| y).unwrap().1).fold("\n".to_string(), |acc, j| {
        acc + &(0..=set.iter().max_by_key(|(x, _)| x).unwrap().0).fold("".to_string(), |iacc, i| {
            iacc + if set.contains(&(i, j)) { "#" } else { "~" }
        }) + "\n"
    })
}
