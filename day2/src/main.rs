use std::fs::read_to_string;

const FILEPATH: &str = "src/input.txt";

fn main() {
    let data = read_to_string(FILEPATH).expect("Unable to read file");
    let data = data
        .split("\n")
        .filter_map(|s| {
            let r: Vec<&str> = s.split_ascii_whitespace().collect();
            if let Some(v) = r.get(1) {
                if let Ok(val) = v.parse::<i32>() {
                    return Some((r[0], val));
                }
            }

            return None;
        })
        .collect();

    println!("Answer to Q1 is: {}", q1(&data));
    println!("Answer to Q2 is: {}", q2(&data));
}

fn q1(data: &Vec<(&str, i32)>) -> i32 {
    let mut hor = 0;
    let mut vert = 0;
    for (direction, value) in data {
        match direction {
            &"up" => vert -= value,
            &"down" => vert += value,
            &"forward" => hor += value,
            _ => continue,
        }
    }

    return hor * vert;
}

fn q2(data: &Vec<(&str, i32)>) -> i32 {
    let mut hor = 0;
    let mut vert = 0;
    let mut aim = 0;
    for (direction, value) in data {
        match direction {
            &"up" => aim -= value,
            &"down" => aim += value,
            &"forward" => {
                hor += value;
                vert += (aim * value);
            }
            _ => continue,
        }
    }

    return hor * vert;
}
