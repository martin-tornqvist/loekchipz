use entity::*;
use geometry::P;
use io::{Io, Key};
use states::*;

// -----------------------------------------------------------------------------
// Components used by the game state
// -----------------------------------------------------------------------------
struct Comps
{
    pos: CompRepo<P>,
    movement: CompRepo<Option<P>>,
}

impl Comps
{
    pub fn new() -> Comps
    {
        Comps {
            pos: CompRepo::new(),
            movement: CompRepo::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    comps: Comps,
    ent_id_generator: IdGenerator,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            comps: Comps::new(),
            ent_id_generator: IdGenerator::new(),
        }
    }
}

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
        let id = self.ent_id_generator.create();

        self.comps.pos.add(id, P::new(0, 0));

        self.comps.movement.add(id, None);

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        for c in self.comps.pos.entries.iter() {
            io.draw_tile(P::new(64, 0), c.data);
        }
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let input = io.read();

        if input.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        if input.mouse_left_pressed {

            let p = P::new(
                (input.mouse_pos.x / 32) * 32,
                (input.mouse_pos.y / 32) * 32,
            );

            *self.comps.movement.get_for(0) = Some(p);
        }

        for c in self.comps.movement.entries.iter() {
            if c.data.is_none() {
                continue;
            }

            let id = c.ent_id;

            let p = c.data.unwrap();

            *self.comps.pos.get_for(id) = p;
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
