use game::GameState;
use geometry::*;
use gui::*;
use io::*;
use states::*;

#[allow(dead_code)]
pub struct MainMenuState
{
    // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***
    new_game_button: Button,
}

impl MainMenuState
{
    pub fn new() -> MainMenuState
    {
        MainMenuState {
            new_game_button: Button::new_wide(P { x: 300, y: 128 }, "New game"),
        }
    }
}

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

    fn draw(&mut self, io: &mut Io)
    {
        // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***

        io.draw_text(
            "This is the main menu.",
            64,
            64,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );

        self.new_game_button.draw(io);

        io.draw_tile(P::new(0, 0), P::new(32, 400));
        io.draw_tile(P::new(32, 0), P::new(64, 400));
        io.draw_tile(P::new(64, 0), P::new(96, 400));
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***
        self.new_game_button.update(&d);

        // Proceed to game state?
        if self.new_game_button.is_triggered || d.char == 'n' {
            let game_state = Box::new(GameState::new());

            return vec![StateSignal::Push { state: game_state }];
        }

        // Exit game?
        if d.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
}
