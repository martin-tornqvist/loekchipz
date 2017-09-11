use io::Io;
use states::*;

#[allow(dead_code)]
pub struct MainMenuState {}

impl State for MainMenuState
{
    fn name(&self) -> &str
    {
        return "Main menu";
    }

    fn on_pushed(&mut self) -> StateFinished
    {
        return StateFinished::No;
    }

    fn on_start(&mut self) -> StateFinished
    {
        return StateFinished::No;
    }

    fn draw(&mut self, _: &mut Io)
    {
    }

    fn update(&mut self, _: &mut Io) -> StateFinished
    {
        return StateFinished::Yes;
    }

    fn on_popped(&mut self)
    {

    }
}
