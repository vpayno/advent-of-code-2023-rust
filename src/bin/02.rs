advent_of_code::solution!(2);

use regex::Regex;

use std::cmp;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut cube_limits = HashMap::<&str, u32>::new();
    cube_limits.insert("red", 12);
    cube_limits.insert("green", 13);
    cube_limits.insert("blue", 14);

    let mut sum: u32 = 0;
    let mut invalid: bool;

    for line in input.lines() {
        invalid = false;

        // game_id(Game 1) : rolls(3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green)
        let Some((game_id, rolls)) = line.split_once(':') else {
            panic!("ERROR: couldn't split string on : for line [{}]", line);
        };

        let Ok(re_rolls) = Regex::new(r"\b(\d+ (blue|red|green))\b") else {
            panic!("ERROR: bad regex for second number");
        };

        for capture in re_rolls.find_iter(rolls) {
            let roll = capture.as_str();
            // println!("roll: {}", roll);

            let Some((quantity, color)) = roll.split_once(' ') else {
                panic!("ERROR: Couldn't separate the count and color in [{}]", roll);
            };

            let count = match quantity.parse::<i32>() {
                Ok(n) => match u32::try_from(n) {
                    Ok(n) => n,
                    Err(_e) => panic!("ERROR: couldn't cube quantity from i32 to u32"),
                },
                Err(_e) => panic!("ERROR: couldn't decode cube quantity from {}", quantity),
            };

            let Some(limit) = cube_limits.get(&color) else {
                panic!(
                    "ERROR: couldn't find color {} in cube_limits hashmap",
                    color
                );
            };

            if limit < &count {
                invalid = true;
            }
        }

        if invalid {
            continue;
        }

        // only need the id number if all the rolls were valid

        let Some((_, id)) = game_id.split_once(' ') else {
            panic!("ERROR: could not extract the game id from [{}]", game_id);
        };

        sum += match id.parse::<i32>() {
            Ok(n) => match u32::try_from(n) {
                Ok(n) => n,
                Err(_e) => panic!("ERROR: couldn't convert game id from i32 to u32"),
            },
            Err(_e) => panic!("ERROR: couldn't decode the game id to i32"),
        };
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cube_limits = HashMap::<&str, u32>::new();
    cube_limits.insert("red", 12);
    cube_limits.insert("green", 13);
    cube_limits.insert("blue", 14);

    let mut sum: u32 = 0;

    for line in input.lines() {
        // game_id(Game 1) : rolls(3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green)
        let Some((_game_id, rolls)) = line.split_once(':') else {
            panic!("ERROR: couldn't split string on : for line [{}]", line);
        };

        let Ok(re_rolls) = Regex::new(r"\b(\d+ (blue|red|green))\b") else {
            panic!("ERROR: bad regex for second number");
        };

        let mut blue: u32 = 1;
        let mut green: u32 = 1;
        let mut red: u32 = 1;

        for capture in re_rolls.find_iter(rolls) {
            let roll = capture.as_str();
            // println!("roll: {}", roll);

            let Some((quantity, color)) = roll.split_once(' ') else {
                panic!("ERROR: Couldn't separate the count and color in [{}]", roll);
            };

            let count = match quantity.parse::<i32>() {
                Ok(n) => match u32::try_from(n) {
                    Ok(n) => n,
                    Err(_e) => panic!("ERROR: couldn't cube quantity from i32 to u32"),
                },
                Err(_e) => panic!("ERROR: couldn't decode cube quantity from {}", quantity),
            };

            match color {
                "blue" => blue = cmp::max(blue, count),
                "green" => green = cmp::max(green, count),
                "red" => red = cmp::max(red, count),
                _ => panic!("ERROR: I should have used an enum?"),
            };
        }

        // println!("blue: {}, green: {}, red: {}", blue, green, red);
        sum += blue * green * red;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_part_one() {
        let Some(result) = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        )) else {
            panic!("ERROR: no value returned");
        };
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day02_part_two() {
        let Some(result) = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        )) else {
            panic!("ERROR: no value returned");
        };
        assert_eq!(result, 2286);
    }
}
