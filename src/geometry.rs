// pub fn foo()
// {
//     println!("foo!");
// }

#[allow(dead_code)]
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
        -1;
    }
    else if value > 0
    {
        1;
    }

    0
}

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
