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
