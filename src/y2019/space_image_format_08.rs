use crate::block_print::{parse_block_letters, BLOCK};
use crate::Part;
use std::sync::mpsc::Sender;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let layers = parse(input);
    tx.send(Part::A(part_one(&layers).to_string())).unwrap();
    tx.send(Part::B(part_two(&layers))).unwrap();
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .as_bytes()
        .chunks(WIDTH * HEIGHT)
        .map(|bs| String::from_utf8(bs.into()).unwrap().chars().collect())
        .collect()
}

fn part_one(layers: &Vec<Vec<char>>) -> usize {
    // blindly assume single-byte characters
    if let Some((_, ones, twos)) = layers
        .iter()
        .map(|cs| {
            let counts = cs.iter().fold((0, 0, 0), |(z, o, t), c| match c {
                '0' => (z + 1, o, t),
                '1' => (z, o + 1, t),
                '2' => (z, o, t + 1),
                _ => (z, o, t),
            });
            counts
        })
        .min()
    {
        return ones * twos;
    }
    panic!("no layers?!")
}

fn part_two(layers: &Vec<Vec<char>>) -> String {
    let mut pixels = Vec::new();
    for i in 0..(WIDTH * HEIGHT) {
        for l in layers {
            match l[i] {
                '0' => {
                    pixels.push(' ');
                    break;
                }
                '1' => {
                    pixels.push(BLOCK);
                    break;
                }
                '2' => continue, // transparent
                _ => panic!("There are no '{}' in space?!", l[i]),
            }
        }
    }
    let mut display = String::new();
    for line in pixels.chunks(WIDTH) {
        display.extend(line.iter());
        display.push('\n')
    }
    parse_block_letters(&display).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_real_input() {
        crate::with_input(2019, 8, do_solve).unwrap();
    }
}
