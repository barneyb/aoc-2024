use crate::block_print::Font;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref BLOCK_GLYPHS: HashMap<&'static str, char> = HashMap::from([
        // ABCDEFG
        ("
#...#
#...#
#...#
#####
#...#
#...#
#...#
#...#", 'H'
        ),
        ("
###
.#.
.#.
.#.
.#.
.#.
.#.
###", 'I'
        ),
        // JKLMNOPQRSTUVWXYZ
    ]);
}

pub fn get_font() -> super::Font {
    Font {
        width: None,
        kerned: true,
        height: 8,
        glyphs: &*BLOCK_GLYPHS,
    }
}
