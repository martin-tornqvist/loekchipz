extern crate sdl2;

use geometry::P;
use entity::Entity;
use sdl2::rect::Rect;
use sdl2::sys::SDL_Texture;
use sdl2::render::Canvas;

// This is fucking ridiculous... Seriously? Can't I just have a const "value"?
const MAP_WIDTH_U32:  u32 = 50;
const MAP_HEIGHT_U32: u32 = 50;
const MAP_WIDTH_I32:  i32 = 50;
const MAP_HEIGHT_I32: i32 = 50;
const TILE_SIZE_U32:  u32 = 16;
const TILE_SIZE_I32:  i32 = 16;

pub struct Map {

    pub map: Vec<Entity>,
}

impl Map {
    pub fn new() -> Map{
        let mut map = Vec::new();
        Map::init_map(&mut map);
        Map { map: map }
    }
    
    fn init_map(map: &mut Vec<Entity>) {
        
        for _y in 0..MAP_HEIGHT_I32 {
            for _x in 0..MAP_WIDTH_I32 {
                let p: P = P{x: _x, y: _y};
                map.push(Entity::new(p, false));
                
            }
        }
        
    }

    pub fn render_map(&mut self, sdl_texture: &sdl2::render::Texture, sdl_context: &mut sdl2::render::WindowCanvas){
        let mut src = Rect::new(0, 0, TILE_SIZE_U32, TILE_SIZE_U32);
        let mut dst = Rect::new(0, 0, TILE_SIZE_U32, TILE_SIZE_U32);

        for i in &self.map {
            let temp_p: P = i.pos;
            dst.x = temp_p.x*TILE_SIZE_I32;
            dst.y = temp_p.y*TILE_SIZE_I32;
            
            sdl_context.copy(sdl_texture, Some(src), Some(dst)).unwrap();
            
        }
        
    }

}
