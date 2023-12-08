#[derive(Clone, Debug)]
struct Race {
    time: i128,
    distance: i128,
}

fn get_races(data: Vec<String>) -> Vec<Race> {
    let s_times = &data[0]
        .split(" ")
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()[1..];
    let s_distances = &data[1]
        .split(" ")
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()[1..];

    let mut iter_times = s_times.into_iter();
    let mut iter_distances = s_distances.into_iter();

    let mut races: Vec<Race> = vec![];

    while iter_times.len() > 0 {
        races.push(Race {
            time: iter_times.next().unwrap().parse().unwrap(),
            distance: iter_distances.next().unwrap().parse().unwrap(),
        })
    }

    return races;
}

fn get_race(data: Vec<String>) -> Vec<Race> {
    let s_times = &data[0]
        .split(" ")
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()[1..];
    let s_distances = &data[1]
        .split(" ")
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()[1..];

    let time = s_times.join("");
    let distance = s_distances.join("");

    return [Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    }]
    .to_vec();
}

fn get_winning_pushes(race: Race) -> i128 {
    let mut win = false;
    let mut win_first = 0;
    let mut win_last = 0;
    let mut time = 0;
    while win == false && time <= race.time {
        if time * (race.time - time) > race.distance {
            win = true;
            win_first = time;
        }
        time += 1;
    }
    time = race.time;
    win = false;
    while win == false && time >= 0 {
        if time * (race.time - time) > race.distance {
            win = true;
            win_last = time;
        }
        time -= 1;
    }

    return win_last - win_first + 1;
}

pub fn part1(data: Vec<String>) -> String {
    return get_races(data)
        .into_iter()
        .map(|x| get_winning_pushes(x))
        .fold(1, |acc, x| acc * x)
        .to_string();
}

pub fn part2(data: Vec<String>) -> String {
    return get_race(data)
        .into_iter()
        .map(|x| get_winning_pushes(x))
        .fold(1, |acc, x| acc * x)
        .to_string();
}
