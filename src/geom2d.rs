use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Sub};
use Dir::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
impl Dir {
    pub(crate) fn turn_right(&self) -> Dir {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub(crate) fn turn_left(&self) -> Dir {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub(crate) fn turn_around(&self) -> Dir {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        if let Some(c) = value.chars().next() {
            c.into()
        } else {
            panic!("Can't convert an empty string to a Dir?!")
        }
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'U' | '^' => North,
            'R' | '>' => East,
            'D' | 'v' => South,
            'L' | '<' => West,
            c => panic!("Can't interpret '{c}' as a direction?!"),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            North => f.write_char('^'),
            East => f.write_char('>'),
            South => f.write_char('v'),
            West => f.write_char('<'),
        }
    }
}

pub fn step<T>(p: (T, T), d: Dir) -> (T, T)
where
    T: Add<Output = T> + Copy + From<u8> + Sub<Output = T>,
{
    step_by(p, d, From::from(1))
}

#[rustfmt::skip]
pub fn step_by<T>(p: (T, T), d: Dir, n: T) -> (T, T)
where
    T: Add<Output = T> + Copy + Sub<Output = T>,
{
    let (x, y) = p;
    match d {
        North => (x    , y - n),
        East  => (x + n, y    ),
        South => (x    , y + n),
        West  => (x - n, y    ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(North, "U".into());
        assert_eq!(East, "R".into());
        assert_eq!(South, "D".into());
        assert_eq!(West, "L".into());
    }

    #[test]
    fn turns() {
        assert_eq!(East, North.turn_right());
        assert_eq!(South, East.turn_right());
        assert_eq!(West, South.turn_right());
        assert_eq!(North, West.turn_right());
        assert_eq!(West, North.turn_left());
        assert_eq!(North, East.turn_left());
        assert_eq!(East, South.turn_left());
        assert_eq!(South, West.turn_left());
        assert_eq!(South, North.turn_around());
        assert_eq!(West, East.turn_around());
        assert_eq!(North, South.turn_around());
        assert_eq!(East, West.turn_around());
    }
}
