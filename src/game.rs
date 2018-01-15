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

        self.world.gfx.add(id, '@');

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
        // Draw entities
        for c in self.world.gfx.entries.iter() {
            let id = c.ent_id;

            let gfx = c.data;

            let map_p = self.world.pos.try_get_for(id);

            if map_p.is_none() {
                continue;
            }

            let px_p = map_p.unwrap().to_px_p();

            io.draw_char(
                gfx,
                px_p.x + (TILE_SIZE / 2),
                px_p.y + (TILE_SIZE / 2),
                TextSize::Big,
                TextAnchorX::Mid,
                TextAnchorY::Mid,
            );

            let time_units = self.world.time_units.try_get_for(id);

            if time_units.is_some() {
                io.draw_text(
                    &time_units.unwrap().to_string(),
                    px_p.x + TILE_SIZE,
                    px_p.y,
                    TextSize::Small,
                    TextAnchorX::Right,
                    TextAnchorY::Top,
                );
            }
        }

        // Draw a grid overlay
        let nr_cells_x = 16;
        let nr_cells_y = 16;

        // Vertical lines
        for x in 0..nr_cells_x {
            io.draw_line(
                x * TILE_SIZE,
                0,
                x * TILE_SIZE,
                nr_cells_y * TILE_SIZE,
            );
        }

        // Horizontal lines
        for y in 0..nr_cells_y {
            io.draw_line(
                0,
                y * TILE_SIZE,
                nr_cells_x * TILE_SIZE,
                y * TILE_SIZE,
            );
        }
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let blocked_dummy = A2::new_copied(P::new(100, 100), false);

        for movement in self.world.movement.entries.iter() {
            // Has target position?
            if movement.data.is_none() {
                continue;
            }

            let id = movement.ent_id;

            // Enough time units to move?
            let time_units = self.world.time_units.try_get_for(id);

            if time_units.is_some() && **time_units.as_ref().unwrap() <= 0 {
                continue;
            }

            let current_map_p = &mut self.world.pos.get_for(id);

            let target_map_p = movement.data.unwrap();

            let path =
                pathfind(current_map_p.pos, target_map_p.pos, &blocked_dummy);

            if !path.is_empty() {
                let next_p = path[0];

                current_map_p.pos = next_p;

                // Decrement time units
                if time_units.is_some() {
                    *time_units.unwrap() -= 1;
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
