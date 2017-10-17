use io::Io;
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Component
// -----------------------------------------------------------------------------
pub struct CompNode
{
    id: i32,
    ent_id: i32,
    pub comp: Box<Comp>,
}

impl CompNode
{
    pub fn new(id: i32, ent_id: i32, comp: Box<Comp>) -> CompNode
    {
        CompNode {
            id: id,
            ent_id: ent_id,
            comp: comp,
        }
    }

    pub fn id(&self) -> i32
    {
        return self.id;
    }

    pub fn ent_id(&self) -> i32
    {
        return self.ent_id;
    }
}

pub trait Comp
{
    fn prepare(&mut self, ent: &Ent, world: &World);

    fn operate(&mut self, ent: &Ent, world: &World);
}

// -----------------------------------------------------------------------------
// Entity
// -----------------------------------------------------------------------------
pub struct Ent
{
    id: i32,
    comp_ids: Vec<i32>,
}

impl Ent
{
    pub fn new(id: i32) -> Ent
    {
        Ent {
            id: id,
            comp_ids: vec![],
        }
    }

    pub fn id(&self) -> i32
    {
        return self.id;
    }
}

// -----------------------------------------------------------------------------
// World
// -----------------------------------------------------------------------------
pub struct World {}

impl World
{
    pub fn new() -> World
    {
        World {}
    }
}

// #[allow(dead_code)]
// pub struct Entity
// {
//     pos: P,
//     is_blocking: bool,
//     tiletype: i32,
//     name: String,
// }

// impl Entity
// {
//     pub fn new(pos: P, is_blocking: bool, tiletype: i32, name: String)
//         -> Entity
//     {
//         Entity {
//             pos: pos,
//             is_blocking: is_blocking,
//             tiletype: tiletype,
//             name: name,
//         }
//     }

//     #[allow(dead_code)]
//     pub fn move_pos(&mut self, p: P)
//     {
//         self.pos = p;
//     }

//     #[allow(dead_code)]
//     pub fn move_dir(&mut self, _: Dir)
//     {
//         // ...
//     }

//     #[allow(dead_code)]
//     pub fn get_pos(&self) -> P
//     {
//         return self.pos;
//     }

//     #[allow(dead_code)]
//     pub fn get_tile_type(&self) -> i32
//     {
//         return self.tiletype;
//     }
// }
