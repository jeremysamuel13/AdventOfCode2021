fn main() {
    let data: Vec<u32> = std::fs::read_to_string("src/input.txt")
        .expect("Unable to read file")
        .split("\n")
        .filter_map(|s| {
            if let Ok(val) = s.parse::<u32>() {
                return Some(val);
            } else {
                return None;
            }
        })
        .collect();

    println!("Q1: {}", q1(&data));
    println!("Q2: {}", q2(&data));
}

fn q1(data: &Vec<u32>) -> u32 {
    let mut counter = 0;

    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            counter += 1;
        }
    }

    counter
}

fn q2(data: &Vec<u32>) -> u32 {
    let mut counter = 0;
    let ub = data.len() - 2;

    for i in 0..data.len() {
        let mut prevsum = 0;
        let mut currsum = 0;

        if i < ub {
            currsum += data[i];
        }

        if (i as i32 - 1) >= 0 && i - 1 < ub {
            currsum += data[i];
            prevsum += data[i - 1]
        }

        if (i as i32 - 2) >= 0 && i - 2 < ub {
            currsum += data[i];
            prevsum += data[i - 2]
        }

        if (i as i32 - 3) >= 0 && i - 3 < ub {
            prevsum += data[i - 3]
        }

        if currsum > prevsum {
            counter += 1;
        }
    }

    counter
}
