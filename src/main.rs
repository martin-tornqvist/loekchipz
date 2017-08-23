extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod geometry;

use geometry::P;

pub fn main()
{
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context
        .video()
        .unwrap();

    let title = "Loekchipz 0.0.1";

    let dims = P { x: 800, y: 600 };

    let window = video_subsystem
        .window(title, dims.x as u32, dims.y as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context
        .event_pump()
        .unwrap();

    'running: loop
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                }
                _ =>
                {}
            }
        }
    }
}
