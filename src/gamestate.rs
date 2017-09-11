use io::{Io, Key};
use map::Map;
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
        return "Game";
    }

    fn on_pushed(&mut self) -> StateFinished
    {
        return StateFinished::No;
    }

    fn on_start(&mut self) -> StateFinished
    {
        return StateFinished::No;
    }

    fn draw(&mut self, renderer: &mut Io)
    {
        self.m.render_map(renderer);
    }

    fn update(&mut self, io: &mut Io) -> StateFinished
    {
        let d = io.read();

        // TODO: Only for demo purposes, remove soon...
        if d.char.is_some()
        {
            log!("char: {}", d.char.unwrap());
        }

        if d.key == Some(Key::Esc)
        {
            return StateFinished::Yes;
        }

        return StateFinished::No;
    }

    fn on_popped(&mut self)
    {

    }
}
