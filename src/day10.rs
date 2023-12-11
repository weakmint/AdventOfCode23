use std::{collections::HashMap, fs::DirEntry};
use tramp::{tramp, Rec};

#[derive(Clone, Debug)]
struct Pipe {
    shape: char,
    is_start: bool,
    x: i32,
    y: i32,
    north: String,
    south: String,
    east: String,
    west: String,
    inside: Vec<char>,
}

fn get_pipe_id(x: i32, y: i32) -> String {
    return format!("{},{}", x, y);
}

fn get_connected_pipes(
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
    shape: char,
) -> (String, char, String, char) {
    let north = if y - 1 >= 0 { y - 1 } else { -1 };
    let south = if y + 1 <= max_y { y + 1 } else { -1 };
    let west = if x - 1 >= 0 { x - 1 } else { -1 };
    let east = if x + 1 <= max_x { x + 1 } else { -1 };
    match shape {
        '|' => (get_pipe_id(x, north), 'n', get_pipe_id(x, south), 's'),
        '-' => (get_pipe_id(east, y), 'e', get_pipe_id(west, y), 'w'),
        'L' => (get_pipe_id(x, north), 'n', get_pipe_id(east, y), 'e'),
        'J' => (get_pipe_id(x, north), 'n', get_pipe_id(west, y), 'w'),
        '7' => (get_pipe_id(x, south), 's', get_pipe_id(west, y), 'w'),
        'F' => (get_pipe_id(x, south), 's', get_pipe_id(east, y), 'e'),
        _ => ("".to_string(), 'x', "".to_string(), 'x'),
    }
}

fn set_sides(s1: String, s1s: char, s2: String, s2s: char) -> (String, String, String, String) {
    let (mut north, mut south, mut east, mut west): (String, String, String, String) =
        ("".to_owned(), "".to_owned(), "".to_owned(), "".to_owned());
    for i in [[s1, s1s.to_string()], [s2, s2s.to_string()]] {
        match i[1].as_str() {
            "n" => north = i[0].clone(),
            "s" => south = i[0].clone(),
            "e" => east = i[0].clone(),
            "w" => west = i[0].clone(),
            _ => panic!("fish!"),
        }
    }
    return (north, south, east, west);
}

fn determine_s_shape(
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
    pipe_map: &mut HashMap<String, Pipe>,
) -> char {
    let north = if y - 1 >= 0 { y - 1 } else { -1 };
    let south = if y + 1 <= max_y { y + 1 } else { -1 };
    let west = if x - 1 >= 0 { x - 1 } else { -1 };
    let east = if x + 1 <= max_x { x + 1 } else { -1 };

    let (north_pipe, south_pipe, east_pipe, west_pipe) = (
        get_pipe_id(x, north),
        get_pipe_id(x, south),
        get_pipe_id(east, y),
        get_pipe_id(west, y),
    );
    let (mut has_north, mut has_south, mut has_east, mut has_west): (bool, bool, bool, bool) =
        (false, false, false, false);

    if pipe_map.contains_key(&north_pipe) && !pipe_map[&north_pipe].south.is_empty() {
        has_north = true;
    }
    if pipe_map.contains_key(&south_pipe) && !pipe_map[&south_pipe].north.is_empty() {
        has_south = true;
    }
    if pipe_map.contains_key(&east_pipe) && !pipe_map[&east_pipe].west.is_empty() {
        has_east = true;
    }
    if pipe_map.contains_key(&west_pipe) && !pipe_map[&west_pipe].east.is_empty() {
        has_west = true;
    }

    if has_north {
        if has_east {
            return 'L';
        }
        if has_west {
            return 'J';
        }
        if has_south {
            return '|';
        }
    }
    if has_south {
        if has_east {
            return 'F';
        }
        if has_west {
            return '7';
        }
    }

    return '-';
}

fn build_pipe_network(data: Vec<String>) -> (HashMap<String, Pipe>, String) {
    let mut pipe_map: HashMap<String, Pipe> = HashMap::new();
    let pipes = data.iter().map(|x| x.chars().collect::<Vec<_>>());
    let (mut start_x, mut start_y): (Option<i32>, Option<i32>) = (None, None);
    let (max_x, max_y) = (
        pipes.clone().last().unwrap().len() as i32 - 1,
        pipes.clone().count() as i32 - 1,
    );
    for (i, row) in pipes.enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            let pipe_id = get_pipe_id(j as i32, i as i32);
            match pipe {
                '.' => {
                    pipe_map.insert(
                        pipe_id,
                        Pipe {
                            shape: *pipe,
                            is_start: false,
                            x: j as i32,
                            y: i as i32,
                            north: "".to_string(),
                            south: "".to_string(),
                            east: "".to_string(),
                            west: "".to_string(),
                            inside: vec![],
                        },
                    );
                }
                'S' => {
                    start_x = Some(j as i32);
                    start_y = Some(i as i32);
                }
                _ => {
                    let (s1, s1s, s2, s2s) =
                        get_connected_pipes(j as i32, i as i32, max_x, max_y, *pipe);
                    let (north, south, east, west) = set_sides(s1, s1s, s2, s2s);
                    pipe_map.insert(
                        pipe_id,
                        Pipe {
                            shape: *pipe,
                            is_start: false,
                            x: j as i32,
                            y: i as i32,
                            north: north,
                            south: south,
                            east: east,
                            west: west,
                            inside: vec![],
                        },
                    );
                }
            }
        }
    }

    if start_x == None && start_y == None {
        panic!("Couldn't find start!");
    }
    let start_shape = determine_s_shape(
        start_x.unwrap(),
        start_y.unwrap(),
        max_x,
        max_y,
        &mut pipe_map,
    );
    let (s1, s1s, s2, s2s) = get_connected_pipes(
        start_x.unwrap(),
        start_y.unwrap(),
        max_x,
        max_y,
        start_shape,
    );
    let (north, south, east, west) = set_sides(s1, s1s, s2, s2s);
    pipe_map.insert(
        get_pipe_id(start_x.unwrap(), start_y.unwrap()),
        Pipe {
            shape: start_shape,
            is_start: true,
            x: start_x.unwrap() as i32,
            y: start_y.unwrap() as i32,
            north: north,
            south: south,
            east: east,
            west: west,
            inside: vec![],
        },
    );

    return (pipe_map, get_pipe_id(start_x.unwrap(), start_y.unwrap()));
}

fn path(pipe_map: HashMap<String, Pipe>, start_id: String) -> Vec<String> {
    fn travel(
        direction: char,
        start: String,
        path: Vec<String>,
        pipe_map: HashMap<String, Pipe>,
    ) -> Rec<Vec<String>> {
        let start_pipe = &pipe_map[&start];
        let (mut next_pipe_id, mut next_incoming_direction, mut next_direction): (
            Option<String>,
            Option<char>,
            Option<char>,
        ) = (None, None, None);

        match direction {
            'n' => {
                next_pipe_id = Some(start_pipe.north.clone());
                next_incoming_direction = Some('s');
            }
            's' => {
                next_pipe_id = Some(start_pipe.south.clone());
                next_incoming_direction = Some('n');
            }
            'e' => {
                next_pipe_id = Some(start_pipe.east.clone());
                next_incoming_direction = Some('w');
            }
            'w' => {
                next_pipe_id = Some(start_pipe.west.clone());
                next_incoming_direction = Some('e');
            }
            _ => panic!("fish!"),
        }

        let id = next_pipe_id.unwrap();
        let next_pipe = &pipe_map[&id.clone()];

        let (next_north, next_south, next_east, next_west) = (
            &next_pipe.north,
            &next_pipe.south,
            &next_pipe.east,
            &next_pipe.west,
        );

        match next_incoming_direction.unwrap() {
            'n' => {
                next_direction = Some(if pipe_map.contains_key(next_south) {
                    's'
                } else if pipe_map.contains_key(next_east) {
                    'e'
                } else if pipe_map.contains_key(next_west) {
                    'w'
                } else {
                    panic!("fish!");
                })
            }
            's' => {
                next_direction = Some(if pipe_map.contains_key(next_north) {
                    'n'
                } else if pipe_map.contains_key(next_east) {
                    'e'
                } else if pipe_map.contains_key(next_west) {
                    'w'
                } else {
                    panic!("fish!");
                })
            }
            'e' => {
                next_direction = Some(if pipe_map.contains_key(next_north) {
                    'n'
                } else if pipe_map.contains_key(next_south) {
                    's'
                } else if pipe_map.contains_key(next_west) {
                    'w'
                } else {
                    panic!("fish!");
                })
            }
            'w' => {
                next_direction = Some(if pipe_map.contains_key(next_north) {
                    'n'
                } else if pipe_map.contains_key(next_south) {
                    's'
                } else if pipe_map.contains_key(next_east) {
                    'e'
                } else {
                    panic!("fish!");
                })
            }
            _ => panic!("fish!"),
        }

        match next_pipe.is_start {
            true => rec_ret!(path.to_vec()),
            _ => {
                let mut new_path = path.clone();
                new_path.push(id.clone());
                rec_call!(travel(next_direction.unwrap(), id, new_path, pipe_map,));
            }
        }
    }

    let start_node = &pipe_map[&start_id];
    let mut start_direction: Option<char> = None;
    if pipe_map.contains_key(&start_node.north) {
        start_direction = Some('n');
    }
    if pipe_map.contains_key(&start_node.south) {
        start_direction = Some('s');
    }
    if pipe_map.contains_key(&start_node.east) {
        start_direction = Some('e');
    }
    if pipe_map.contains_key(&start_node.west) {
        start_direction = Some('w');
    }

    let mut path = tramp(travel(
        start_direction.unwrap(),
        start_id,
        vec![],
        pipe_map.clone(),
    ));
    let mut startpath = [get_pipe_id(start_node.x, start_node.y)].to_vec();
    startpath.append(&mut path.clone());
    return startpath;
}

fn get_insides(path: Vec<String>, pipe_map: &mut HashMap<String, Pipe>) -> Vec<String> {
    let mut inside_direction: Option<Vec<char>> = None;
    let mut prev_shape: char = pipe_map[&path[0]].shape;
    match prev_shape {
        '-' => inside_direction = Some(['s'].to_vec()),
        '7' => inside_direction = Some(['n', 'e'].to_vec()),
        'F' => inside_direction = Some(['s', 'e'].to_vec()),
        '|' => inside_direction = Some(['e'].to_vec()),
        'J' => inside_direction = Some(['s', 'e'].to_vec()),
        'L' => inside_direction = Some(['n', 'e'].to_vec()),
        _ => panic!("Fish!"),
    }
    let set_inside_directions = || {
        for (mut i, id) in path.clone().iter().enumerate() {
            let start = &pipe_map[id];
            match start.shape {
                '-' => {
                    if inside_direction.unwrap().contains(&'s') {
                        inside_direction = Some(['s'].to_vec());
                    } else {
                        inside_direction = Some(['n'].to_vec());
                    }
                    prev_shape = '-'
                }
                '|' => {
                    if inside_direction.unwrap().contains(&'e') {
                        inside_direction = Some(['e'].to_vec());
                    } else {
                        inside_direction = Some(['w'].to_vec());
                    }
                    prev_shape = '|'
                }
                '7' => {
                    let mut new = vec![];
                    match prev_shape {
                        '7' => {
                            new = inside_direction.unwrap();
                        }
                        '-' => {
                            if inside_direction.unwrap().contains(&'n') {
                                new.push('n');
                                new.push('e');
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        '|' => {
                            if inside_direction.unwrap().contains(&'e') {
                                new.push('n');
                                new.push('e');
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        'F' => {
                            if inside_direction.unwrap().eq(&['n', 'w'].to_vec()) {
                                new.push('n');
                                new.push('e')
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        'L' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('n');
                                new.push('e')
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        'J' => {
                            if inside_direction.unwrap().eq(&['n', 'w'].to_vec()) {
                                new.push('s');
                                new.push('w')
                            } else {
                                new.push('n');
                                new.push('e');
                            }
                        }
                        _ => panic!("Fish!"),
                    }
                    prev_shape = '7';
                    inside_direction = Some(new);
                }
                'F' => {
                    let mut new = vec![];
                    match prev_shape {
                        'F' => {
                            new = inside_direction.unwrap();
                        }
                        '-' => {
                            if inside_direction.unwrap().contains(&'n') {
                                new.push('n');
                                new.push('w');
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        '|' => {
                            if inside_direction.unwrap().contains(&'e') {
                                new.push('s');
                                new.push('e');
                            } else {
                                new.push('n');
                                new.push('w');
                            }
                        }
                        '7' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('n');
                                new.push('w')
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        'L' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('s');
                                new.push('e')
                            } else {
                                new.push('n');
                                new.push('w');
                            }
                        }
                        'J' => {
                            if inside_direction.unwrap().eq(&['n', 'w'].to_vec()) {
                                new.push('n');
                                new.push('w')
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        _ => {
                            panic!("Fish!");
                        }
                    }
                    prev_shape = 'F';
                    inside_direction = Some(new);
                }
                'L' => {
                    let mut new = vec![];
                    match prev_shape {
                        'L' => {
                            new = inside_direction.unwrap();
                        }
                        '-' => {
                            if inside_direction.unwrap().contains(&'n') {
                                new.push('n');
                                new.push('e');
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        '|' => {
                            if inside_direction.unwrap().contains(&'e') {
                                new.push('n');
                                new.push('e');
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        '7' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('n');
                                new.push('e')
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        'F' => {
                            if inside_direction.unwrap().eq(&['s', 'e'].to_vec()) {
                                new.push('n');
                                new.push('e')
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        'J' => {
                            if inside_direction.unwrap().eq(&['n', 'w'].to_vec()) {
                                new.push('n');
                                new.push('e')
                            } else {
                                new.push('s');
                                new.push('w');
                            }
                        }
                        _ => panic!("Fish!"),
                    }
                    prev_shape = 'L';
                    inside_direction = Some(new);
                }
                'J' => {
                    let mut new = vec![];
                    match prev_shape {
                        'J' => {
                            new = inside_direction.unwrap();
                        }
                        '-' => {
                            if inside_direction.unwrap().contains(&'n') {
                                new.push('n');
                                new.push('w');
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        '|' => {
                            if inside_direction.unwrap().contains(&'e') {
                                new.push('s');
                                new.push('e');
                            } else {
                                new.push('n');
                                new.push('w');
                            }
                        }
                        '7' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('s');
                                new.push('e')
                            } else {
                                new.push('n');
                                new.push('w');
                            }
                        }
                        'F' => {
                            if inside_direction.unwrap().eq(&['n', 'w'].to_vec()) {
                                new.push('n');
                                new.push('w')
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        'L' => {
                            if inside_direction.unwrap().eq(&['n', 'e'].to_vec()) {
                                new.push('n');
                                new.push('w')
                            } else {
                                new.push('s');
                                new.push('e');
                            }
                        }
                        _ => panic!("Fish!"),
                    }
                    prev_shape = 'J';
                    inside_direction = Some(new);
                }
                _ => {
                    panic!("fish!");
                }
            }
            pipe_map.insert(
                id.to_string(),
                Pipe {
                    shape: start.shape,
                    is_start: start.is_start,
                    x: start.x,
                    y: start.y,
                    inside: inside_direction.as_mut().unwrap().to_vec(),
                    north: start.north.clone(),
                    south: start.south.clone(),
                    east: start.east.clone(),
                    west: start.west.clone(),
                },
            );
        }
    };
    set_inside_directions();
    let max_x = pipe_map.clone().values().map(|x| x.x).max().unwrap();
    let max_y = pipe_map.clone().values().map(|x| x.y).max().unwrap();
    println!("keys {}", pipe_map.clone().keys().len());
    return pipe_map
        .clone()
        .keys()
        .filter(|x| !path.contains(x))
        .map(|x| x.to_string())
        .collect();
}

fn trim_the_fat(
    insides: Vec<String>,
    path: Vec<String>,
    pipe_map: HashMap<String, Pipe>,
) -> (Vec<String>, Vec<String>) {
    let mut trimmed = vec![];
    let mut fat: Vec<String> = vec![];
    let min_x = 0;
    let min_y = 0;
    let max_x = pipe_map.clone().values().map(|x| x.x).max().unwrap();
    let max_y = pipe_map.clone().values().map(|x| x.y).max().unwrap();

    for i in insides {
        let (mut n, mut s, mut e, mut w): (bool, bool, bool, bool) = (false, false, false, false);
        let coords = i.split(",").collect::<Vec<_>>();
        let (inside_x, inside_y): (i32, i32) =
            (coords[0].parse().unwrap(), coords[1].parse().unwrap());
        fn has_bound(
            start_x: i32,
            start_y: i32,
            op_direction: char,
            x_mod: i32,
            y_mod: i32,
            path: Vec<String>,
            pipe_map: HashMap<String, Pipe>,
            fat: &mut Vec<String>,
        ) -> bool {
            let (mut x, mut y) = (x_mod, y_mod);
            loop {
                let node = get_pipe_id(start_x + x, start_y + y);
                x += x_mod;
                y += y_mod;
                if pipe_map.contains_key(&node) && pipe_map[&node].inside.contains(&op_direction) {
                    return true;
                } else if fat.contains(&node)
                    || !pipe_map.contains_key(&node)
                    || (path.clone().contains(&node)
                        && !pipe_map[&node].inside.contains(&op_direction))
                {
                    if !pipe_map.contains_key(&node) {
                        println!("uncontained {}", &node);
                    }
                    return false;
                }
            }
        }

        n = has_bound(
            inside_x,
            inside_y,
            's',
            0,
            -1,
            path.clone(),
            pipe_map.clone(),
            &mut fat,
        );
        s = has_bound(
            inside_x,
            inside_y,
            'n',
            0,
            1,
            path.clone(),
            pipe_map.clone(),
            &mut fat,
        );
        e = has_bound(
            inside_x,
            inside_y,
            'w',
            1,
            0,
            path.clone(),
            pipe_map.clone(),
            &mut fat,
        );
        w = has_bound(
            inside_x,
            inside_y,
            'e',
            -1,
            0,
            path.clone(),
            pipe_map.clone(),
            &mut fat,
        );

        if n && s && e && w {
            trimmed.push(i)
        } else {
            fat.push(i)
        }
    }

    return (trimmed, fat);
}

pub fn part1(data: Vec<String>) -> String {
    let (pipe_map, start_id) = build_pipe_network(data);
    let path = path(pipe_map, start_id);
    return (path.len() / 2).to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (mut pipe_map, start_id) = build_pipe_network(data);
    //let path = rearrange_path(path(pipe_map.clone(), start_id.clone()));
    let path = path(pipe_map.clone(), start_id.clone());

    let mut insides = get_insides(path.clone(), &mut pipe_map);
    // println!("path {:#?}", path);
    println!("{:#?}", insides.clone());
    insides.sort();
    let (trim, fat) = trim_the_fat(insides, path.clone(), pipe_map);
    println!("all {}", trim.len() + fat.len() + path.len());
    println!("trim {:#?}", trim);
    return trim.len().to_string();
}
