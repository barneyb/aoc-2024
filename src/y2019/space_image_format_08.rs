use crate::Part;
use std::sync::mpsc::Sender;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    tx.send(Part::A(part_one(input).to_string())).unwrap();
}

fn part_one(input: &str) -> usize {
    // blindly assume single-byte characters
    if let Some(((_, o, t), _)) = input
        .as_bytes()
        .chunks(WIDTH * HEIGHT)
        .map(|bs| String::from_utf8(bs.into()).unwrap())
        .map(|s| {
            let zc = s.chars().fold((0, 0, 0), |(z, o, t), c| match c {
                '0' => (z + 1, o, t),
                '1' => (z, o + 1, t),
                '2' => (z, o, t + 1),
                _ => (z, o, t),
            });
            (zc, s)
        })
        .min()
    {
        return o * t;
    }
    panic!("no layers?!")
}

// fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_real_input() {
        crate::with_input(2019, 8, do_solve).unwrap();
    }
}
