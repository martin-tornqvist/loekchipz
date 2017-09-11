use geometry::Dir;
use geometry::P;

#[allow(dead_code)]
pub struct Entity
{
    pos: P,
    is_blocking: bool,
    tiletype: i32,
    name: String,
}

impl Entity
{
    pub fn new(pos: P, is_blocking: bool, tiletype: i32, name: String)
        -> Entity
    {
        Entity {
            pos: pos,
            is_blocking: is_blocking,
            tiletype: tiletype,
            name: name,
        }
    }

    #[allow(dead_code)]
    pub fn move_pos(&mut self, p: P)
    {
        self.pos = p;
    }

    #[allow(dead_code)]
    pub fn move_dir(&mut self, _: Dir)
    {
        // ...
    }

    #[allow(dead_code)]
    pub fn get_pos(&self) -> P
    {
        return self.pos;
    }

    #[allow(dead_code)]
    pub fn get_tile_type(&self) -> i32
    {
        return self.tiletype;
    }
}
