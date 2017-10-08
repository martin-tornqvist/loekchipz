extern crate sfml;

use self::sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture,
                           Transformable, IntRect, Font, Text};
use self::sfml::window::{ContextSettings, VideoMode, style, Event,
                         Key as SfmlKey};
use geometry::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// -----------------------------------------------------------------------------
// Public data returned when reading input
// -----------------------------------------------------------------------------
pub struct InputData
{
    pub char: char,
    pub key: Key,
}

impl InputData
{
    fn new() -> InputData
    {
        InputData {
            char: 0 as char,
            key: Key::Undefined,
        }
    }
}

// -----------------------------------------------------------------------------
// Public struct handling drawing and user input (wraps SFML)
// -----------------------------------------------------------------------------
pub struct Io
{
    window: RenderWindow,
    texture: Texture,
    font: Font,
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

        let texture = Texture::from_file("gfx/tile_sheet.png").unwrap();

        let font = Font::from_file("font/alagard.ttf").unwrap();

        Io {
            window: window,
            texture: texture,
            font: font,
        }
    }

    pub fn clear_screen(&mut self)
    {
        self.window.clear(&Color::rgb(0, 0, 0));
    }

    pub fn update_screen(&mut self)
    {
        self.window.display();
    }

    // TODO: Draw what? /Martin
    pub fn draw(&mut self, src: R, dst: R)
    {
        let mut spr = Sprite::new();
        spr.set_texture(&self.texture, true);
        spr.set_texture_rect(&IntRect::new(src.p0.x, src.p0.y, 16, 16));
        spr.set_position2f(dst.p0.x as f32, dst.p0.y as f32);

        self.window.draw(&spr);
    }

    pub fn draw_text(&mut self, str: &str, x: i32, y: i32)
    {
        let mut text = Text::new();

        text.set_font(&self.font);

        text.set_character_size(40);

        text.set_position2f(x as f32, y as f32);

        // text.set_fill_color(&Color::white());

        text.set_string(str);

        self.window.draw(&text);
    }

    #[allow(dead_code)]
    pub fn draw_text_p(&mut self, str: &str, p: P)
    {
        self.draw_text(str, p.x, p.y);
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
                        d.char = sfml_key_to_char('a', SfmlKey::A, code);
                        return d;
                    }
                    // Numbers
                    else if (code >= SfmlKey::Num0) && (code <= SfmlKey::Num9)
                    {
                        d.char = sfml_key_to_char('0', SfmlKey::Num0, code);
                        return d;
                    }
                    // Special keys
                    else
                    {
                        match code
                        {
                            SfmlKey::Escape =>
                            {
                                d.key = Key::Esc;
                                return d;
                            }
                            SfmlKey::Space =>
                            {
                                d.key = Key::Space;
                                return d;
                            }
                            SfmlKey::Return =>
                            {
                                d.key = Key::Return;
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

// -----------------------------------------------------------------------------
// Read file to string
// -----------------------------------------------------------------------------
pub fn file_to_str(path: &Path) -> String
{
    let path_display = path.display();

    let mut file = match File::open(&path)
    {
        Err(why) =>
        {
            panic!(
                "Could not open file '{}': {}",
                path_display,
                why.description()
            )
        }
        Ok(file) =>
        {
            log!("Successfully opened file '{}'", path_display);

            file
        }
    };

    let mut s = String::new();

    match file.read_to_string(&mut s)
    {
        Err(why) =>
        {
            panic!(
                "Could not read file '{}': {}",
                path_display,
                why.description()
            )
        }
        Ok(_) =>
        {
            log!("Successfully read file '{}'", path_display);
        }
    }

    return s;
}

// -----------------------------------------------------------------------------
// Special keys
// -----------------------------------------------------------------------------
#[derive(PartialEq, Eq)]
pub enum Key
{
    Undefined,
    Esc,
    Space,
    Return,
}

// -----------------------------------------------------------------------------
// Convert from SFML key to char
// -----------------------------------------------------------------------------
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
