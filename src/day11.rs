use std::{collections::HashMap, vec};

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    number: i128,
    x: i128,
    y: i128,
}

fn build_galaxy_map(
    data: Vec<String>,
    increasing_size: i128,
) -> (HashMap<i128, Galaxy>, Vec<[i128; 2]>) {
    let mut map: Vec<Vec<i128>> = vec![];
    let mut galaxies: HashMap<i128, Galaxy> = HashMap::new();
    let mut galaxy_pairs: Vec<[i128; 2]> = vec![];
    let mut galaxy_number: i128 = 1;
    for (i, row) in data.into_iter().enumerate() {
        let mut map_row = vec![];
        for (j, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    galaxies.insert(
                        galaxy_number,
                        Galaxy {
                            number: galaxy_number,
                            x: j as i128,
                            y: i as i128,
                        },
                    );
                    map_row.push(galaxy_number);
                    galaxy_number += 1;
                }
                _ => {
                    map_row.push(-1);
                }
            }
        }
        map.push(map_row);
    }

    // expand rows
    for (i, row) in map.clone().into_iter().enumerate() {
        if row.clone().into_iter().all(|x| x == -1) {
            for (y, r) in map.clone()[i..].to_vec().into_iter().enumerate() {
                for (x, c) in r.clone().into_iter().enumerate() {
                    if c != -1 {
                        let galaxy = galaxies[&c];
                        galaxies.insert(
                            c,
                            Galaxy {
                                number: galaxy.number,
                                x: galaxy.x,
                                y: galaxy.y + increasing_size,
                            },
                        );
                    }
                }
            }
        }
    }

    // expand columns
    for i in 0..map[0].len() {
        let mut to_check = vec![];
        for j in 0..map.len() {
            to_check.push(map[j][i]);
        }
        if to_check.iter().all(|i| i == &-1) {
            for (y, r) in map.clone().into_iter().enumerate() {
                for (x, c) in r.clone()[i..].into_iter().enumerate() {
                    if c != &-1 {
                        let galaxy = galaxies[c];
                        galaxies.insert(
                            *c,
                            Galaxy {
                                number: galaxy.number,
                                x: galaxy.x + increasing_size,
                                y: galaxy.y,
                            },
                        );
                    }
                }
            }
        }
    }

    let mut g = galaxies.keys().collect::<Vec<_>>();
    for i in galaxies.keys() {
        for j in g.clone() {
            if i != j {
                galaxy_pairs.push([*i, *j]);
            }
        }
        g = g[1..].to_vec();
    }

    return (galaxies, galaxy_pairs);
}

fn get_distance(g1: Galaxy, g2: Galaxy) -> i128 {
    let rise = g2.y - g1.y;
    let run = g2.x - g1.x;

    let mut distance = rise.abs() + run.abs();
    if run == 0 {
        distance = rise.abs();
    } else if rise == 0 {
        distance = run.abs();
    }
    return distance;
}

pub fn part1(data: Vec<String>) -> String {
    let (galaxies, galaxy_pairs) = build_galaxy_map(data, 1);
    let mut distances = vec![];
    for pair in galaxy_pairs {
        distances.push(get_distance(galaxies[&pair[0]], galaxies[&pair[1]]));
    }

    return distances.iter().sum::<i128>().to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (galaxies, galaxy_pairs) = build_galaxy_map(data, 999999);
    let mut distances = vec![];
    for pair in galaxy_pairs {
        distances.push(get_distance(galaxies[&pair[0]], galaxies[&pair[1]]));
    }

    return distances.iter().sum::<i128>().to_string();
}
