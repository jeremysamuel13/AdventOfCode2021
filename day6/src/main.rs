use std::collections::HashMap;
const FILEPATH: &str = "src/input.txt";

fn main() {
    let data: Vec<u32> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Q1 answer is: {}", q1(&data, 80));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Vec<u32>, days: u32) -> usize {
    let mut count = [0; 9];

    for v in data {
        count[*v as usize] += 1;
    }

    for _ in 0..days {
        let new = count[0];
        for i in 0..8 {
            count[i] = count[i + 1]
        }
        count[6] += new;
        count[8] = new;
    }

    count.iter().sum()
}

fn q2(data: &Vec<u32>) -> usize {
    q1(data, 256)
}
