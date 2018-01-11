use entity::*;
use geometry::P;
use io::{Io, Key};
use map::*;
use states::*;
use world::*;

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    world: World,
    ent_id_generator: IdGenerator,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            world: World::new(),
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

        self.world.pos.add(
            id,
            MapP::new_from_map_xy(
                0,
                0,
            ),
        );

        self.world.movement.add(id, None);

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        for c in self.world.pos.entries.iter() {
            io.draw_tile(P::new(64, 0), c.data.to_px_p());
        }
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let input = io.read();

        if input.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        if input.mouse_left_pressed {
            *self.world.movement.get_for(0) = Some(MapP::new_from_px_xy(
                input.mouse_pos.x,
                input.mouse_pos.y,
            ));
        }

        for c in self.world.movement.entries.iter() {
            if c.data.is_none() {
                continue;
            }

            let id = c.ent_id;

            let p = c.data.unwrap();

            *self.world.pos.get_for(id) = p;
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
