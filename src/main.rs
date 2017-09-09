#![feature(vec_resize_default)]

extern crate sdl2;

#[macro_use]
mod log;
mod geometry;
mod states;
mod floodfill;
mod pathfind;
mod main_menu;

use main_menu::MainMenuState;
use states::StateFinished;
use states::States;
use std::time::Duration;

pub fn main()
{
    log!("Init SDL");

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let title = "Loekchipz 0.0.1";

    let w = 800;
    let h = 600;

    let window = video_subsystem
        .window(title, w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut states = States::new();

    states.push(Box::new(MainMenuState {}));

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

        canvas.clear();

        states.draw();

        canvas.present();

        // TODO: Don't pass SDL stuff here, find a better way to read user input
        let state_finished = states.update(&sdl_context);

        if state_finished == StateFinished::Yes
        {
            states.pop();
        }

        std::thread::sleep(Duration::from_millis(10));

    } // State loop
}
