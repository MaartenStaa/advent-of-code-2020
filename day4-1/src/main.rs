mod split;

use crate::split::SplitBy;

#[derive(Default)]
struct Passport<'a> {
    byr: Option<&'a str>,
    cid: Option<&'a str>,
    ecl: Option<&'a str>,
    eyr: Option<&'a str>,
    hcl: Option<&'a str>,
    hgt: Option<&'a str>,
    iyr: Option<&'a str>,
}

impl<'a> Passport<'a> {
    pub fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.ecl.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.hgt.is_some()
            && self.iyr.is_some()
    }
}

fn main() {
    println!("Hello, world!");
}

fn count_valid(input: &str) -> usize {
    // input.lines().fold((vec![], None), |(mut vec, mut current), line| {
    //     if line.is_empty() {
    //         vec.push(current);
    //         return (vec, current);
    //     }
    //
    //     let current = current.
    // })
    input.lines().split_by(|line| line.is_empty())
}

#[cfg(test)]
mod tests {
    use crate::{count_valid, Passport};

    static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_valid() {
        assert!(Passport {
            byr: Some(""),
            cid: Some(""),
            ecl: Some(""),
            eyr: Some(""),
            hcl: Some(""),
            hgt: Some(""),
            iyr: Some(""),
        }
        .is_valid());

        assert!(Passport {
            byr: Some(""),
            cid: None,
            ecl: Some(""),
            eyr: Some(""),
            hcl: Some(""),
            hgt: Some(""),
            iyr: Some(""),
        }
        .is_valid());
    }
    #[test]
    fn test_invalid() {
        assert!(!Passport {
            byr: None,
            cid: Some(""),
            ecl: Some(""),
            eyr: Some(""),
            hcl: Some(""),
            hgt: Some(""),
            iyr: Some(""),
        }
        .is_valid());
    }

    #[test]
    fn test_count_valid() {
        assert_eq!(count_valid(TEST_INPUT), 2);
    }
}
