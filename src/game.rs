use entity::*;
use io::*;
use states::*;

// -----------------------------------------------------------------------------
// Demo component
// -----------------------------------------------------------------------------
struct DemoComp
{
    x: i32,
    dx: i32,
}

impl DemoComp
{
    fn new() -> DemoComp
    {
        DemoComp { x: 11, dx: 0 }
    }
}

impl Comp for DemoComp
{
    fn prepare(&mut self, _: &Ent, _: &World)
    {
        self.dx = 0;

        if self.x < 19
        {
            self.dx = 1;
        }
    }

    fn run(&mut self, _: &Ent, _: &World)
    {
        self.x += self.dx;
    }

    fn draw(&self, io: &mut Io)
    {
        let w = 16;

        let y = 128;

        io.draw_text(
            "[",
            w * 10,
            y,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );

        io.draw_text(
            "X",
            w * self.x,
            y,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );

        io.draw_text(
            "]",
            w * 20,
            y,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );
    }
}

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    world: World,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState { world: World::new() }
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
        let demo_comp = Box::new(DemoComp::new());

        let mut demo_ent = Ent::new();

        demo_ent.add_comp(demo_comp);

        self.world.push_ent(demo_ent);

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

        // Draw
        for ent in &mut self.world.ents
        {
            for comp in &mut ent.comps
            {
                comp.draw(io);
            }
        }
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
            for ent in &mut self.world.ents
            {
                for comp in &mut ent.comps
                {
                    comp.prepare(ent, &self.world);
                }
            }

            // Operate
            for ent in &mut self.world.ents
            {
                for comp in &mut ent.comps
                {
                    comp.run(ent, &self.world);
                }
            }
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
