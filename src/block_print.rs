use std::collections::HashMap;

/// Unicode U+2588 Full Block, for prettier - but non-ASCII - block printing.
pub const BLOCK: char = '█';

mod font_5x6;
mod font_6x10;
mod font_x8;

type Glyphs = HashMap<&'static str, char>;

pub struct Font {
    width: Option<usize>,
    kerned: bool,
    #[allow(dead_code)]
    height: usize,
    glyphs: &'static Glyphs,
}

impl Font {
    fn is_fixed_width(&self) -> bool {
        self.width.is_some()
    }

    fn is_kerned(&self) -> bool {
        self.kerned
    }

    fn recognize(&self, lines: &[&[char]]) -> Result<String, &'static str> {
        let mut result = String::new();
        let mut buffer = String::new();
        let line_len = lines[0].len();
        let mut i = 0;
        while i < line_len {
            buffer.clear();
            if self.is_fixed_width() {
                self.read_fixed_width(lines, &mut i, self.width.unwrap(), &mut buffer);
            } else {
                self.read_variable_width(lines, &mut i, &mut buffer);
            }
            // I remain confused why a &String doesn't coerce to &str, and requires
            // an explicit [..] to slice it.
            if let Some(&c) = self.glyphs.get(&buffer[..]) {
                result.push(c)
            } else {
                eprintln!("Block printed character wasn't recognized.\n[{buffer}]",);
                return Err("Failed to identify block-printed glyph.");
            }
            if self.is_kerned() {
                self.soak_up_spaces(lines, &mut i)
            }
        }
        Ok(result)
    }

    fn is_space(&self, lines: &[&[char]], i: usize) -> bool {
        lines.iter().all(|l| l[i] == '.')
    }

    fn soak_up_spaces(&self, lines: &[&[char]], i: &mut usize) {
        let len = lines[0].len();
        while *i < len && self.is_space(lines, *i) {
            *i += 1;
        }
    }

    fn read_variable_width(&self, lines: &[&[char]], i: &mut usize, buffer: &mut String) {
        let len = lines[0].len();
        let mut end = *i + 1;
        while end < len && !self.is_space(lines, end) {
            end += 1;
        }
        self.read_fixed_width(lines, i, end - *i, buffer)
    }

    fn read_fixed_width(
        &self,
        lines: &[&[char]],
        i: &mut usize,
        width: usize,
        buffer: &mut String,
    ) {
        let pos = *i;
        *i += width;
        for l in lines.iter() {
            buffer.push('\n');
            buffer.extend(&l[pos..*i]);
        }
    }
}

/// Parse block printing (a la 2016/08 Two-Factor Authentication) and turn it
/// into an equivalent string of uppercase ASCII letters. All block printed
/// strings are of uniform height, but individual glyphs may vary in width,
/// depending on the font.
///
/// Spaces and periods are considered "blank"; all other glyphs are considered
/// "marked". Leading and trailing newlines will be trimmed. The final trailing
/// column of the last character **MUST** be present, even if it's all blanks,
/// when using fixed-width font. See the example below.
///
/// The full AoC font is not yet known, just enough to get Barney's answers. No
/// guessing is performed; exact matches only. If parsing fails, an `Err` will
/// be returned with a message about what went wrong, and some info on STDERR.
///
/// ```
/// # use aoc::block_print::parse_block_letters;
/// // note the mandatory trailing spaces, shown as periods.
/// let display = r#"
/// ABC   ██   ██..
/// B  █ #  X O  0.
/// B██  #  X O  0.
/// B  █ #  X O  0.
/// B  █ #  X O  0.
/// def   ██   ██..
/// "#;
///
/// assert_eq!(Ok("BOO".to_owned()), parse_block_letters(display));
/// ```
pub fn parse_block_letters(display: &str) -> Result<String, &str> {
    let sanitized = display
        .trim_matches(['\n', '\r'])
        .chars()
        .map(|c| match c {
            ' ' => '.',
            '.' | '\n' | '\r' => c,
            _ => '#',
        })
        .collect::<Vec<_>>();
    let lines = sanitized.split(|&c| c == '\n').collect::<Vec<_>>();
    let font = match lines.len() {
        6 => font_5x6::get_font(),
        8 => font_x8::get_font(),
        10 => font_6x10::get_font(),
        _ => {
            eprintln!(
                "Block printed letters with height {} are not supported.\n{display}",
                lines.len()
            );
            return Err("Block printed letters of unsupported height.");
        }
    };
    let mut line_len = None;
    for (i, l) in lines.iter().enumerate() {
        let len = l.len();
        if let Some(ll) = line_len {
            if ll != len {
                eprintln!(
                    "Block printed line {} has {len} characters, not {ll}.\n{display}",
                    i + 1
                );
                return Err("Block printed lines are always the same width.");
            }
        } else if font.is_fixed_width() && !font.is_kerned() {
            let w = font.width.unwrap();
            if len % w != 0 {
                eprintln!(
                    "Block printed line {i} has {len} characters, which isn't divisible by {w}.\n{display}",
                );
                return Err("Block printed letters are always exactly five chars wide.");
            }
        } else {
            line_len = Some(len)
        }
    }
    font.recognize(&lines)
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn assert_well_formed_glyphs(
        glyphs: &HashMap<&'static str, char>,
        height: usize,
        width: usize,
    ) {
        let char_len = width * height + height;
        for (s, c) in glyphs.iter() {
            let cl = s.chars().count();
            if cl != char_len {
                panic!("Glyph for '{c}' is malformed: len {cl} not len {char_len}")
            }
        }
    }

    #[test]
    fn cockleburs_5x6() {
        let display = r"
.██...██...██..█..█.█....████.███..█..█.███...███.
█..█.█..█.█..█.█.█..█....X....█..█.█..█.█..█.█....
█....█..█.█....██...█....###..███..█..█.█..█.█....
█....█..█.█....█.█..█....█....█..█.█..█.███...██..
█..█.█..█.█..█.█.█..█....█....█..█.█..█.█.█.....█.
.██...██...██..█..█.████.████.███...██..█..█.███..
";
        assert_eq!(Ok("COCKLEBURS".to_string()), parse_block_letters(display))
    }

    #[test]
    fn raz_6x10() {
        let display = r"
█████.....██....██████
█....█...█..█........█
█....█..█....█.......█
█....█..█....█......█.
█████...█....█.....█..
█..█....██████....█...
█...█...█....█...█....
█...█...█....█..█.....
█....█..█....█..█.....
█....█..█....█..██████";
        assert_eq!(Ok("RAZ".to_string()), parse_block_letters(display))
    }

    #[test]
    fn hi_x8() {
        let display = r"
█...█..███
█...█...█.
█...█...█.
█████...█.
█...█...█.
█...█...█.
█...█...█.
█...█..███";
        assert_eq!(Ok("HI".to_string()), parse_block_letters(display))
    }

    #[test]
    fn wrong_line_count() {
        // one of the "middle" lines is missing, so only five tall
        let display = "
.██..
█..█.
█....
█..█.
.██..";
        assert!(parse_block_letters(display).is_err())
    }

    #[test]
    fn missing_trailing_spaces_first() {
        // first line is missing a trailing space
        let display = "
.██.
█..█.
█....
█....
█..█.
.██..";
        assert!(parse_block_letters(display).is_err())
    }

    #[test]
    fn missing_trailing_spaces_subsequent() {
        // second line is missing the final space
        let display = "
.██..
█..█
█....
█....
█..█.
.██..";
        assert!(parse_block_letters(display).is_err())
    }
}
