// -----------------------------------------------------------------------------
// Component data --- TODO: Move to own file
// -----------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
pub enum Data
{
    DemoPos
    { is_frob: bool, x: i32, nr_bars: i32 },

    DemoDelta
    { dx: i32 },
}

// -----------------------------------------------------------------------------
// Component
// -----------------------------------------------------------------------------
#[allow(dead_code)]
pub struct Comp
{
    id: i32,
    ent_id: i32,
    pub data: Data,
    pub prepare: Option<fn(data: &mut Data, ent: &Ent, world: &World)>,
    pub operate: Option<fn(data: &mut Data, ent: &Ent, world: &World)>,
}

impl Comp
{
    pub fn new(
        id: i32,
        ent_id: i32,
        data: Data,
        prepare: Option<fn(data: &mut Data, ent: &Ent, world: &World)>,
        operate: Option<fn(data: &mut Data, ent: &Ent, world: &World)>,
    ) -> Comp
    {
        Comp {
            id: id,
            ent_id: ent_id,
            data: data,
            prepare: prepare,
            operate: operate,
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> i32
    {
        return self.id;
    }

    #[allow(dead_code)]
    pub fn ent_id(&self) -> i32
    {
        return self.ent_id;
    }
}

// pub trait Comp
// {
//     fn prepare(&self, ent: &Ent, world: &World) -> Option<Box<Comp>>
//     {
//         return None
//     }

//     fn operate(&self, ent: &Ent, world: &World) -> Option<Box<Comp>>
//     {
//         return None
//     }
// }

// -----------------------------------------------------------------------------
// Entity
// -----------------------------------------------------------------------------
pub struct Ent
{
    id: i32,
    pub comps: Vec<Comp>,
}

impl Ent
{
    pub fn new(id: i32) -> Ent
    {
        Ent {
            id: id,
            comps: vec![],
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
