use Dir::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
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
            'U' => North,
            'R' => East,
            'D' => South,
            'L' => West,
            c => panic!("Can't interpret '{c}' as a direction?!"),
        }
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
}
