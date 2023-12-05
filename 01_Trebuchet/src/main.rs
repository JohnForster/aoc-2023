use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let total = calculate_for_iterator(lines);
        println!("total: {:?}", total);
    }
}

fn calculate_for_iterator<T, U>(iter: T) -> u32
where
    T: Iterator<Item = Result<String, U>>,
{
    let mut total = 0;
    for line in iter {
        if let Ok(line) = line {
            let line_total = calculate_for_line(&line);
            println!("{} + {} <= {}", total, line_total, line);
            total = total + line_total;
        }
    }
    return total;
}

fn calculate_for_line(line: &str) -> u32 {
    let forwards_re =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();
    let backwards_re =
        Regex::new(r"(9|8|7|6|5|4|3|2|1|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();

    let first = forwards_re.find(line);
    let line_backwards = line.chars().rev().collect::<String>();
    let last = backwards_re.find(line_backwards.as_str());

    return match first {
        None => 0,
        Some(first) => match last {
            None => {
                let first_str = normalize_num_strs(first.as_str());
                u32::from_str_radix(&format!("{}{}", first_str, first_str), 10).unwrap()
            }
            Some(last) => {
                let last = reverse_str(last.as_str());
                let first = normalize_num_strs(first.as_str());
                let last = normalize_num_strs(last.as_str());
                u32::from_str_radix(&format!("{}{}", first, last), 10).unwrap()
            }
        },
    };
}

fn reverse_str(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

fn normalize_num_strs(str: &str) -> &str {
    match str {
        // "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

#[cfg(test)]
mod tests {
    use crate::{calculate_for_iterator, calculate_for_line};

    #[test]
    fn test_single_string() {
        assert_eq!(calculate_for_line("1abc2"), 12);
        assert_eq!(calculate_for_line("pqr3stu8vwx"), 38);
        assert_eq!(calculate_for_line("a1b2c3d4e5f"), 15);
        assert_eq!(calculate_for_line("treb7uchet"), 77);
    }

    #[test]
    fn test_with_spelling() {
        assert_eq!(calculate_for_line("two1nine"), 29);
        assert_eq!(calculate_for_line("eightwothree"), 83);
        assert_eq!(calculate_for_line("abcone2threexyz"), 13);
        assert_eq!(calculate_for_line("xtwone3four"), 24);
        assert_eq!(calculate_for_line("4nineeightseven2"), 42);
        assert_eq!(calculate_for_line("zoneight234"), 14);
        assert_eq!(calculate_for_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_annoying_combos() {
        assert_eq!(calculate_for_line("1oneight"), 18);
        assert_eq!(calculate_for_line("fiveight"), 58);
        assert_eq!(calculate_for_line("eightwo"), 82);
    }

    struct DummyErr;

    #[test]
    fn test_multiple_lines() {
        let iter = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            .split("\n")
            .map(|str| Ok::<String, DummyErr>(str.trim().to_string()));

        assert_eq!(calculate_for_iterator(iter), 281);
    }
}
