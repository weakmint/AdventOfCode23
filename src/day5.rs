use std::{collections::{hash_map, HashMap}, vec};

#[derive(Clone, Debug)]
struct Map {
    source_min: i64,
    source_max: i64,
    destination_min: i64,
    destination_max: i64,
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
                };

                curr_maps.push(map);
            }
        }
    }

    return (seeds, maps);
}

fn get_seed_location(seed: i64, maps: &HashMap<String, Vec<Map>>) -> i64 {
    let mut number = seed;
    println!("seed: {}", number);
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
    let key = map_keys[0];
    let mut min = num_min;
    let mut max = num_max;
    for map in maps[key] {
        let min_contained = between(min, map.source_min, map.source_max);
        let max_contained: = between(max, map.source_min, map.source_max);
        if min_contained && max_contained {
            let next
            return chunk_it_up(map.destination_min, map.destination_max, maps, map_keys.clone())
        }
    }

    return smallest
}

pub fn part1(data: Vec<String>) -> String {
    let (seeds, maps) = parse_data(data);
    println!("{:#?}", maps);
    let mut locations: Vec<i64> = seeds.iter().map(|x| get_seed_location(*x, &maps)).collect();
    println!("{:?}", locations);
    locations.sort_unstable();
    return locations[0].to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (seeds, maps) = parse_data(data);
    let mut locations: Vec<i64> = vec![];
    let seed_ranges = get_seed_ranges(seeds);
    //for range in seed_ranges {
    //    //for i in range.start..range.range {
    //    //    locations.push(get_seed_location(i, &maps))
    //    //}
    //}
    let fish: i64 = seed_ranges
        .iter()
        .map(|x| x.seed_max - x.seed_min + 1)
        .sum();
    println!("{}", fish);
    locations.sort_unstable();
    return locations[0].to_string();
}
