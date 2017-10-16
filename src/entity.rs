use io::Io;

// -----------------------------------------------------------------------------
// Component trait
// -----------------------------------------------------------------------------
pub trait Comp
{
    fn prepare(&mut self, ent: &Ent, world: &World);

    fn run(&mut self, ent: &Ent, world: &World);

    fn draw(&self, io: &mut Io);
}

// -----------------------------------------------------------------------------
// Entity - carries components
// -----------------------------------------------------------------------------
pub struct Ent
{
    pub comps: Vec<Box<Comp>>,
}

impl Ent
{
    pub fn new() -> Ent
    {
        Ent { comps: vec![] }
    }

    pub fn add_comp(&mut self, comp: Box<Comp>)
    {
        self.comps.push(comp);
    }
}

// -----------------------------------------------------------------------------
// World - carries entities (this could be for the map, for a battle, ...)
// -----------------------------------------------------------------------------
pub struct World
{
    pub ents: Vec<Ent>,
}

impl World
{
    pub fn new() -> World
    {
        World { ents: vec![] }
    }

    pub fn push_ent(&mut self, ent: Ent)
    {
        self.ents.push(ent);
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
