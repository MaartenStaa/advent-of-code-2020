use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input.txt");

struct Input {
    minimum: usize,
    maximum: usize,
    character: char,
    password: String,
}

impl Input {
    pub fn is_valid(&self) -> bool {
        let times = self
            .password
            .chars()
            .filter(|c| c == &self.character)
            .count();

        times >= self.minimum && times <= self.maximum
    }
}

fn main() {
    println!(
        "Valid passwords: {}",
        INPUT
            .lines()
            .filter_map(|line| parse_line(line).ok())
            .filter(|input| input.is_valid())
            .count()
    );
}

fn parse_line(line: &str) -> Result<Input, ParseIntError> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<minimum>\d+)-(?P<maximum>\d+) (?P<character>\w): (?P<password>\w+)")
                .unwrap();
    }

    let captures = RE.captures(line).unwrap();

    Ok(Input {
        character: captures["character"].chars().next().unwrap(),
        minimum: captures["minimum"].parse()?,
        maximum: captures["maximum"].parse()?,
        password: captures["password"].to_owned(),
    })
}
