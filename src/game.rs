use geometry::*;
use io::*;
use map::{to_map_pos, to_px_pos};
use pathfind::pathfind;
use states::*;
use world::*;

fn draw_path(px_pos: &P, path: &Vec<P>, io: &mut Io)
{
    for i in 0..path.len() {
        let mut px_pos_prev = if i == 0 {
            *px_pos
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
        GameState {
            world: World::new(),
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
        let mut actor = Actor::new();

        actor.gfx = '@';

        actor.pos = P::new(0, 0);

        actor.time_units = 20;

        self.world.actors.push(actor);

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        // Draw actors
        for actor in self.world.actors.iter() {
            let px_pos = to_px_pos(actor.pos);

            // Draw actor
            io.draw_char(
                actor.gfx,
                px_pos.x + (TILE_SIZE / 2),
                px_pos.y + (TILE_SIZE / 2),
                TextSize::Big,
                TextAnchorX::Mid,
                TextAnchorY::Mid,
            );

            // Draw time units
            io.draw_text(
                &actor.time_units.to_string(),
                px_pos.x + TILE_SIZE,
                px_pos.y,
                TextSize::Small,
                TextAnchorX::Right,
                TextAnchorY::Top,
            );

            // Draw the movement path
            if !actor.movement.path.is_empty() {
                draw_path(&px_pos, &actor.movement.path, io);
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

        for actor in self.world.actors.iter_mut() {
            let mov = &mut actor.movement;

            if !mov.is_moving {
                continue;
            }

            // Actor is moving

            assert!(!mov.path.is_empty());

            if actor.time_units <= 0 {
                continue;
            }

            let next_pos = mov.path[0];

            mov.path.remove(0);

            actor.pos = next_pos;

            actor.time_units -= 1;

            // Target reached?
            if mov.path.is_empty() {
                mov.is_moving = false;
            }
        }

        let input = io.read();

        if input.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        if input.mouse_left_pressed {
            let selected_map_pos = to_map_pos(input.mouse_pos);

            let actor = &mut self.world.actors[0];

            let mov = &mut actor.movement;

            let target_before: Option<P> = if mov.path.is_empty() {
                None
            } else {
                Some(*mov.path.last().unwrap())
            };

            mov.path = pathfind(actor.pos, selected_map_pos, &blocked_dummy);

            if target_before.is_some() {
                if selected_map_pos == target_before.unwrap() {
                    // Selected position is actor's target - start moving
                    mov.is_moving = true;
                }
            }
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {
    }
} // impl State for GameState
