use aoc::y2015::not_quite_lisp_01::{part_one, part_two};
use aoc::{with_input, Part};
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2015, 1, |input, tx| {
        tx.send(Part::A(Box::new(part_one(input)))).unwrap();
        tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    })
}
