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
            // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***
            new_game_button: Button::new(
                R {
                    p0: P { x: 128, y: 128 },
                    p1: P { x: 256, y: 192 },
                },
                "New game",
            ),
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
            TextAnchorX::Left,
            TextAnchorY::Top,
        );

        // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***
        self.new_game_button.draw(io);
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        // *** ONLY FOR DEMO PURPOSES - IMPLEMENT A BETTER SETUP ***
        self.new_game_button.update(&d);

        // Proceed to game state?
        if self.new_game_button.is_triggered || d.char == 'n'
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
