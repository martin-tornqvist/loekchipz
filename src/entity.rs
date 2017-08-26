use geometry::P;

#[allow(dead_code)]
pub struct Entity{
    
    pub pos: P,
    pub is_blocking: bool,
    pub tiletype: i32
        
}

impl Entity {

    pub fn new(pos: P, is_blocking: bool, tiletype: i32) -> Entity {
        Entity {
            pos: pos,
            is_blocking: is_blocking,
            tiletype: tiletype,
        }
    }
    
    
    pub fn move_pos(&mut self, dx: i32, dy:i32) {
        self.pos.x += dx;
        self.pos.y += dy;
    }
    
}
