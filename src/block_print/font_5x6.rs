use crate::block_print::Font;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref BLOCK_GLYPHS: HashMap<&'static str, char> = HashMap::from([
        // A
        ("
###..
#..#.
###..
#..#.
#..#.
###..", 'B'),
        ("
.##..
#..#.
#....
#....
#..#.
.##..", 'C'),
        // D
        ("
####.
#....
###..
#....
#....
####.", 'E'),
        ("
####.
#....
###..
#....
#....
#....", 'F'),
        ("
.##..
#..#.
#....
#.##.
#..#.
.###.", 'G'),
        ("
#..#.
#..#.
####.
#..#.
#..#.
#..#.", 'H'),
        // I
        ("
..##.
...#.
...#.
...#.
#..#.
.##..", 'J'),
        ("
#..#.
#.#..
##...
#.#..
#.#..
#..#.", 'K'),
        ("
#....
#....
#....
#....
#....
####.", 'L'),
        // M
        // N
        ("
.##..
#..#.
#..#.
#..#.
#..#.
.##..", 'O'),
        ("
###..
#..#.
#..#.
###..
#....
#....", 'P'),
        // Q
        ("
###..
#..#.
#..#.
###..
#.#..
#..#.", 'R'),
        ("
.###.
#....
#....
.##..
...#.
###..", 'S'),
        // T
        ("
#..#.
#..#.
#..#.
#..#.
#..#.
.##..", 'U'),
        // V
        // W
        // X
        ("
#...#
#...#
.#.#.
..#..
..#..
..#..", 'Y'),
        ("
####.
...#.
..#..
.#...
#....
####.", 'Z'),
   ]);
}

pub fn get_font() -> super::Font {
    Font {
        width: Some(5),
        kerned: false,
        height: 6,
        glyphs: &*BLOCK_GLYPHS,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block_print::test::assert_well_formed_glyphs;

    #[test]
    fn glyph_layouts_5x6() {
        assert_well_formed_glyphs(&*BLOCK_GLYPHS, 6, 5);
    }
}
