use std::cmp::{max, min};

pub fn first(input: Option<&str>) -> i32 {
    let lines: Vec<Vec<char>> = input
        .unwrap_or_else(|| include_str!("../../inputs/3.txt"))
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut sum = 0;

    for i in 0..lines.len() {
        let mut number: String = String::new();
        let mut is_part_number: bool = false;
        for j in 0..lines[0].len() {
            if lines[i][j].is_digit(10) {
                number.push_str(&lines[i][j].to_string());
                if !is_part_number && seek_for_symbols(&lines, i as i32, j as i32) {
                    is_part_number = true;
                }
                if is_part_number && j == lines[0].len() - 1 {
                    sum += number.parse::<i32>().unwrap();
                    number = String::new();
                    is_part_number = false;
                }
                continue;
            } else if is_part_number {
                sum += number.parse::<i32>().unwrap();
            }
            number = String::new();
            is_part_number = false;
        }
    }
    sum
}

fn seek_for_symbols(lines: &Vec<Vec<char>>, row_id: i32, col_id: i32) -> bool {
    for i in max(0, row_id - 1)..min(row_id + 2, lines.len() as i32) {
        for j in max(0, col_id - 1)..min(col_id + 2, lines[0].len() as i32) {
            if lines[i as usize][j as usize] != '.' && !lines[i as usize][j as usize].is_digit(10) {
                return true;
            }
        }
    }
    false
}

mod tests {
    #[test]
    fn first() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ..........
        ........+1";
        assert_eq!(super::first(Some(&input)), 4362);
    }
}
