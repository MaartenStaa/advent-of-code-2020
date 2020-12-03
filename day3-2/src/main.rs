const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let map: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    let total_trees: u64 = vec![1, 1, 3, 1, 5, 1, 7, 1, 1, 2]
        .chunks(2)
        .map(|parts| (parts[0], parts[1]))
        .map(|(x_offset, y_offset)| {
            dbg!(x_offset, y_offset);
            let mut x = 0;
            let mut y = 0;

            let mut trees_seen = 0;

            loop {
                x += x_offset;
                y += y_offset;

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

            trees_seen
        })
        .product();

    println!("{}", total_trees);
}
