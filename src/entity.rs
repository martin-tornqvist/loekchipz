use geometry::P;
use geometry::Dir;

#[allow(dead_code)]
pub struct Entity{
    
    pos: P,
    is_blocking: bool,
    tiletype: i32,
    name: String,
        
}

impl Entity {

    pub fn new(pos: P, is_blocking: bool, tiletype: i32, name: String) -> Entity {
        Entity {
            pos: pos,
            is_blocking: is_blocking,
            tiletype: tiletype,
            name: name,
        }
    }
    
    
    pub fn move_pos(&mut self, p: P) {
        self.pos = p;
    }

    pub fn move_dir(&mut self, d: Dir){
        // ...
    }

    pub fn get_pos(&self) -> P {
        return self.pos;
    }

    pub fn get_tile_type(&self) -> i32 {
        return self.tiletype;
    }
    
}
