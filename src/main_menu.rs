use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use states::*;

pub struct MainMenuState {}

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
    }

    // TODO: Don't pass SDL stuff here, find a better way to read user input
    fn update(&mut self, sdl_context: &sdl2::Sdl) -> StateFinished
    {
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
