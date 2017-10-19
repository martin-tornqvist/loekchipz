use geometry::*;

// -----------------------------------------------------------------------------
// Entity - Generic container struct for objects in a world
// -----------------------------------------------------------------------------
pub struct Ent<T>
{
    id: i32,
    pub data: T,
}

impl<T> Ent<T>
{
    pub fn new(id: i32, data: T) -> Ent<T>
    {
        Ent { id: id, data: data }
    }

    pub fn id(&self) -> i32
    {
        return self.id;
    }
}

// -----------------------------------------------------------------------------
// World data types --- TODO: Define in separate file (maybe game_world.rs)
// -----------------------------------------------------------------------------
pub struct ArmyData
{
    pub pos: P,
    pub size: i32,
}

// -----------------------------------------------------------------------------
// World --- TODO: Define in separate file (maybe game_world.rs)
// -----------------------------------------------------------------------------
pub struct World
{
    pub armies: Vec<Ent<ArmyData>>,
}

impl World
{
    pub fn new() -> World
    {
        World { armies: vec![] }
    }

    pub fn find_army(&mut self, ent_id: i32) -> Option<&mut Ent<ArmyData>>
    {
        for army in &mut self.armies
        {
            if army.id() == ent_id
            {
                return Some(army);
            }
        }

        return None;
    }
}

// -----------------------------------------------------------------------------
// Agent --- TODO: Define in separate file
// -----------------------------------------------------------------------------
pub struct Agent
{
    ent_id: i32,
    pub act: Option<fn(agent: &Agent, world: &mut World)>,
}

impl Agent
{
    pub fn new(
        ent_id: i32,
        act: Option<fn(agent: &Agent, world: &mut World)>,
    ) -> Agent
    {
        Agent {
            ent_id: ent_id,
            act: act,
        }
    }

    pub fn ent_id(&self) -> i32
    {
        return self.ent_id;
    }
}
