use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input.txt");

struct Input {
    position_1: usize,
    position_2: usize,
    character: char,
    password: String,
}

impl Input {
    pub fn is_valid(&self) -> bool {
        let mut chars = self.password.chars();

        let first = chars.nth(self.position_1 - 1).unwrap() == self.character;
        let second = chars.nth(self.position_2 - self.position_1 - 1).unwrap() == self.character;

        first ^ second
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
            Regex::new(r"^(?P<pos_1>\d+)-(?P<pos_2>\d+) (?P<character>\w): (?P<password>\w+)")
                .unwrap();
    }

    let captures = RE.captures(line).unwrap();

    Ok(Input {
        character: captures["character"].chars().next().unwrap(),
        position_1: captures["pos_1"].parse()?,
        position_2: captures["pos_2"].parse()?,
        password: captures["password"].to_owned(),
    })
}
