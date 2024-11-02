pub fn part_one(input: &str) -> usize {
    input.len()
}

// pub fn part_two(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;
    
    #[test]
    fn example_1() {
        assert_eq!(r"42", part_one(EXAMPLE_1).to_string());
    }
    

    // #[test]
    // fn test_real_input() {
    //     use crate::{with_input, Part};
    //     with_input(2016, 12, |input, tx| {
    //         tx.send(Part::A(Box::new(part_one(input)))).unwrap();
    //         // tx.send(Part::B(Box::new(part_two(input)))).unwrap();
    //     })
    //     .unwrap();
    // }
}
