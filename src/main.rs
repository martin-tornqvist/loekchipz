#[macro_use]
mod log;
mod geometry;
mod states;
mod floodfill;
mod pathfind;
mod main_menu;
mod game;
mod world;
mod map;
mod io;
mod gui;
mod frobs; // *** ONLY FOR DEMO PURPOSES - TO BE REMOVED ***

extern crate json;

use main_menu::MainMenuState;
use states::*;
use std::time::Duration;

pub fn main()
{
    // -------------------------------------------
    // *** TESTING - TO BE REMOVED ***
    // -------------------------------------------
    frobs::test_load_frobs();
    // -------------------------------------------

    let mut io = io::Io::new();

    let mut states = States::new();

    states.push(Box::new(MainMenuState::new()));

    'state_loop: loop {
        if states.is_empty() {
            log!("No states left - bye!");

            break 'state_loop;
        }

        let signals_start = states.start();

        if !signals_start.is_empty() {
            // Push/pop states etc
            states.process_signals(signals_start);

            continue;
        }

        io.clear_screen();

        states.draw(&mut io);

        io.update_screen();

        let signals_update = states.update(&mut io);

        if !signals_update.is_empty() {
            // Push/pop states etc
            states.process_signals(signals_update);

            continue;
        }

        std::thread::sleep(Duration::from_millis(1));
    } // State loop
}
