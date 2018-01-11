use entity::*;
use map::*;

pub struct World
{
    pub pos: CompRepo<MapP>,
    pub movement: CompRepo<Option<MapP>>,
}

impl World
{
    pub fn new() -> World
    {
        World {
            pos: CompRepo::new(),
            movement: CompRepo::new(),
        }
    }
}
