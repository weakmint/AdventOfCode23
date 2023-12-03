#[derive(Clone, Copy)]
struct Star {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy)]
struct Number {
    value: u32,
    star_index: isize,
}

fn bound_index(index: usize, bound: usize) -> usize {
    let signed_index: isize = index.try_into().unwrap();
    match bound {
        0 => {
            if signed_index - 1 < 0 {
                return 0;
            } else {
                return index - 1;
            }
        }
        _ => {
            if index + 1 > bound {
                return bound;
            } else {
                return index + 1;
            }
        }
    }
}

fn get_star_pos(i: usize, center_x: usize, center_y: usize) -> (usize, usize) {
    let star_x: usize;
    let star_y: usize;

    let rel_x = (i + 1) % 3;
    let rel_y = (i / 3 + 1) % 3;

    match rel_x {
        1 => star_x = center_x - 1,
        2 => star_x = center_x,
        0 => star_x = center_x + 1,
        _ => panic!("fish"),
    }

    match rel_y {
        1 => star_y = center_y - 1,
        2 => star_y = center_y,
        0 => star_y = center_y + 1,
        _ => panic!("fish"),
    }

    return (star_x, star_y);
}

fn has_symbol(x: usize, y: usize, map: &Vec<Vec<char>>) -> (bool, Option<Star>) {
    let x_len = map.first().unwrap().len();
    let to_check = [
        &map[bound_index(y, 0)][bound_index(x, 0)..bound_index(x + 1, x_len)],
        &map[y][bound_index(x, 0)..bound_index(x + 1, x_len)],
        &map[bound_index(y, map.len() - 1)][bound_index(x, 0)..bound_index(x + 1, x_len)],
    ]
    .concat();

    let mut symbol = false;
    let mut star: Star = Star { x: 0, y: 0 };
    for (i, c) in to_check.iter().enumerate() {
        if !c.is_digit(10) && c != &'.' {
            symbol = true;
            if c == &'*' {
                let (star_x, star_y) = get_star_pos(i, x, y);
                star = Star {
                    x: star_x as u32,
                    y: star_y as u32,
                };
            }
        }
    }

    if star.x == 0 && star.y == 0 {
        return (symbol, None);
    }
    return (symbol, Some(star));
}

fn get_map(data: Vec<String>) -> Vec<Vec<char>> {
    let mut map = vec![];
    for row in data {
        let mut row_vec = vec![];
        for c in row.chars() {
            row_vec.push(c);
        }
        map.push(row_vec);
    }
    return map;
}

fn get_star_index(star: Star, stars: &Vec<Star>) -> Option<usize> {
    let mut stars_enum = stars.iter();
    let existing_star = stars_enum.position(|&s| s.x == star.x && s.y == star.y);

    match existing_star {
        Some(x) => Some(x),
        None => None,
    }
}

fn get_part_numbers(map: Vec<Vec<char>>) -> (Vec<Number>, Vec<Star>) {
    let mut numbers: Vec<Number> = vec![];
    let mut stars: Vec<Star> = vec![];
    let mut curr_num: Vec<char> = vec![];
    let mut is_part_num: bool = false;
    let mut star_index: isize = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                _is_digit if c.is_digit(10) => {
                    let (symbol, star) = has_symbol(x, y, &map);
                    if symbol {
                        is_part_num = true;
                        match star {
                            Some(x) => match get_star_index(x, &stars) {
                                Some(index) => star_index = index.try_into().unwrap(),
                                None => {
                                    stars.push(x);
                                    star_index =
                                        TryInto::<isize>::try_into(stars.len()).unwrap() - 1;
                                }
                            },
                            None => {}
                        }
                    }
                    curr_num.push(c.clone());
                }
                _end_of_number if !c.is_digit(10) => {
                    if is_part_num {
                        let number = String::from_iter(&curr_num);
                        numbers.push(Number {
                            value: number.parse().unwrap(),
                            star_index: star_index,
                        });
                    }
                    curr_num = vec![];
                    is_part_num = false;
                    star_index = -1;
                }
                &_ => panic!("fish"),
            }
        }
        if is_part_num {
            let number = String::from_iter(&curr_num);
            numbers.push(Number {
                value: number.parse().unwrap(),
                star_index: star_index,
            });
        }
        curr_num = vec![];
        is_part_num = false;
        star_index = -1;
    }
    return (numbers, stars);
}

fn get_gears(numbers: &Vec<Number>, stars: &Vec<Star>) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = vec![];
    for (i, _) in stars.iter().enumerate() {
        let gear_numbers: Vec<&Number> = numbers
            .into_iter()
            .filter(|number| number.star_index == i.try_into().unwrap())
            .collect();
        if gear_numbers.len() == 2 {
            gear_ratios.push(gear_numbers[0].value * gear_numbers[1].value);
        }
    }
    return gear_ratios;
}

pub fn part1(data: Vec<String>) -> String {
    let mut sum: u32 = 0;
    let (numbers, _) = get_part_numbers(get_map(data));
    for number in numbers {
        sum += number.value;
    }

    return sum.to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let mut sum: u32 = 0;
    let (numbers, stars) = get_part_numbers(get_map(data));
    let gear_ratios = get_gears(&numbers, &stars);
    for gear_ratio in gear_ratios {
        sum += gear_ratio;
    }

    return sum.to_string();
}
