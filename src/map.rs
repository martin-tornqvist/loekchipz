extern crate sdl2;

use geometry::P;
use entity::Entity;
use sdl2::rect::Rect;
use sdl2::sys::SDL_Texture;
use sdl2::render::Canvas;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TILE_SIZE:  i32 = 16;

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
        
        let mut f = BufReader::new(File::open("data/map").unwrap());

        let mut s = String::new();
        f.read_line(&mut s).unwrap();

        for(_y, line) in f.lines().enumerate() {

            for(_x, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                let new_x: i32 = _x as i32;
                let new_y: i32 = _y as i32;
                map.push(Entity::new( P{x: new_x, y: new_y},
                                      false,
                                      number.trim().parse().unwrap(),
                                      "ground".to_string()));
            }
            
        }
        
    }
    
    pub fn render_map(&mut self,
                      sdl_texture: &sdl2::render::Texture,
                      sdl_context: &mut sdl2::render::WindowCanvas) {
        
        let mut src = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);
        let mut dst = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);

        for i in &self.map {
            let dst_p: P = i.get_pos();
            let tiletype: i32 = i.get_tile_type();;
            dst.x = dst_p.x*TILE_SIZE as i32;
            dst.y = dst_p.y*TILE_SIZE as i32;

            // TODO: Enums?
            
            if tiletype == 0 {
                src.x = 0*TILE_SIZE as i32;
                src.y = 0*TILE_SIZE as i32;
            }
            else if tiletype == 1 {
                src.x = 1*TILE_SIZE as i32;
                src.y = 0*TILE_SIZE as i32;
            }
            else if tiletype == 2 {
                src.x = 2*TILE_SIZE as i32;
                src.y = 0*TILE_SIZE as i32;
            }
            
            sdl_context.copy(sdl_texture, Some(src), Some(dst)).unwrap();
            
        }
        
    }

}
