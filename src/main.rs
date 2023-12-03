mod day1;
mod day2;
mod day3;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let day_num = std::env::args()
        .nth(1)
        .expect("no day given (just the number)");
    let test = std::env::args().nth(2);

    let filename: String;
    match test {
        Some(test) => filename = format!("test{}.txt", test),
        None => filename = "data.txt".to_string(),
    }

    let path = format!("data/day{}/{}", day_num, filename);
    println!("{}", path);

    let mut result1: String = "".to_string();
    let mut result2: String = "".to_string();

    if let Ok(data) = read_lines(&path) {
        let mut vec_data = vec![];
        for line in data {
            if let Ok(text) = line {
                vec_data.push(text)
            }
        }
        match &day_num[..] {
            "1" => result2 = day1::part2(vec_data.clone()),
            "2" => {
                result1 = day2::part1(vec_data.clone());
                result2 = day2::part2(vec_data.clone());
            }
            "3" => {
                result1 = day3::part1(vec_data.clone());
                result2 = day3::part2(vec_data.clone());
            }
            _ => panic!("invalid day passed"),
        }
    }

    println!("result1 is {}", result1);
    println!("result2 is {}", result2);
}

fn read_lines(filename: &str) -> std::io::Result<std::io::Lines<BufReader<std::fs::File>>> {
    let root = std::env::current_dir()?;

    let relative_path = relative_path::RelativePath::new(filename);
    let path = relative_path.to_path(&root);
    let file = File::open(path)?;
    Ok(std::io::BufReader::new(file).lines())
}
