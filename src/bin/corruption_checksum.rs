use aoc::y2017::corruption_checksum_02::{part_one, part_two};
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2017, 2, |input, tx| {
        tx.send(Part::A(Box::new(part_one(input)))).unwrap();
        tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    })
}
