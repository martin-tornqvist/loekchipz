#![feature(vec_resize_default)]

#[macro_use]
mod log;
mod geometry;
mod states;
mod floodfill;
mod pathfind;
mod main_menu;
mod entity;
mod gamestate;
mod map;
mod io;

use gamestate::GameState;
use map::Map;
use states::StateFinished;
use states::States;
use std::time::Duration;

pub fn main()
{
    let mut io = io::Io::new();

    let mut states = States::new();

    states.push(Box::new(GameState { m: Map::new() }));

    'state_loop: loop
    {
        if states.is_empty()
        {
            log!("No states left - bye!");

            break 'state_loop;
        }

        let state_finished = states.start();

        if state_finished == StateFinished::Yes
        {
            states.pop();

            continue;
        }

        io.clear_screen();

        states.draw(&mut io);

        io.update();

        // TODO: Handle input
        let state_finished = states.update(&mut io);

        if state_finished == StateFinished::Yes
        {
            states.pop();
        }

        std::thread::sleep(Duration::from_millis(10));

    } // State loop
}
