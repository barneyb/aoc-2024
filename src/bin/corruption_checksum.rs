use aoc::y2017::corruption_checksum_02::{part_one, part_two};
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2017, 2, |input, tx| {
        tx.send(Part::A(part_one(input).to_string())).unwrap();
        tx.send(Part::B(part_two(input).to_string())).unwrap();
    })
}
