use std::{
    collections::{hash_map, HashMap},
    vec,
};

#[derive(Clone, Debug)]
struct Map {
    source_min: i64,
    source_max: i64,
    destination_min: i64,
    destination_max: i64,
    range: i64,
}

#[derive(Clone, Debug)]
struct SeedRange {
    seed_min: i64,
    seed_max: i64,
}

const MAP_KEYS: [&str; 7] = [
    "seed2soil",
    "soil2fertilizer",
    "fertilizer2water",
    "water2light",
    "light2temperature",
    "temperature2humidity",
    "humidity2location",
];

fn parse_data(data: Vec<String>) -> (Vec<i64>, HashMap<String, Vec<Map>>) {
    let mut maps: HashMap<String, Vec<Map>> = HashMap::new();

    let mut iter = data.iter();

    let seeds: Vec<i64> = iter
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let keys = MAP_KEYS.clone();
    let mut hash_keys = keys.iter();

    iter.next();

    let mut curr_maps: Vec<Map> = vec![];
    let mut curr_map_key: String = "".to_string();
    for (i, s) in iter.clone().enumerate() {
        match s.as_str() {
            _empty if s.is_empty() || iter.len() - 1 == i => {
                maps.insert(curr_map_key.to_string(), curr_maps.to_vec());
            }
            _new_map if s.contains("map") => {
                curr_map_key = hash_keys.next().unwrap().to_string();
                curr_maps = vec![];
            }
            _ => {
                let parts: Vec<i64> = s.split(' ').map(|x| x.parse().unwrap()).collect();
                let map = Map {
                    source_min: parts[1],
                    source_max: parts[1] + parts[2] - 1,
                    destination_min: parts[0],
                    destination_max: parts[0] + parts[2] - 1,
                    range: parts[2] - 1,
                };

                curr_maps.push(map);
            }
        }
    }

    return (seeds, maps);
}

fn get_seed_location(seed: i64, maps: &HashMap<String, Vec<Map>>) -> i64 {
    let mut number = seed;
    for i in MAP_KEYS {
        let curr_maps = &maps[i];
        for map in curr_maps {
            if number >= map.source_min && number <= map.source_max {
                number = map.destination_min + number - map.source_min;
                break;
            }
        }
    }
    return number;
}

fn get_seed_ranges(seeds: Vec<i64>) -> Vec<SeedRange> {
    let mut seed_range = vec![];
    let mut iter = seeds.iter();
    while let Some(seed) = iter.next() {
        seed_range.push(SeedRange {
            seed_min: *seed,
            seed_max: *seed + *iter.next().unwrap() - 1,
        })
    }

    println!("{:#?}", seed_range);
    return seed_range;
}

fn between(t: i64, min: i64, max: i64) -> bool {
    return min <= t && t <= max;
}

fn chunk_it_up(
    num_min: i64,
    num_max: i64,
    maps: &HashMap<String, Vec<Map>>,
    map_keys: &Vec<&str>,
) -> Vec<Vec<i64>> {
    let mut smallest: Vec<Vec<i64>> = vec![];
    let min = num_min;
    let max = num_max;
    if map_keys.len() > 0 {
        let key = map_keys[0];
        let next_keys = if map_keys.len() > 1 {
            &map_keys[..1]
        } else {
            &map_keys
        };
        //println!("{}", key);
        for (i, map) in maps[key].clone().into_iter().enumerate() {
            //println!("key {} min {} max {}", key, map.source_min, map.source_max);
            let min_contained = between(min, map.source_min, map.source_max);
            let max_contained = between(max, map.source_min, map.source_max);
            if min < map.source_min && max > map.source_max {
                //println!("overflow");
                //println!("min {} max {}", min, max);
                // left
                smallest.append(&mut chunk_it_up(
                    min,
                    map.source_min - 1,
                    maps,
                    &map_keys.clone(),
                ));
                //center
                smallest.append(&mut chunk_it_up(
                    map.destination_min,
                    map.destination_max,
                    maps,
                    &mut next_keys.to_vec(),
                ));
                // right
                smallest.append(&mut chunk_it_up(
                    map.source_max + 1,
                    max,
                    maps,
                    &map_keys.clone(),
                ));
                break;
            } else if min_contained && max_contained {
                //println!("both");
                //println!("min {} max {}", min, max);
                if map_keys.len() == 1 {
                    smallest.push([min].to_vec());
                } else {
                    smallest.append(&mut chunk_it_up(
                        min - map.source_min + map.destination_min,
                        max - map.source_min + map.destination_min,
                        maps,
                        &mut next_keys.to_vec(),
                    ));
                }
                break;
            } else if min_contained {
                //println!("min");
                //println!("min {} max {}", min, max);
                if map_keys.len() == 1 {
                    smallest.push([min].to_vec());
                } else {
                    smallest.append(&mut chunk_it_up(
                        min - map.source_min + map.destination_min,
                        map.destination_max,
                        maps,
                        &mut next_keys.to_vec(),
                    ));
                    smallest.append(&mut chunk_it_up(
                        map.source_max + 1,
                        max,
                        maps,
                        &map_keys.clone(),
                    ));
                    //println!("min {} max {}", min, max);
                }
                break;
            } else if max_contained {
                //println!("max");
                //println!("min {} max {}", min, max);
                if map_keys.len() == 1 {
                    smallest.push([min].to_vec());
                } else {
                    smallest.append(&mut chunk_it_up(
                        map.destination_min,
                        max - map.source_min + map.destination_min,
                        maps,
                        &mut next_keys.to_vec(),
                    ));
                    //println!("garf! {}", key);
                    smallest.append(&mut chunk_it_up(
                        min,
                        map.source_min - 1,
                        maps,
                        &map_keys.clone(),
                    ));
                }
                break;
            } //println!("oop");
            if i == maps[key].len() - 1 {
                //println!("neither");
                smallest.append(&mut chunk_it_up(min, max, maps, &mut next_keys.to_vec()));
            }
        }
    } else {
        smallest.push([min].to_vec());
    }
    // println!("{} {} {:#?}", min, max, smallest);
    return smallest;
}

pub fn part1(data: Vec<String>) -> String {
    let (seeds, maps) = parse_data(data);
    let mut locations: Vec<i64> = seeds.iter().map(|x| get_seed_location(*x, &maps)).collect();
    locations.sort_unstable();
    return locations[0].to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (seeds, maps) = parse_data(data);
    println!("{:#?}", maps);
    let mut locations: Vec<Vec<i64>> = vec![];
    let seed_ranges = get_seed_ranges(seeds);
    for range in seed_ranges {
        locations.append(&mut chunk_it_up(
            range.seed_min,
            range.seed_max,
            &maps,
            &MAP_KEYS.to_vec(),
        ));
        //println!("{} {}", range.seed_min, range.seed_max);
    }
    let mut ehhhh = locations.iter().map(|x| x[0]).collect::<Vec<i64>>();
    ehhhh.sort();
    //println!("{:#?}", ehhhh);
    return ehhhh[0].to_string();
}
