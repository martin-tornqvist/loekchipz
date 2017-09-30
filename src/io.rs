use geometry::*;
use json;
use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture,
                     Transformable, IntRect};
use sfml::window::{ContextSettings, VideoMode, style, Event, Key as SfmlKey};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

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
// Create a new texture object from external tile sheet
// -----------------------------------------------------------------------------
pub fn make_texture() -> Texture
{
    Texture::from_file("gfx/tile_sheet.png").unwrap()
}

// -----------------------------------------------------------------------------
// Public struct handling drawing and user input (wraps SFML)
// -----------------------------------------------------------------------------
pub struct Io<'s>
{
    window: RenderWindow,
    pub sprites: Vec<Sprite<'s>>,
    sprite_table: HashMap<String, i32>,
}

impl<'s> Io<'s>
{
    pub fn new(t: &'s mut Texture) -> Io<'s>
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

        let mut io = Io {
            window: window,
            sprites: Vec::new(),
            sprite_table: HashMap::new(),
        };


        let path: PathBuf = ["data", "tiles.json"].iter().collect();

        let content = file_to_str(&path);

        let data = json::parse(&content).unwrap();

        for tile in data["tiles"].members()
        {
            let name: &str = tile["name"].as_str().unwrap();

            let x: i32 = tile["x"].as_i32().unwrap();

            let y: i32 = tile["y"].as_i32().unwrap();

            let w: i32 = tile["w"].as_i32().unwrap();

            let h: i32 = tile["h"].as_i32().unwrap();

            let mut spr = Sprite::with_texture(t);

            spr.set_texture_rect(&IntRect::new(x, y, w, h));

            io.sprites.push(spr);

            io.sprite_table.insert(
                name.to_string(),
                io.sprites.len() as i32,
            );
        }

        return io;
    }

    pub fn clear_screen(&mut self)
    {
        self.window.clear(&Color::rgb(0, 0, 0));
    }

    pub fn update_screen(&mut self)
    {
        self.window.display();
    }

    fn lookup_tile(&mut self, tile_name: &str) -> i32
    {
        let tile_id = self.sprite_table.get(tile_name);

        return *tile_id.unwrap();
    }

    pub fn draw(&mut self, tile_name: &str, dst: P)
    {

        let tile_id = self.lookup_tile(tile_name);

        self.sprites[tile_id as usize]
            .set_position2f(dst.x as f32, dst.y as f32);

        self.window.draw(
            &self.sprites[tile_id as
                              usize],
        );

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
