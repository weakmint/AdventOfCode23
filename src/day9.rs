fn extrapolate(data: Vec<String>) -> (i64, i64) {
    fn get_differences(history: &Vec<i64>, values: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut next = vec![];
        for i in 0..history.len() - 1 {
            let difference = history[i] - history[i + 1];
            next.push(difference * -1);
        }
        values.push(next.clone());
        if !next.clone().iter().all(|x| x == &0) {
            get_differences(&next, values);
        }
        return values.to_vec();
    }
    let differences = data
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|x| get_differences(&x, &mut [x.clone()].to_vec()));

    let forward = differences.clone().fold(0, |acc, x| {
        acc + x
            .clone()
            .into_iter()
            .fold(0, |acc2, y| acc2 + y.last().unwrap())
    });
    let reverse: i64 = differences.clone().into_iter().rev().fold(0, |acc, x| {
        return acc
            + x.clone().into_iter().rev().fold(0, |acc2, y| {
                println!("{:?}", y);
                return y[0] - acc2;
            });
    });

    return (forward, reverse);
}

pub fn part1(data: Vec<String>) -> String {
    let (extrapolated, _) = extrapolate(data);
    return extrapolated.to_string();
}

pub fn part2(data: Vec<String>) -> String {
    let (_, rev_extrapolated) = extrapolate(data);
    return rev_extrapolated.to_string();
}
