advent_of_code::solution!(1);

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // loop lines
    // use regex to capture first digit from the left
    // store number*10 in sum
    // use regex to capture first digit from the right
    // add number to sum
    //
    // return sum

    let mut sum: u32 = 0;

    for line in input.lines() {
        let Ok(re_first) = Regex::new(r"(?m)^[^0-9]*([0-9]).*$") else {
            panic!("ERROR: bad regex for first number");
        };

        let Ok(re_last) = Regex::new(r"(?m)^.*([0-9])[^0-9]*$") else {
            panic!("ERROR: bad regex for second number");
        };

        let Some(cap_first) = re_first.captures(line) else {
            panic!("ERROR: couldn't get first number");
        };

        let first: u32 = match cap_first[1].to_string().parse::<i32>() {
            Ok(n) => match u32::try_from(n) {
                Ok(n) => n * 10,
                Err(_e) => panic!("ERROR: couldn't convert first number from i32 to u32"),
            },
            Err(_e) => panic!("ERROR: couldn't decode the first number"),
        };

        let Some(cap_last) = re_last.captures(line) else {
            panic!("ERROR: couldn't get second number");
        };

        let last: u32 = match cap_last[1].to_string().parse::<i32>() {
            Ok(n) => match u32::try_from(n) {
                Ok(n) => n,
                Err(_e) => panic!("ERROR: couldn't convert last number from i32 to u32"),
            },
            Err(_e) => panic!("ERROR: couldn't decode the last number"),
        };

        sum += first + last;
    }

    Some(sum)
}

// replace word numbers to numbers from left to right
pub fn part_two_convert(input: &str) -> String {
    let mut replacements = HashMap::<&str, u32>::new();
    replacements.insert("one", 1);
    replacements.insert("two", 2);
    replacements.insert("three", 3);
    replacements.insert("four", 4);
    replacements.insert("five", 5);
    replacements.insert("six", 6);
    replacements.insert("seven", 7);
    replacements.insert("eight", 8);
    replacements.insert("nine", 9);

    let mut new_input: String = String::new();

    for line in input.lines() {
        let mut new_line: String = String::new();

        // println!();
        // println!("checking line '{}'\n", line);

        let mut i: usize = 0;
        for (_index, rune) in line.char_indices() {
            /*
            if i > index {
                continue;
            }
            */

            if i >= line.len() {
                break;
            }

            let slice = line[i..].to_string();

            // println!();
            // println!("i: {}; new: {}; slice: {}", i, new_line, slice);

            let mut words: String = String::new();
            for (word, _number) in replacements.iter().sorted_by_key(|x| x.1) {
                words += &word;
                words += &"|";
            }
            // should be "one|two|....|nine"
            words = words[0..words.len() - 1].to_string();

            // println!("trying to replace ({}) numbers in {}\n", words, slice);

            let re_str = r"^(?<wordy>".to_string() + &words + ")(?<rest>.*)$";
            let Ok(re_start) = Regex::new(&re_str) else {
                panic!("ERROR: bad regex for replaceing wordy number: {}", re_str);
            };

            let key: String = match re_start.captures(&slice) {
                Some(caps) => caps["wordy"].to_string(),
                _ => {
                    if rune.is_ascii_digit() {
                        // println!("adding rune, {}, to new line, {}", rune, new_line);
                        new_line += &rune.to_string();
                        // println!("no wordy number matches at start of slice");
                    }
                    i += 1;
                    continue;
                }
            };
            // println!("found number: {}", key);

            // let key_len = key.len();
            let Some(numeric) = replacements.get(key.as_str()) else {
                panic!("failed to get key {} from replacements hasmap", key);
            };

            new_line += &numeric.to_string();
            // i += key_len;
            i += 1;
        }

        // println!("writing new line: {}", new_line);
        new_input += &new_line;
        new_input += &'\n'.to_string();
    }

    new_input.truncate(new_input.len() - 1);
    new_input
}

pub fn part_two(input: &str) -> Option<u32> {
    // loop lines
    // convert word number to number
    //
    // use part1 to get new num
    //
    // return new sum

    let mut new_input = part_two_convert(input);
    new_input += &'\n'.to_string();

    part_one(&part_two_convert(&new_input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let Some(result) = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        )) else {
            panic!("ERROR: test day01, part1 failed");
        };
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two_convert() {
        let result = part_two_convert(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(
            result,
            r###"219
823
123
2134
49872
18234
76"###
        );
    }

    #[test]
    fn test_part_two_convert_dc() {
        let result = part_two_convert(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(
            result,
            r###"6189
8282
24377898
1
988
16459
519"###
        );
    }

    #[test]
    fn test_day01_part_two() {
        let Some(result) = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        )) else {
            panic!("ERROR: test day01, part2 failed");
        };
        assert_eq!(result, 281);
    }

    #[test]
    fn test_day01_part_two_dc() {
        let Some(result) = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        )) else {
            panic!("ERROR: test day01, part2 failed");
        };
        assert_eq!(result, 366);
    }
}
