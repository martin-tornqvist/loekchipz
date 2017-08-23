use std::ops::Add;
use std::ops::Sub;

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Dir
{
    UpLeft,
    Up,
    UpRight,
    Left,
    Center,
    Right,
    DownLeft,
    Down,
    DownRight,
}

pub fn dir_to_offset(dir: Dir) -> P
{
    match dir
    {
        Dir::UpLeft => P { x: -1, y: -1 },
        Dir::Up => P { x: 0, y: -1 },
        Dir::UpRight => P { x: 1, y: -1 },
        Dir::Left => P { x: -1, y: 0 },
        Dir::Center => P { x: 0, y: 0 },
        Dir::Right => P { x: 1, y: 0 },
        Dir::DownLeft => P { x: -1, y: 1 },
        Dir::Down => P { x: 0, y: 1 },
        Dir::DownRight => P { x: 1, y: 1 },
    }
}

pub fn offset_to_dir(offset: P) -> Dir
{
    match offset
    {
        P { x: -1, y: -1 } => Dir::UpLeft,
        P { x: 0, y: -1 } => Dir::Up,
        P { x: 1, y: -1 } => Dir::UpRight,
        P { x: -1, y: 0 } => Dir::Left,
        P { x: 0, y: 0 } => Dir::Center,
        P { x: 1, y: 0 } => Dir::Right,
        P { x: -1, y: 1 } => Dir::DownLeft,
        P { x: 0, y: 1 } => Dir::Down,
        P { x: 1, y: 1 } => Dir::DownRight,        
        _ => panic!("Not a direction offset"),
    }
}

pub fn sign(value: i32) -> i32
{
    if value < 0
    {
        return -1;
    }
    else if value > 0
    {
        return 1;
    }

    return 0;
}

// -----------------------------------------------------------------------------
// Position
// -----------------------------------------------------------------------------
#[derive(Copy)]
pub struct P
{
    pub x: i32,
    pub y: i32,
}

impl P
{
    #[allow(dead_code)]
    fn offset(&self, x: i32, y: i32) -> P
    {
        P {
            x: self.x + x,
            y: self.y + y,
        }
    }

    #[allow(dead_code)]
    fn offset_p(&self, p: &Self) -> P
    {
        P {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    #[allow(dead_code)]
    fn offset_dir(&self, dir: Dir) -> P
    {
        let p = dir_to_offset(dir);

        P {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    #[allow(dead_code)]
    // NOTE: Assumes that both x and y is -1, 0, or 1
    fn dir(self) -> Dir
    {
        offset_to_dir(self)
    }

    #[allow(dead_code)]
    fn signs(&self) -> P
    {
        P {
            x: sign(self.x),
            y: sign(self.y),
        }
    }
}

impl Default for P
{
    fn default() -> P
    {
        P { x: 0, y: 0 }
    }
}

impl Clone for P
{
    fn clone(&self) -> P
    {
        P {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for P
{
    fn eq(&self, other: &P) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}

impl Add for P
{
    type Output = P;

    fn add(self, other: P) -> P
    {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for P
{
    type Output = P;

    fn sub(self, other: P) -> P
    {
        P {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p()
    {
        // Verify copy
        let p0 = P { x: 3, y: 5 };

        let p1 = p0;

        assert_eq!(p0.x, p1.x);
        assert_eq!(p0.y, p1.y);

        // Verify comparison
        assert!(p0 == p1);

        // Verify offset
        let p2 = p0.offset_p(&P { x: 100, y: 200 });

        assert_eq!(103, p2.x);
        assert_eq!(205, p2.y);

        // Verify addition
        let p3 = p0 + P { x: 200, y: 300 };

        assert_eq!(203, p3.x);
        assert_eq!(305, p3.y);

        // Verify subtraction
        let p4 = p0 - P { x: 10, y: 1 };

        assert_eq!(-7, p4.x);
        assert_eq!(4, p4.y);

        // Verify direction conversion
        let d = P { x: 1, y: 0 };

        let dir = d.dir();

        assert!(dir == Dir::Right);

        // Verify signs
        let p = P { x: -42, y: 99 };

        let signs = p.signs();

        assert_eq!(signs.x, -1);
        assert_eq!(signs.y, 1);
    }
}
