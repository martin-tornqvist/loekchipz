extern crate sdl2;
use map::Map;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use states::State;
use states::StateFinished;


pub struct GameState
{
    pub m: Map,
}

impl State for GameState
{
    fn name(&self) -> &str
    {
        return "GameState";
    }

    fn on_pushed(&mut self) -> StateFinished
    {
        println!("GameState: on_pushed()");

        return StateFinished::No;
    }

    fn on_start(&mut self) -> StateFinished
    {
        println!("GameState: on_start()");
        let mut m: Map = Map::new();
        return StateFinished::No;
    }

    fn draw(
        &mut self,
        texture: &sdl2::render::Texture,
        canvas: &mut sdl2::render::WindowCanvas,
    )
    {
        self.m.render_map(&texture, canvas);
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
        println!("GameState: on_popped()");
    }
}
