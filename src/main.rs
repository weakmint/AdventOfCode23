#[macro_use]
extern crate tramp;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec;

fn main() {
    let day_num = std::env::args()
        .nth(1)
        .expect("no day given (just the number)");
    let test = std::env::args().nth(2);

    let filename: String;
    match test {
        Some(ref test) => filename = format!("test{}.txt", test.clone()),
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
            "4" => {
                result1 = day4::part1(vec_data.clone());
                result2 = day4::part2(vec_data.clone());
            }
            "5" => {
                result1 = day5::part1(vec_data.clone());
                result2 = day5::part2(vec_data.clone());
            }
            "6" => {
                result1 = day6::part1(vec_data.clone());
                result2 = day6::part2(vec_data.clone());
            }
            "7" => {
                result1 = day7::part1(vec_data.clone());
                result2 = day7::part2(vec_data.clone());
            }
            "8" => {
                result1 = if test == None || test.unwrap().parse::<i32>().unwrap() != 3 {
                    day8::part1(vec_data.clone())
                } else {
                    "Not applicable for test 3".to_string()
                };
                result2 = day8::part2(vec_data.clone());
            }
            "9" => {
                result1 = day9::part1(vec_data.clone());
                result2 = day9::part2(vec_data.clone());
            }
            "10" => {
                result1 = if test == None || test.unwrap().parse::<i32>().unwrap() != 3 {
                    day10::part1(vec_data.clone())
                } else {
                    "Not applicable for test 3".to_string()
                };
                result2 = day10::part2(vec_data.clone());
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
