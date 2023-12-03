advent_of_code::solution!(1);

use regex::Regex;

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
