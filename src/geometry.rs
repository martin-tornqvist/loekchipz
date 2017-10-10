use std::ops::Add;
use std::ops::Sub;

// -----------------------------------------------------------------------------
// Conversions between 1d and 2d representations
// -----------------------------------------------------------------------------
#[allow(dead_code)]
pub fn vec_idx(x: i32, y: i32, width: i32) -> usize
{
    let idx = width * y + x;

    return idx as usize;
}

#[allow(dead_code)]
pub fn vec_idx_p(pos2d: P, width: i32) -> usize
{
    let idx = width * pos2d.y + pos2d.x;

    return idx as usize;
}

#[allow(dead_code)]
pub fn vec_size(dims: P) -> usize
{
    let size = dims.x * dims.y;

    return size as usize;
}

// -----------------------------------------------------------------------------
// Direction
// -----------------------------------------------------------------------------
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

// -----------------------------------------------------------------------------
// Misc utils
// -----------------------------------------------------------------------------
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
// Position/dimensions
// -----------------------------------------------------------------------------
#[derive(Copy)]
#[derive(Debug)]
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
    fn offset_p(&self, p: Self) -> P
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
// Some nice arrays to iterate over in algorithms
// -----------------------------------------------------------------------------

// Offsets sorted for optimized 2d array iteration (minimized jumping)
//
// Each index corresponds to a direction:
//
// 0 1 2
//  \|/
// 3-4-5
//  /|\
// 6 7 8
//
#[allow(dead_code)]
pub const OFFSETS: [P; 9] = [
    P { x: -1, y: -1 }, // Up left
    P { x: 0, y: -1 },  // Up
    P { x: 1, y: -1 },  // Up right
    P { x: -1, y: 0 },  // Left
    P { x: 0, y: 0 },   // Center
    P { x: 1, y: 0 },   // Right
    P { x: -1, y: 1 },  // Down left
    P { x: 0, y: 1 },   // Down
    P { x: 1, y: 1 },   // Down right
];

// Offsets prioritizing cardinal movement (gives nicer pathfinding for example),
// with no center value
//
// Each index corresponds to a direction:
//
// 4 2 5
//  \|/
// 0-X-1
//  /|\
// 6 3 7
//
#[allow(dead_code)]
pub const OFFSETS_CARDINAL_FIRST_NO_CENTER: [P; 8] = [
    P { x: -1, y: 0 },  // Left
    P { x: 1, y: 0 },   // Right
    P { x: 0, y: -1 },  // Up
    P { x: 0, y: 1 },   // Down
    P { x: -1, y: -1 }, // Up left
    P { x: 1, y: -1 },  // Up right
    P { x: -1, y: 1 },  // Down left
    P { x: 1, y: 1 },   // Down right
];

// -----------------------------------------------------------------------------
// Rectangle
// -----------------------------------------------------------------------------
#[derive(Copy)]
pub struct R
{
    pub p0: P,
    pub p1: P,
}

impl Clone for R
{
    fn clone(&self) -> R
    {
        R {
            p0: self.p0,
            p1: self.p1,
        }
    }
}

impl PartialEq for R
{
    fn eq(&self, other: &R) -> bool
    {
        self.p0 == other.p0 && self.p1 == other.p1
    }
}

impl R
{
    #[allow(dead_code)]
    pub fn is_p_inside(&self, p: P) -> bool
    {
        let x_inside = (p.x >= self.p0.x) && (p.x <= self.p1.x);
        let y_inside = (p.y >= self.p0.y) && (p.y <= self.p1.y);

        return x_inside && y_inside;
    }

    pub fn w(&self) -> i32
    {
        return (self.p1.x - self.p0.y) + 1;
    }

    pub fn h(&self) -> i32
    {
        return (self.p1.y - self.p0.y) + 1;
    }

    pub fn center(&self) -> P
    {
        P {
            x: self.p0.x + self.w() / 2,
            y: self.p0.y + self.h() / 2,
        }
    }
}

// -----------------------------------------------------------------------------
// Dynamic 2d array
// -----------------------------------------------------------------------------
pub struct A2<T>
{
    pub data: Vec<T>,
    w: i32,
    h: i32,
}

impl<T> A2<T>
{
    #[allow(dead_code)]
    pub fn w(&self) -> i32
    {
        return self.w;
    }

    #[allow(dead_code)]
    pub fn h(&self) -> i32
    {
        return self.h;
    }

    #[allow(dead_code)]
    pub fn dims(&self) -> P
    {
        return P {
            x: self.w,
            y: self.h,
        };
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize
    {
        return (self.w * self.h) as usize;
    }

    #[allow(dead_code)]
    pub fn at(&mut self, x: i32, y: i32) -> &mut T
    {
        let i = self.vec_idx(x, y);

        return &mut self.data[i];
    }

    #[allow(dead_code)]
    pub fn at_p(&mut self, p: P) -> &mut T
    {
        let mut v = self.at(p.x, p.y);

        return v;
    }

    #[allow(dead_code)]
    pub fn is_p_inside(&self, p: P) -> bool
    {
        let x_ok = (p.x >= 0) && (p.x < self.w);
        let y_ok = (p.y >= 0) && (p.y < self.h);

        return x_ok && y_ok;
    }

    #[allow(dead_code)]
    fn vec_idx(&self, x: i32, y: i32) -> usize
    {
        return vec_idx(x, y, self.w);
    }
}

impl<T: Copy> A2<T>
{
    #[allow(dead_code)]
    pub fn new_copied(dims: P, new_element: T) -> A2<T>
    {
        A2 {
            data: vec![new_element; (dims.x * dims.y) as usize],
            w: dims.x,
            h: dims.y,
        }
    }

    #[allow(dead_code)]
    pub fn copy_from(&self, x: i32, y: i32) -> T
    {
        let i = self.vec_idx(x, y);

        return self.data[i];
    }

    #[allow(dead_code)]
    pub fn copy_from_p(&self, p: P) -> T
    {
        let i = self.vec_idx(p.x, p.y);

        return self.data[i];
    }
}

impl<T: Default> A2<T>
{
    #[allow(dead_code)]
    pub fn new_default(dims: P) -> A2<T>
    {
        let mut a = A2 {
            data: Vec::new(),
            w: dims.x,
            h: dims.y,
        };

        a.data.resize_default(
            (a.w * a.h) as usize,
        );

        return a;
    }
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
#[derive(Copy)]
pub struct TestStructClonable
{
    pub v: i32,
}

#[cfg(test)]
impl Clone for TestStructClonable
{
    fn clone(&self) -> TestStructClonable
    {
        TestStructClonable { v: self.v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p()
    {
        // Test copy
        let p0 = P { x: 3, y: 5 };

        let p1 = p0;

        assert_eq!(p0.x, p1.x);
        assert_eq!(p0.y, p1.y);

        // Test comparison
        assert!(p0 == p1);

        // Test offset
        let p2 = p0.offset_p(P { x: 100, y: 200 });

        assert_eq!(103, p2.x);
        assert_eq!(205, p2.y);

        // Test addition
        let p3 = p0 + P { x: 200, y: 300 };

        assert_eq!(203, p3.x);
        assert_eq!(305, p3.y);

        // Test subtraction
        let p4 = p0 - P { x: 10, y: 1 };

        assert_eq!(-7, p4.x);
        assert_eq!(4, p4.y);

        // Test direction conversion
        let d = P { x: 1, y: 0 };

        let dir = d.dir();

        assert!(dir == Dir::Right);

        // Test signs
        let p = P { x: -42, y: 99 };

        let signs = p.signs();

        assert_eq!(signs.x, -1);
        assert_eq!(signs.y, 1);
    }

    #[test]
    fn test_a2()
    {
        // Test array of bools
        let mut bools: A2<bool> = A2::new_default(P { x: 5, y: 3 });

        assert_eq!(bools.size(), 5 * 3);

        assert_eq!(*bools.at(4, 2), false);

        *bools.at(4, 2) = true;

        assert_eq!(*bools.at(4, 2), true);

        // Test array of structs
        let mut test_structs: A2<TestStructClonable> =
            A2::new_copied(P { x: 512, y: 256 }, TestStructClonable { v: 42 });

        assert_eq!(test_structs.size(), 512 * 256);

        test_structs.at(17, 99).v = 1337;

        assert_eq!(test_structs.at(16, 99).v, 42);

        assert_eq!(test_structs.at(17, 99).v, 1337);
    }
}
