const FILEPATH: &str = "src/input.txt";
type DataType = i32;

fn main() {
    let data: Vec<DataType> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));

    assert_eq!(q1(&data), 336120);
    assert_eq!(q2(&data), 96864235);
}

fn q1(data: &[DataType]) -> i32 {
    (*data.iter().min().unwrap()..=*data.iter().max().unwrap())
        .map(|i| data.iter().map(|x| (x - i).abs()).sum::<i32>())
        .min()
        .unwrap()
}

fn q2(data: &[DataType]) -> i32 {
    (*data.iter().min().unwrap()..=*data.iter().max().unwrap())
        .map(|i| {
            data.iter()
                .map(|x| (x - i).abs() * ((x - i).abs() + 1) / 2)
                .sum::<i32>()
        })
        .min()
        .unwrap()
}
