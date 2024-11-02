use aoc::y2016::leonardo_s_monorail_12::part_one;
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2016, 12, |input, tx| {
        tx.send(Part::A(Box::new(part_one(input)))).unwrap();
        // tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    })
}
