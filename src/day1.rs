pub fn part2(data: Vec<String>) -> String {
    let mut sum: u32 = 0;
    for text in data {
        let chars: Vec<char> = text.chars().collect();
        let backwards_chars = text.chars().rev().collect();

        let first: u32 = get_first_int(chars);
        let last: u32 = get_first_int(backwards_chars);

        let num: u32 = first * 10 + last;
        sum += num;
    }
    return sum.to_string();
}

fn get_first_int(chars: Vec<char>) -> u32 {
    let mut word: String = "".to_string();
    for c in chars {
        word = word + &c.to_string();
        if word.contains("one") || word.contains("eno") {
            return 1;
        }
        if word.contains("two") || word.contains("owt") {
            return 2;
        }
        if word.contains("three") || word.contains("eerht") {
            return 3;
        }
        if word.contains("four") || word.contains("ruof") {
            return 4;
        }
        if word.contains("five") || word.contains("evif") {
            return 5;
        }
        if word.contains("six") || word.contains("xis") {
            return 6;
        }
        if word.contains("seven") || word.contains("neves") {
            return 7;
        }
        if word.contains("eight") || word.contains("thgie") {
            return 8;
        }
        if word.contains("nine") || word.contains("enin") {
            return 9;
        }
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    return 1;
}
