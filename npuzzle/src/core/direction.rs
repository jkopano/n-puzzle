use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        };
        write!(f, "{}", ch)
    }
}
impl Dir {
    pub fn values() -> [Dir; 4] {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }

    pub fn reverse(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Dir::Up),
            'D' => Some(Dir::Down),
            'L' => Some(Dir::Left),
            'R' => Some(Dir::Right),
            _ => None,
        }
    }

    pub fn to_dirs(order: &str) -> Option<[Dir; 4]> {
        let mut dirs = Vec::new();

        for ch in order.chars() {
            match ch {
                'R' => dirs.push(Dir::Right),
                'D' => dirs.push(Dir::Down),
                'U' => dirs.push(Dir::Up),
                'L' => dirs.push(Dir::Left),
                _ => return None, // Niepoprawny znak
            }
        }

        if dirs.len() == 4 {
            Some([dirs[0], dirs[1], dirs[2], dirs[3]])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod dir_tests {
    use super::Dir;
    // use crate::vector::Vector;

    #[test]
    fn test_values() {
        let values = Dir::values();
        assert_eq!(values.len(), 4);
        assert!(values.contains(&Dir::Down));
        assert!(values.contains(&Dir::Up));
        assert!(values.contains(&Dir::Left));
        assert!(values.contains(&Dir::Right));
    }
}
