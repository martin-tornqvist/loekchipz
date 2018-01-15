use entity::*;
use map::*;

pub struct World
{
    pub gfx: CompRepo<char>,
    pub pos: CompRepo<MapP>,
    pub movement: CompRepo<Option<MapP>>,
    pub time_units: CompRepo<i32>,
}

impl World
{
    pub fn new() -> World
    {
        World {
            gfx: CompRepo::new(),
            pos: CompRepo::new(),
            movement: CompRepo::new(),
            time_units: CompRepo::new(),
        }
    }
}
