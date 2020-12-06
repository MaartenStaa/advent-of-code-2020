use std::num::ParseIntError;

const INPUT: &str = include_str!("../input.txt");
const EXPECTED: u32 = 2020;

fn main() -> Result<(), ParseIntError> {
    let lines: Vec<_> = INPUT.lines().collect();
    'outer: for (index, a) in lines.iter().enumerate() {
        let a: u32 = a.parse()?;
        for b in lines.iter().skip(index + 1) {
            let b: u32 = b.parse()?;
            for c in lines.iter().skip(index + 2) {
                let c: u32 = c.parse()?;
                if a + b + c == EXPECTED {
                    println!("{} + {} + {} = {}", a, b, c, EXPECTED);
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);

                    break 'outer;
                }
            }
        }
    }

    Ok(())
}
