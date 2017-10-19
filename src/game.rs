use entity::*;
use geometry::*;
use io::*;
use states::*;

// -----------------------------------------------------------------------------
// Demo agent functions
// -----------------------------------------------------------------------------
fn army_move_east(agent: &Agent, world: &mut World)
{
    let army = world.find_army(agent.ent_id()).unwrap();

    army.data.pos.offset_dir(Dir::Right);
}

fn army_move_south(agent: &Agent, world: &mut World)
{
    let army = world.find_army(agent.ent_id()).unwrap();

    army.data.pos.offset_dir(Dir::Down);
}

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    world: World,
    agents: Vec<Agent>,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            world: World::new(),
            agents: vec![],
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
        // TODO: The IDs here will be handled by some kind of allocator object

        self.world.armies.push(Ent::new(
            42,
            ArmyData {
                pos: P { x: 18, y: 50 },
                size: 5000,
            },
        ));

        self.agents.push(Agent::new(
            42,
            Some(army_move_east),
        ));

        self.world.armies.push(Ent::new(
            1337,
            ArmyData {
                pos: P { x: 77, y: 9 },
                size: 8750,
            },
        ));

        self.agents.push(Agent::new(
            1337,
            Some(army_move_south),
        ));

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
            // TODO: Just iterating over the agents for now - but eventually
            // this will rather be something like:
            // "For each agent, for each army, for the current team, do actions
            // until this team has finished its turn"
            for agent in &self.agents
            {
                if agent.act.is_some()
                {
                    agent.act.unwrap()(&agent, &mut self.world);
                }
            }

            let army_1 = &self.world.armies[0];
            let army_2 = &self.world.armies[1];

            log!(
                "Army 1 position: ({},{})",
                army_1.data.pos.x,
                army_1.data.pos.y
            );
            log!(
                "Army 2 position: ({},{})",
                army_2.data.pos.x,
                army_2.data.pos.y
            );
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
