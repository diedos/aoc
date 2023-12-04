use std::io::{self, BufRead};

pub fn first() {
    let mut result = 0;

    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let mut iter = line_str.chars();
        let mut line_number: String = "".to_string();

        loop {
            if let Some(c) = iter.next() {
                if c.is_digit(10) {
                    line_number.push(c);
                    break;
                }
            } else {
                break;
            }
        }

        loop {
            if let Some(c) = iter.next_back() {
                if c.is_digit(10) {
                    line_number.push(c);
                    break;
                }
            } else {
                break;
            }
        }

        if line_number.len() == 1 {
            line_number.push(line_number.chars().nth(0).unwrap());
        }

        result += line_number.parse::<u32>().unwrap_or(0);
    }

    println!("{}", result);
}

pub fn second() {
    let mut result = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let mut iter = line_str.chars();
        let mut line_number: String = String::new();

        let mut word: String = String::new();

        'outer: loop {
            if let Some(c) = iter.next() {
                if c.is_digit(10) {
                    line_number.push(c);
                    break;
                }

                word.push(c);

                for (i, n) in numbers.iter().enumerate() {
                    if word.contains(n) {
                        line_number.push_str(&(i + 1).to_string());
                        word.clear();
                        break 'outer;
                    }
                }
            } else {
                break;
            }
        }

        'outer: loop {
            if let Some(c) = iter.next_back() {
                if c.is_digit(10) {
                    line_number.push(c);
                    break;
                }

                word = c.to_string() + &word;

                for (i, n) in numbers.iter().enumerate() {
                    if word.contains(n) {
                        line_number.push_str(&(i + 1).to_string());
                        word.clear();
                        break 'outer;
                    }
                }
            } else {
                break;
            }
        }

        if line_number.len() == 1 {
            line_number.push(line_number.chars().nth(0).unwrap());
        }

        result += line_number.parse::<u32>().unwrap_or(0);
    }

    println!("{}", result);
}
