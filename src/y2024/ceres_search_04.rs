use crate::Part;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
    // tx.send(Part::Other(part_two(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == 'X' {
                let can_n = r >= 3;
                let can_s = r < height - 3;
                let can_w = c >= 3;
                let can_e = c < width - 3;
                if can_n {
                    if grid[r - 1][c] == 'M' && grid[r - 2][c] == 'A' && grid[r - 3][c] == 'S' {
                        count += 1;
                    }
                    if can_w
                        && grid[r - 1][c - 1] == 'M'
                        && grid[r - 2][c - 2] == 'A'
                        && grid[r - 3][c - 3] == 'S'
                    {
                        count += 1;
                    }
                    if can_e
                        && grid[r - 1][c + 1] == 'M'
                        && grid[r - 2][c + 2] == 'A'
                        && grid[r - 3][c + 3] == 'S'
                    {
                        count += 1;
                    }
                }
                if can_e && grid[r][c + 1] == 'M' && grid[r][c + 2] == 'A' && grid[r][c + 3] == 'S'
                {
                    count += 1;
                }
                if can_s {
                    if grid[r + 1][c] == 'M' && grid[r + 2][c] == 'A' && grid[r + 3][c] == 'S' {
                        count += 1;
                    }
                    if can_w
                        && grid[r + 1][c - 1] == 'M'
                        && grid[r + 2][c - 2] == 'A'
                        && grid[r + 3][c - 3] == 'S'
                    {
                        count += 1;
                    }
                    if can_e
                        && grid[r + 1][c + 1] == 'M'
                        && grid[r + 2][c + 2] == 'A'
                        && grid[r + 3][c + 3] == 'S'
                    {
                        count += 1;
                    }
                }
                if can_w && grid[r][c - 1] == 'M' && grid[r][c - 2] == 'A' && grid[r][c - 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    count
}

// fn part_two(input: &str) -> usize {
//     99999
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn example_1() {
        assert_eq!(r"18", part_one(EXAMPLE_1).to_string());
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 4, do_solve).unwrap();
    }
}
