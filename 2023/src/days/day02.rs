use regex::Regex;

pub fn first(input: Option<&str>) -> i32 {
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let cubes: [i32; 3] = [12, 13, 14];
    let mut sum_of_possible_ids: i32 = 0;

    let games = input.unwrap_or_else(|| include_str!("../../inputs/2.txt"));

    games.lines().into_iter().for_each(|line| {
        let game: Vec<&str> = line.trim().split(":").collect(); // ID: game[0], sets: game[1]
        let game_id: i32 = game[0].split(" ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let sets: Vec<&str> = game[1].split(";").collect();
        let mut possible = true;

        for set in sets.into_iter() {
            let red = red_regex.captures(set);
            if red.is_some()
                && red
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
                    > cubes[0]
            {
                possible = false;
            }

            let green = green_regex.captures(set);
            if green.is_some()
                && green
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
                    > cubes[1]
            {
                possible = false;
            }

            let blue = blue_regex.captures(set);
            if blue.is_some()
                && blue
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
                    > cubes[2]
            {
                possible = false;
            }
        }

        if possible {
            sum_of_possible_ids += game_id;
        }
    });

    sum_of_possible_ids
}

mod tests {
    #[test]
    fn first() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(super::first(Some(&input)), 8);
    }
}
