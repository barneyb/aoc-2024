use aoc::y2016::a_maze_of_twisty_little_cubicles_13::part_one;
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2016, 13, |input, tx| {
        tx.send(Part::A(part_one(input).to_string())).unwrap();
        // tx.send(Part::B(aoc::y2016::a_maze_of_twisty_little_cubicles_13::part_two(input).to_string())).unwrap();
    })
}
