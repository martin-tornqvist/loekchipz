use geometry::*;

pub struct Movement
{
    pub path: Vec<P>,
    pub is_moving: bool,
}

impl Movement
{
    pub fn new() -> Movement
    {
        Movement {
            path: vec![],
            is_moving: false,
        }
    }
}

pub struct Actor
{
    pub gfx: char,
    pub pos: P,
    pub movement: Movement,
    pub time_units: i32,
}

impl Actor
{
    pub fn new() -> Actor
    {
        Actor {
            gfx: 0 as char,
            pos: P::new(-1, -1),
            movement: Movement::new(),
            time_units: 0,
        }
    }
}

// -----------------------------------------------------------------------------
// Game world
// -----------------------------------------------------------------------------
pub struct World
{
    pub actors: Vec<Actor>,
}

impl World
{
    pub fn new() -> World
    {
        World { actors: vec![] }
    }
}
