use entity::*;
use geometry::*;

// -----------------------------------------------------------------------------
// Movement component
// -----------------------------------------------------------------------------
pub struct Movement {
    pub path: Vec<P>,
    pub is_moving: bool,
}

// -----------------------------------------------------------------------------
// Game world components
// -----------------------------------------------------------------------------
pub struct World {
    pub gfx: CompRepo<char>,
    pub pos: CompRepo<P>,
    pub movement: CompRepo<Movement>,
    pub time_units: CompRepo<i32>,
}

impl World {
    pub fn new() -> World {
        World {
            gfx: CompRepo::new(),
            pos: CompRepo::new(),
            movement: CompRepo::new(),
            time_units: CompRepo::new(),
        }
    }
}
