const FILEPATH: &str = "src/input.txt";

fn main() {
    let data: Vec<Vec<char>> = std::fs::read_to_string(FILEPATH)
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    println!("Q1 answer is: {}", q1(&data));
    println!("Q2 answer is: {}", q2(&data));
}

fn q1(data: &Vec<Vec<char>>) -> i32 {
    let l = data[0].len();
    let mut pockets = vec![0; l];

    for val in data {
        for i in 0..l {
            if val[i] == '1' {
                pockets[i] += 1;
            }
        }
    }

    let gamma = i32::from_str_radix(
        pockets
            .iter()
            .map(|x| {
                if 2 * x >= data.len() {
                    return "1";
                }
                "0"
            })
            .collect::<Vec<&str>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap();

    let epsilon = i32::from_str_radix(
        pockets
            .iter()
            .map(|x| {
                if 2 * x < data.len() {
                    return "1";
                }
                "0"
            })
            .collect::<Vec<&str>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap();

    return gamma * epsilon;
}

fn q2(data: &Vec<Vec<char>>) -> i32 {
    let mut cd = data.clone();
    let mut ox = data.clone();
    let len = data[0].len();
    let mut idx = 0;

    let commonchar = |cd: Vec<Vec<char>>, idx: usize, ox: bool| -> Vec<Vec<char>> {
        let c1 = ox.then(|| '1').unwrap_or('0');
        let c2 = ox.then(|| '0').unwrap_or('1');

        let mut ones = 0;
        for val in &cd {
            if val[idx] == '1' {
                ones += 1;
            }
        }

        let c = (ones >= cd.len() - ones).then(|| c1).unwrap_or(c2);

        cd.into_iter().filter(|val| val[idx] == c).collect()
    };

    while (cd.len() != 1 || ox.len() != 1) && idx < len {
        if ox.len() != 1 {
            ox = commonchar(ox, idx, true);
        }
        if cd.len() != 1 {
            cd = commonchar(cd, idx, false);
        }
        idx += 1;
    }

    let ox = i32::from_str_radix(ox[0].iter().collect::<String>().as_str(), 2).unwrap();
    let cd = i32::from_str_radix(cd[0].iter().collect::<String>().as_str(), 2).unwrap();

    return ox * cd;
}
