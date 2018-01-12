use entity::*;
use geometry::*;
use io::{Io, Key};
use map::*;
use pathfind::pathfind;
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
        let blocked_dummy = A2::new_copied(P::new(100, 100), false);

        for c in self.world.movement.entries.iter() {
            if c.data.is_none() {
                continue;
            }

            let id = c.ent_id;

            let current_map_p = &mut self.world.pos.get_for(id);

            let target_map_p = c.data.unwrap();

            let path =
                pathfind(current_map_p.pos, target_map_p.pos, &blocked_dummy);

            if !path.is_empty() {
                let next_p = path[0];

                current_map_p.pos = next_p;
            }
        }

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

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
