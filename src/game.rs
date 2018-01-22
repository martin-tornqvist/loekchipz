use entity::*;
use geometry::*;
use io::*;
use map::{to_map_pos, to_px_pos};
use pathfind::pathfind;
use states::*;
use world::*;

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState {
    world: World,
    ent_id_generator: IdGenerator,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            ent_id_generator: IdGenerator::new(),
        }
    }
}

impl State for GameState {
    fn name(&self) -> &str {
        return "Game";
    }

    fn on_pushed(&mut self) {}

    fn on_start(&mut self) -> Vec<StateSignal> {
        let id = self.ent_id_generator.create();

        self.world.gfx.add(id, '@');

        self.world.pos.add(id, P::new(0, 0));

        self.world.movement.add(
            id,
            Movement {
                path: vec![],
                is_moving: false,
            },
        );

        self.world.time_units.add(id, 20);

        return Vec::new();
    }

    fn on_resume(&mut self) {}

    fn draw(&mut self, io: &mut Io) {
        // Draw entities
        for ent_gfx_entry in self.world.gfx.entries.iter() {
            let id = ent_gfx_entry.ent_id;

            let ent_gfx = ent_gfx_entry.data;

            let ent_pos = self.world.pos.try_get_for(id);

            if ent_pos.is_none() {
                continue;
            }

            let px_pos = to_px_pos(*ent_pos.unwrap());

            io.draw_char(
                ent_gfx,
                px_pos.x + (TILE_SIZE / 2),
                px_pos.y + (TILE_SIZE / 2),
                TextSize::Big,
                TextAnchorX::Mid,
                TextAnchorY::Mid,
            );

            let time_units = self.world.time_units.try_get_for(id);

            if time_units.is_some() {
                io.draw_text(
                    &time_units.unwrap().to_string(),
                    px_pos.x + TILE_SIZE,
                    px_pos.y,
                    TextSize::Small,
                    TextAnchorX::Right,
                    TextAnchorY::Top,
                );
            }

            // Draw the movement path of the current entity
            let ent_movement = self.world.movement.try_get_for(id);

            if ent_movement.is_some() {
                let path: &Vec<P> = &ent_movement.unwrap().path;

                for i in 0..path.len() {
                    let mut px_pos_prev = if i == 0 {
                        px_pos
                    } else {
                        to_px_pos(path[i - 1])
                    };

                    let px_offset = P::new(TILE_SIZE / 2, TILE_SIZE / 2);

                    px_pos_prev = px_pos_prev + px_offset;

                    let px_pos_current = to_px_pos(path[i]) + px_offset;

                    io.draw_line(
                        px_pos_prev.x,
                        px_pos_prev.y,
                        px_pos_current.x,
                        px_pos_current.y,
                    );
                }
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

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal> {
        let blocked_dummy = A2::new_copied(P::new(100, 100), false);

        for movement in self.world.movement.entries.iter_mut() {
            // Is moving?
            if !movement.data.is_moving {
                continue;
            }

            assert!(!movement.data.path.is_empty());

            let id = movement.ent_id;

            // Enough time units to move?
            let time_units = self.world.time_units.try_get_for(id);

            if time_units.is_some() && **time_units.as_ref().unwrap() <= 0 {
                continue;
            }

            let entity_pos = self.world.pos.get_for(id);

            let entity_movement = &mut movement.data;

            let next_p: P = entity_movement.path[0];

            entity_movement.path.remove(0);

            *entity_pos = next_p;

            // Decrement time units
            if time_units.is_some() {
                *time_units.unwrap() -= 1;
            }

            // Target reached?
            if entity_movement.path.is_empty() {
                entity_movement.is_moving = false;
            }
        }

        let input = io.read();

        if input.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        if input.mouse_left_pressed {
            let selected_map_pos = to_map_pos(input.mouse_pos);

            let ent_move = &mut self.world.movement.get_for(0);

            let mut target_before: Option<P> = None;

            if ent_move.path.last().is_some() {
                target_before = Some(*ent_move.path.last().unwrap());
            }

            ent_move.path = pathfind(
                *self.world.pos.get_for(0),
                selected_map_pos,
                &blocked_dummy,
            );

            if target_before.is_some() {
                log!("Entity has target");

                if selected_map_pos == target_before.unwrap() {
                    // Selected position is same as entities target

                    log!("Selected position is SAME as entity target");

                    ent_move.is_moving = true;
                } else {
                    // Selected position is different from entities target

                    log!("Selected position is DIFFERENT from entity target");

                    ent_move.is_moving = false;
                }
            } else {
                // Entity does not have a current target

                log!("Entity does NOT have a target");

                assert!(!ent_move.is_moving);
            }
        }

        return Vec::new();
    }

    fn on_popped(&mut self) {}
} // impl State for GameState
