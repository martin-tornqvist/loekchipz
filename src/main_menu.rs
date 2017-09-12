use game::GameState;
use io::*;
use states::*;

#[allow(dead_code)]
pub struct MainMenuState {}

impl State for MainMenuState
{
    fn name(&self) -> &str
    {
        return "Main menu";
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

    fn draw(&mut self, _: &mut Io)
    {
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        // Proceed to game state?
        if d.char == 'n'
        {
            let game_state = Box::new(GameState::new());

            return vec![StateSignal::Push { state: game_state }];
        }

        // Exit game?
        if d.key == Key::Esc
        {
            return vec![StateSignal::Pop];
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
}
