use entity::Entity;
use geometry::*;
use io::Io;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
const TILE_SIZE: i32 = 16;

pub struct Map
{
    pub map: A2<Vec<Entity>>,
}

impl Map
{
    pub fn new() -> Map
    {
        let map = Map::read_map();

        Map { map: map }
    }

    fn read_map() -> A2<Vec<Entity>>
    {
        let mut f = BufReader::new(File::open("data/map").unwrap());

        let mut s = String::new();

        f.read_line(&mut s).unwrap();

        let mut width: i32 = 0;
        let mut height: i32 = 0;

        let mut tiletypes: Vec<i32> = Vec::new();

        for (_y, line) in f.lines().enumerate()
        {
            for (_x, number) in line.unwrap()
                .split(char::is_whitespace)
                .enumerate()
            {
                width += 1;

                tiletypes.push(number.trim().parse().unwrap());
            }

            height += 1;
        }

        let dims: P = P {
            x: width,
            y: height,
        };

        let mut map: A2<Vec<Entity>> = A2::new_default(dims);

        for y in 0..height
        {
            for x in 0..width
            {
                map.at(x as i32, y as i32).push(
                    Entity::new(
                        P {
                            x: x as i32,
                            y: y as i32,
                        },
                        false,
                        tiletypes[0],
                        "ground".to_string(),
                    ),
                );
            }
        }

        // TODO: Remove...
        map.at(10, 10).push(Entity::new(
            P { x: 10, y: 10 },
            false,
            3,
            "obviously a goldnugget, duh".to_string(),
        ));

        map.at(14, 11).push(Entity::new(
            P { x: 14, y: 11 },
            false,
            3,
            "obviously a goldnugget, duh".to_string(),
        ));

        map.at(10, 10).push(Entity::new(
            P { x: 10, y: 10 },
            false,
            2,
            "aim".to_string(),
        ));

        return map;
    }

    pub fn render_map(&mut self, io: &mut Io)
    {

        let mut src = R {
            p0: P { x: 0, y: 0 },
            p1: P {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        };

        let mut dst = src;

        for y in 0..self.map.h()
        {

            for x in 0..self.map.w()
            {

                for i in self.map.at(x, y)
                {
                    let dst_p: P = i.get_pos();
                    let tiletype: i32 = i.get_tile_type();

                    dst.p0.x = dst_p.x * TILE_SIZE as i32;
                    dst.p0.y = dst_p.y * TILE_SIZE as i32;

                    // TODO: Enums?

                    if tiletype == 0
                    {
                        src.p0.x = 0 * TILE_SIZE as i32;
                        src.p0.y = 0 * TILE_SIZE as i32;
                        src.p1.x = TILE_SIZE as i32;
                        src.p1.y = TILE_SIZE as i32;
                    }
                    else if tiletype == 1
                    {
                        src.p0.x = 1 * TILE_SIZE as i32;
                        src.p0.y = 0 * TILE_SIZE as i32;
                        src.p1.x = TILE_SIZE as i32;
                        src.p1.y = TILE_SIZE as i32;
                    }
                    else if tiletype == 2
                    {
                        src.p0.x = 2 * TILE_SIZE as i32;
                        src.p0.y = 0 * TILE_SIZE as i32;
                        src.p1.x = TILE_SIZE as i32;
                        src.p1.y = TILE_SIZE as i32;
                    }
                    else if tiletype == 3
                    {
                        src.p0.x = 3 * TILE_SIZE as i32;
                        src.p0.y = 0 * TILE_SIZE as i32;
                        src.p1.x = TILE_SIZE as i32;
                        src.p1.y = TILE_SIZE as i32;
                    }

                    io.draw(src, dst);

                }
            }
        }
    }
}
