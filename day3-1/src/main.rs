const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let map: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    let mut x = 0;
    let mut y = 0;

    let mut trees_seen = 0;

    loop {
        x += 3;
        y += 1;

        if y >= height {
            break;
        }
        if x >= width {
            x -= width;
        }

        if map[y][x] {
            trees_seen += 1;
        }
    }

    println!("{}", trees_seen);
}
