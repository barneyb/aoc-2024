use lazy_static::lazy_static;
use std::collections::HashMap;

const BLOCK: char = '█';
const WIDTH: usize = 5;
const HEIGHT: usize = 6;

lazy_static! {
    static ref BLOCK_GLYPHS: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        // A
        m.insert("███  \n█  █ \n███  \n█  █ \n█  █ \n███  \n", 'B');
        m.insert(" ██  \n█  █ \n█    \n█    \n█  █ \n ██  \n", 'C');
        // D
        m.insert("████ \n█    \n███  \n█    \n█    \n████ \n", 'E');
        m.insert("████ \n█    \n███  \n█    \n█    \n█    \n", 'F');
        m.insert(" ██  \n█  █ \n█    \n█ ██ \n█  █ \n ███ \n", 'G');
        m.insert("█  █ \n█  █ \n████ \n█  █ \n█  █ \n█  █ \n", 'H');
        // I
        m.insert("  ██ \n   █ \n   █ \n   █ \n█  █ \n ██  \n", 'J');
        m.insert("█  █ \n█ █  \n██   \n█ █  \n█ █  \n█  █ \n", 'K');
        m.insert("█    \n█    \n█    \n█    \n█    \n████ \n", 'L');
        // M
        // N
        m.insert(" ██  \n█  █ \n█  █ \n█  █ \n█  █ \n ██  \n", 'O');
        m.insert("███  \n█  █ \n█  █ \n███  \n█    \n█    \n", 'P');
        // Q
        m.insert("███  \n█  █ \n█  █ \n███  \n█ █  \n█  █ \n", 'R');
        m.insert(" ███ \n█    \n█    \n ██  \n   █ \n███  \n", 'S');
        // T
        m.insert("█  █ \n█  █ \n█  █ \n█  █ \n█  █ \n ██  \n", 'U');
        // V
        // W
        // X
        m.insert("█   █\n█   █\n █ █ \n  █  \n  █  \n  █  \n", 'Y');
        m.insert("████ \n   █ \n  █  \n █   \n█    \n████ \n", 'Z');
        let char_len = WIDTH * HEIGHT + HEIGHT;
        for (s,c) in m.iter() {
            let cl =s.chars().count();
            if cl != char_len {
                panic!("Char '{c}' is malformed: len {cl} not len {char_len}")
            }
        }
        m
    };
}

/// Parse block printing (a la 2016/08 Two-Factor Authentication) and turn it
/// into an equivalent string of uppercase ASCII letters. All block printed
/// strings are six rows 'tall', and fixed-width at five columns.
///
/// Spaces and periods are considered "blank"; all other glyphs are considered
/// "marked". Leading and trailing newlines will be trimmed. The final trailing
/// column of the last character **MUST** be present, so that it is the full
/// five columns wide. See the example below.
///
/// The full AoC font is not yet known, just enough to get Barney's answers. No
/// guessing is performed; exact matched only. If parsing fails, an `Err` will
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
            '.' => ' ',
            ' ' | '\n' | '\r' => c,
            _ => BLOCK,
        })
        .collect::<String>();
    let lines = sanitized.lines().collect::<Vec<_>>();
    if lines.len() != 6 {
        eprintln!("Block printed letters with height {}, not 6.", lines.len());
        eprintln!("{display}");
        return Err("Block printed letters are always exactly six lines tall.");
    }
    let mut line_len = None;
    for (i, l) in lines.iter().enumerate() {
        let len = l.chars().count();
        if let Some(ll) = line_len {
            if ll != len {
                eprintln!(
                    "Block printed line {i} has {len} characters, not {ll}.\n{}",
                    display.replace(' ', ".")
                );
                return Err("Block printed lines are always the same width.");
            }
        } else if len % 5 != 0 {
            eprintln!(
                "Block printed line {i} has {len} characters, which isn't divisible by 5.\n{}",
                display.replace(' ', ".")
            );
            return Err("Block printed letters are always exactly five chars wide.");
        } else {
            line_len = Some(len)
        }
    }
    let mut result = String::new();
    if let Some(line_len) = line_len {
        let mut buffer = String::new();
        for i in (0..line_len).step_by(WIDTH) {
            buffer.clear();
            for l in lines.iter() {
                buffer.extend(l.chars().skip(i).take(WIDTH));
                buffer.push('\n');
            }
            // I remain confused why a &String doesn't coerce to &str, and requires
            // an explicit [..] to slice it.
            if let Some(&c) = BLOCK_GLYPHS.get(&buffer[..]) {
                result.push(c)
            } else {
                eprintln!(
                    "Block printed character wasn't recognized.\n[{}]",
                    buffer.replace(' ', ".")
                );
                return Err("Failed to identify block-printed glyph.");
            }
        }
        Ok(result)
    } else {
        Err("Unknown error.")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cockleburs() {
        let display = r"
 ██   ██   ██  █  █ █    ████ ███  █  █ ███   ███.
█  █ █  █ █  █ █ █  █    X    █  █ █  █ █  █ █    
█    █  █ █    ██   █    ###  ███  █  █ █  █ █    
█    █  █ █    █ █  █    █    █  █ █  █ ███   ██  
█  █ █  █ █  █ █ █  █    █    █  █ █  █ █ █     █ 
 ██   ██   ██  █  █ ████ ████ ███   ██  █  █ ███..
";
        assert_eq!(Ok("COCKLEBURS".to_string()), parse_block_letters(display))
    }

    #[test]
    fn wrong_line_count() {
        // one of the "middle" lines is missing, so only five tall
        let display = " ██  \n█  █ \n█    \n█  █ \n ██  ";
        assert!(parse_block_letters(display).is_err())
    }

    #[test]
    fn missing_trailing_spaces_first() {
        // first line is missing a trailing space
        let display = " ██ \n█  █ \n█    \n█    \n█  █ \n ██  ";
        assert!(parse_block_letters(display).is_err())
    }

    #[test]
    fn missing_trailing_spaces_subsequent() {
        // second line is missing the final space
        let display = " ██  \n█  █\n█    \n█    \n█  █ \n ██  ";
        assert!(parse_block_letters(display).is_err())
    }
}
