use io::{Io, Key};
use map::Map;
use states::*;

pub struct GameState
{
    pub m: Map,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState { m: Map::new() }
    }
} // impl GameState

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
        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, renderer: &mut Io)
    {
        self.m.render_map(renderer);
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        if d.key == Key::Esc
        {
            return vec![StateSignal::Pop];
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
