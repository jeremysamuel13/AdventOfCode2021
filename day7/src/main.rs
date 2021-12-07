use std::iter::repeat;

const FILEPATH: &str = "src/input.txt";
type DataType = i32;

fn main() {
    let data: Vec<DataType> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Vec<DataType>) -> i32 {
    vec![0; *data.iter().max().unwrap() as usize + 1]
        .iter()
        .enumerate()
        .map(|(i, val)| data.iter().map(|x| (x - i as i32).abs()).sum::<i32>())
        .min()
        .unwrap()
}

fn q2(data: &Vec<DataType>) -> i32 {
    vec![0; *data.iter().max().unwrap() as usize + 1]
        .iter()
        .enumerate()
        .map(|(i, _)| {
            data.iter()
                .map(|x| (x - i as i32).abs() * ((x - i as i32).abs() + 1) / 2)
                .sum::<i32>()
        })
        .min()
        .unwrap()
}
