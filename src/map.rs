extern crate sdl2;

use geometry::P;
use geometry::A2;
use entity::Entity;
use sdl2::rect::Rect;

use std::fs::File;
use std::io::{BufRead, BufReader};

const TILE_SIZE:  i32 = 16;

pub struct Map {

    pub map: A2<Vec<Entity>>,
}

impl Map {
    pub fn new() -> Map{

        let mut map: A2<Vec<Entity>>;
        
        map = Map::read_map();
        Map { map: map }
        
    }

    fn read_map() -> A2<Vec<Entity>> {
        
        let mut f = BufReader::new(File::open("data/map").unwrap());
        
        let mut s = String::new();

        f.read_line(&mut s).unwrap();

        let mut width :i32 = 0;
        let mut height :i32 = 0;

        let mut first_line :bool = true;

        let mut tiletypes :Vec<i32> = Vec::new();
        
        for(_y, line) in f.lines().enumerate() {

            for(_x, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                
                if first_line {
                    width += 1;
                }
                tiletypes.push(number.trim().parse().unwrap());
                
            }
            height += 1;
        }
        
        let dims :P = P{x: width, y: height};
        
        let mut map: A2<Vec<Entity>> = A2::new_default(dims);


        let mut tile_counter :i32 = 0;
        
        for _y in 0..height {

            for _x in 0..width {

                map.at(_x as i32, _y as i32).push(Entity::new( P{x: _x as i32, y: _y as i32},
                                                               false,
                                                               tiletypes[tile_counter as usize],
                                                               "ground".to_string()));
                
            }
            
        }


        // TODO: Remove...
        map.at(10, 10).push(Entity::new( P{x: 10, y: 10},
                                         false,
                                         3,
                                         "obviously a goldnugget, duh".to_string()));

        map.at(14, 11).push(Entity::new( P{x: 14, y: 11},
                                         false,
                                         3,
                                         "obviously a goldnugget, duh".to_string()));
        
        map.at(10, 10).push(Entity::new( P{x: 10, y: 10},
                                         false,
                                         2,
                                         "aim".to_string()));
        
 

        
        
        
        
        return map;
        
    }
    
    
    pub fn render_map(&mut self,
                      sdl_texture: &sdl2::render::Texture,
                      sdl_context: &mut sdl2::render::WindowCanvas) {
        
        let mut src = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);
        let mut dst = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);

        for _y in 0..self.map.h() {
            
            for _x in 0..self.map.w() {

                for i in self.map.at(_x, _y) {
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
                    else if tiletype == 3 {
                        src.x = 3*TILE_SIZE as i32;
                        src.y = 0*TILE_SIZE as i32;
                    }
                    
                    sdl_context.copy(sdl_texture, Some(src), Some(dst)).unwrap();
                }

                
            }
        }


    }

}

