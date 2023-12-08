use std::collections::HashMap;

fn get_map(data: Vec<String>) -> (String, HashMap<String, Vec<String>>) {
    let directions = data[0].to_string();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for text in &data[2..] {
        let parts = text
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let node = parts[0].to_owned();
        let left = parts[2].replace("(", "").replace(",", "");
        let right = parts[3].replace(")", "");

        map.insert(node, [left, right].to_vec());
    }
    return (directions, map);
}

fn walk_map(
    start_node: String,
    directions: String,
    map: HashMap<String, Vec<String>>,
) -> Vec<i128> {
    let mut start = &start_node;
    let mut step_count: i128 = 0;
    let mut next;
    let mut to_walk = directions.chars();
    let mut steps_to_ends: Vec<i128> = vec![];

    while start.chars().last().unwrap() != 'Z' {
        for c in to_walk {
            step_count += 1;
            if c == 'L' {
                next = &map[start][0];
            } else {
                next = &map[start][1];
            }
            if next.clone().chars().last().unwrap() == 'Z' {
                steps_to_ends.push(step_count);
            }
            start = next;
        }
        to_walk = directions.chars();
    }

    return steps_to_ends;
}

pub fn get_lcm(nums: &mut Vec<i128>) -> i128 {
    nums.sort();

    fn gcd(a: i128, b: i128) -> i128 {
        if b == 0 {
            return a;
        } else {
            return gcd(b, a % b);
        }
    }

    fn lcm(a: i128, b: i128) -> i128 {
        return (a * b) / gcd(a, b);
    }

    let low = nums[0];

    nums.iter().fold(low, |acc, x| lcm(acc, *x))
}

pub fn part1(data: Vec<String>) -> String {
    let (directions, map) = get_map(data);
    return walk_map("AAA".to_string(), directions, map)[0].to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (directions, map) = get_map(data);
    let start_keys = map
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect::<Vec<_>>();

    let mut end_steps = vec![];
    for i in start_keys {
        end_steps.append(&mut walk_map(i.to_owned(), directions.clone(), map.clone()));
    }

    let lcm = get_lcm(&mut end_steps);

    println!("fish {:#?}", lcm);
    return lcm.to_string();
}
