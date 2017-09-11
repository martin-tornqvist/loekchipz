extern crate sfml;

use self::sfml::graphics::{Color, RenderTarget, RenderWindow};
use self::sfml::window::{ContextSettings, VideoMode, style, Event,
                         Key as SfmlKey};

#[derive(PartialEq, Eq)]
pub enum Key
{
    Esc,
    Space,
    Return,
}

// TODO: Maybe it would be more effective to set up a table for this...
fn sfml_key_to_char(
    base_char: char,
    base_sfml_key: SfmlKey,
    sfml_key: SfmlKey,
) -> char
{
    let mut val = base_char as u8;

    val -= base_sfml_key as u8;

    val += sfml_key as u8;

    return val as char;
}

pub struct InputData
{
    pub char: Option<char>,
    pub key: Option<Key>,
}

impl InputData
{
    fn new() -> InputData
    {
        InputData {
            char: None,
            key: None,
        }
    }
}

pub struct Io
{
    window: RenderWindow,
}

impl Io
{
    pub fn new() -> Io
    {
        let title = "Loekchipz 0.0.1";

        let w = 800;
        let h = 600;

        let bpp = 32;

        let context_settings = ContextSettings {
            antialiasing_level: 0,
            ..Default::default()
        };

        let mut window = RenderWindow::new(
            VideoMode::new(w, h, bpp),
            title,
            style::TITLEBAR,
            &context_settings,
        ).unwrap();

        window.set_vertical_sync_enabled(true);

        Io { window: window }
    }

    pub fn clear_screen(&mut self)
    {
        self.window.clear(&Color::rgb(0, 0, 0));
    }

    pub fn update_screen(&mut self)
    {
        self.window.display();
    }

    pub fn read(&mut self) -> InputData
    {
        let mut d = InputData::new();

        for event in self.window.events()
        {
            match event
            {
                Event::KeyPressed { code, .. } =>
                {
                    // Letters
                    if (code >= SfmlKey::A) && (code <= SfmlKey::Z)
                    {
                        d.char = Some(sfml_key_to_char('a', SfmlKey::A, code));
                        return d;
                    }
                    // Numbers
                    else if (code >= SfmlKey::Num0) && (code <= SfmlKey::Num9)
                    {
                        d.char =
                            Some(sfml_key_to_char('0', SfmlKey::Num0, code));
                        return d;
                    }
                    // Special keys
                    else
                    {
                        match code
                        {
                            SfmlKey::Escape =>
                            {
                                d.key = Some(Key::Esc);
                                return d;
                            }
                            SfmlKey::Space =>
                            {
                                d.key = Some(Key::Space);
                                return d;
                            }
                            SfmlKey::Return =>
                            {
                                d.key = Some(Key::Return);
                                return d;
                            }
                            _ =>
                            {}
                        }
                    }
                }
                _ =>
                {}
            }
        }

        // No input was read
        return d;
    }
}
