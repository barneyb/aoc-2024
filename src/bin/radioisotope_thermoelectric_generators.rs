use aoc::with_input;
use aoc::y2016::radioisotope_thermoelectric_generators_11::do_solve;
use std::io::Error;

fn main() -> Result<(), Error> {
    with_input(2016, 11, do_solve)
}
