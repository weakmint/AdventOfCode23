#[derive(Clone, Copy)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
    min_red: u32,
    min_green: u32,
    min_blue: u32,
}

fn process_games(game_data: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for text in game_data {
        let parts = text.split(':').collect::<Vec<&str>>();
        let gameid: u32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();
        let mut game: Game = Game {
            id: gameid,
            rounds: vec![],
            min_red: 0,
            min_green: 0,
            min_blue: 0,
        };

        for rounds in parts[1].split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let round_parts = rounds.split(',').collect::<Vec<&str>>();
            for part in round_parts {
                let mut part_parts = part.split_whitespace();
                let num: u32 = part_parts.next().unwrap().parse().unwrap();
                let color = part_parts.next().unwrap();
                match color {
                    "red" => red = num,
                    "green" => green = num,
                    "blue" => blue = num,
                    &_ => panic!("fish"),
                }
            }
            let round = Round {
                red: red,
                green: green,
                blue: blue,
            };
            game.rounds.push(round);
        }
        games.push(game)
    }
    return games;
}

fn get_possible_games(games: Vec<Game>) -> Vec<Game> {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    let mut possible_games: Vec<Game> = vec![];
    for game in games {
        let mut valid = true;
        for round in &game.rounds {
            if round.red > max_red || round.green > max_green || round.blue > max_blue {
                valid = false;
            }
        }
        if valid {
            possible_games.push(game)
        }
    }
    return possible_games;
}

fn set_minimum_cubes(games: Vec<Game>) -> Vec<Game> {
    let mut minimum_games = vec![];
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in &game.rounds {
            if round.red > red {
                red = round.red
            }
            if round.green > green {
                green = round.green
            }
            if round.blue > blue {
                blue = round.blue
            }
        }

        println!("{} - r:{} g:{} b:{}", game.id, red, green, blue);
        let minimum_game = Game {
            id: game.id,
            rounds: game.rounds.clone(),
            min_red: red,
            min_green: green,
            min_blue: blue,
        };
        minimum_games.push(minimum_game);
    }

    return minimum_games;
}

fn get_power(game: Game) -> u32 {
    return game.min_red * game.min_green * game.min_blue;
}

pub fn part1(data: Vec<String>) -> String {
    let mut sum: u32 = 0;
    let possible_games = get_possible_games(process_games(data));

    for game in possible_games {
        sum += game.id
    }

    return sum.to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let games = process_games(data);
    let mut sum: u32 = 0;
    let minimum_games = set_minimum_cubes(games.clone());
    for game in minimum_games {
        sum += get_power(game);
    }

    return sum.to_string();
}
