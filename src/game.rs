use entity::*;
use geometry::*;
use io::*;
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

        self.world.time_units.add(id, 20);

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        for c in self.world.pos.entries.iter() {
            let id = c.ent_id;

            let px_p = c.data.to_px_p();

            io.draw_tile(P::new(64, 0), px_p);

            if self.world.time_units.contains_for(id) {
                let time_units = *self.world.time_units.get_for(id);

                io.draw_text(
                    &time_units.to_string(),
                    px_p.x + (TILE_PX_SIZE / 2),
                    px_p.y + TILE_PX_SIZE,
                    TextSize::Small,
                    TextAnchorX::Mid,
                    TextAnchorY::Top,
                );
            }
        }
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let blocked_dummy = A2::new_copied(P::new(100, 100), false);

        for c in self.world.movement.entries.iter() {
            // Has target position?
            if c.data.is_none() {
                continue;
            }

            let id = c.ent_id;

            // Enough time units to move?
            let time_units = if self.world.time_units.contains_for(id) {
                Some(*self.world.time_units.get_for(id))
            } else {
                None
            };

            if time_units.is_some() && time_units.unwrap() <= 0 {
                continue;
            }

            let current_map_p = &mut self.world.pos.get_for(id);

            let target_map_p = c.data.unwrap();

            let path =
                pathfind(current_map_p.pos, target_map_p.pos, &blocked_dummy);

            if !path.is_empty() {
                let next_p = path[0];

                current_map_p.pos = next_p;

                // Decrement time units
                if time_units.is_some() {
                    *self.world.time_units.get_for(id) -= 1;
                }
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
