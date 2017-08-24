extern crate sdl2;

mod geometry;
mod states;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use states::State;
use states::StateFinished;
use states::States;
use std::time::Duration;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;

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

    fn draw(&mut self)
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

    let point_a = Point::new(40, 10);
    let point_b = Point::new(400, 300);

    let window = video_subsystem
        .window(title, w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut tex0 = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 32, 64).unwrap();
    tex0.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..64 {
            for x in 0..32 {
                let offset = y * pitch + 3 * x;
                buffer[offset + 0] = (13 * x) as u8;
                buffer[offset + 1] = (37 * y) as u8;
                buffer[offset + 2] = 0;
            }
        }
    }).unwrap();

    let mut states = States::new();

    states.push(Box::new(MainMenuState {}));

    let mut t = 35.0f64;

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

        states.draw();

        canvas.copy(&tex0, None, Some(Rect::new(100, 50, 32, 64))).unwrap();
        let moving_x = 133.7 * (0.01 * t).cos();
        canvas.copy_ex(&tex0, None, Some(Rect::new((500.0 + moving_x) as i32, 100, 32, 64)), 35.0, None, false, false).unwrap();
        t = t + 1.0;
        canvas.copy_ex(&tex0, None, Some(Rect::new(500, 300, 4 * 32, 4 * 64)), t, None, false, false).unwrap();
        canvas.draw_line(point_a, point_b).unwrap();


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
