use entity::*;
use io::*;
use states::*;
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Demo component
// -----------------------------------------------------------------------------
struct DemoPosComp
{
    x: i32,
}

impl DemoPosComp
{
    fn new() -> DemoPosComp
    {
        DemoPosComp { x: 11 }
    }
}

impl Comp for DemoPosComp
{
    fn prepare(&mut self, ent: &Ent, world: &World)
    {
        log!("Entity id: {}", ent.id());

        // TODO: Fetch from world
        let dx = 1;

        log!("dx: {}", dx);
    }

    fn operate(&mut self, ent: &Ent, world: &World)
    {
        // TODO: Fetch from world
        let dx = 1;

        self.x += dx;

        log!("x: {}", self.x);
    }
}

struct DemoVelComp
{
    dx: i32,
}

impl DemoVelComp
{
    fn new() -> DemoVelComp
    {
        DemoVelComp { dx: 0 }
    }
}

impl Comp for DemoVelComp
{
    fn prepare(&mut self, ent: &Ent, world: &World)
    {
        log!("Entity id: {}", ent.id());

        self.dx = 1;

        log!("dx: {}", self.dx);
    }

    fn operate(&mut self, ent: &Ent, world: &World)
    {

    }
}

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    comp_nodes: HashMap<i32, CompNode>,
    ents: HashMap<i32, Ent>,
    world: World,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            comp_nodes: HashMap::new(),
            ents: HashMap::new(),
            world: World::new(),
        }
    }
} // impl GameState

impl State for GameState
{
    fn name(&self) -> &str
    {
        return "Game";
    }

    fn on_pushed(&mut self)
    {
    }

    fn on_start(&mut self) -> Vec<StateSignal>
    {
        let demo_ent = Ent::new(42);

        let demo_pos_comp_node =
            CompNode::new(1337, 42, Box::new(DemoPosComp::new()));

        let demo_vel_comp_node =
            CompNode::new(666, 42, Box::new(DemoPosComp::new()));

        self.ents.insert(42, demo_ent);

        self.comp_nodes.insert(
            1337,
            demo_pos_comp_node,
        );

        self.comp_nodes.insert(
            666,
            demo_vel_comp_node,
        );

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        io.draw_text(
            "Press 'n' to step entities\nPress 'esc' to quit",
            0,
            0,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        if d.key == Key::Esc
        {
            return vec![StateSignal::Pop];
        }

        if d.char == 'n'
        {
            // Prepare
            for (_, comp_node) in self.comp_nodes.iter_mut()
            {
                let ent: &Ent = self.ents
                    .get(&comp_node.ent_id())
                    .unwrap();

                comp_node.comp.prepare(ent, &self.world);

                // TODO: Update world
            }

            // Operate
            for (_, comp_node) in self.comp_nodes.iter_mut()
            {
                let ent: &Ent = self.ents
                    .get(&comp_node.ent_id())
                    .unwrap();

                comp_node.comp.operate(ent, &self.world);

                // TODO: Update world
            }
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
