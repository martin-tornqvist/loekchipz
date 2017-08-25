use geometry::P;

#[allow(dead_code)]
pub struct Entity{
    
    pos: P,
    name: String,
    is_blocking: bool
        
}

impl Entity {

    pub fn new(pos: P, name: String, is_blocking: bool) -> Entity {
        Entity {
            pos: pos,
            name: name,
            is_blocking: is_blocking,
        }
    }
    
    
    pub fn move_pos(&mut self, dx: i32, dy:i32) {
        self.pos.x += dx;
        self.pos.y += dy;
    }
}
