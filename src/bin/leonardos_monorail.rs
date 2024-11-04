use aoc::y2016::leonardos_monorail_12::{part_one, part_two};
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2016, 12, |input, tx| {
        tx.send(Part::A(part_one(input).to_string())).unwrap();
        tx.send(Part::B(part_two(input).to_string())).unwrap();
    })
}
