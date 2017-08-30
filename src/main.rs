#![feature(vec_resize_default)]

extern crate sdl2;

mod geometry;
mod states;

mod entity;
mod gamestate;
mod map;

mod floodfill;


use map::Map;
use std::path::Path;
use gamestate::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use states::State;
use states::StateFinished;
use states::States;
use std::time::Duration;
use entity::Entity;
use geometry::P;

// -----------------------------------------------------------------------------
// *** T E M P O R A R Y   D U M M Y   S T A T E ***
// -----------------------------------------------------------------------------
struct MainMenuState {}

impl State for MainMenuState
{
    fn on_pushed(&mut self) -> StateFinished
    {
        println!("MainMenuState: on_pushed()");

        return StateFinished::No;
    }

    fn on_start(&mut self) -> StateFinished
    {
        println!("MainMenuState: on_start()");

        return StateFinished::No;
    }

    fn draw(&mut self, texture: &sdl2::render::Texture, canvas: &mut sdl2::render::WindowCanvas)
    {
        println!("MainMenuState: draw()");
    }

    // TODO: Don't pass SDL stuff here, find a better way to read user input
    fn update(&mut self, sdl_context: &sdl2::Sdl) -> StateFinished
    {
        println!("MainMenuState: update()");

        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    return StateFinished::Yes;
                }
                _ =>
                {}
            }
        }

        return StateFinished::No;
    }

    fn on_popped(&mut self)
    {
        println!("MainMenuState: on_popped()");
    }
}

pub fn main()
{
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

    let texture_creator = canvas.texture_creator();
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("gfx/tile_sheet.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();
    
    let mut states = States::new();
    
    states.push(Box::new(GameState {m :Map::new()}));

    'state_loop: loop
    {
        if states.is_empty()
        {
            break 'state_loop;
        }

        let state_finished = states.start();

        if state_finished == StateFinished::Yes
        {
            states.pop();

            continue;
        }

        canvas.clear();

        states.draw(&texture, &mut canvas);
        
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
