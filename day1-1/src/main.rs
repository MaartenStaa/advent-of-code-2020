use std::num::ParseIntError;

const INPUT: &str = include_str!("../input.txt");
const EXPECTED: u32 = 2020;

fn main() -> Result<(), ParseIntError> {
    let lines: Vec<_> = INPUT.lines().collect();
    'outer: for (index, a) in lines.iter().enumerate() {
        let a: u32 = a.parse()?;
        for b in lines.iter().skip(index + 1) {
            let b: u32 = b.parse()?;
            if a + b == EXPECTED {
                println!("{} + {} = {}", a, b, EXPECTED);
                println!("{} * {} = {}", a, b, a * b);

                break 'outer;
            }
        }
    }

    Ok(())
}
