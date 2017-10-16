extern crate sfml;

use self::sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture,
                           Transformable, IntRect, Font, Text, Shape,
                           RectangleShape};
use self::sfml::window::{ContextSettings, VideoMode, style, Event,
                         Key as SfmlKey, mouse};
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
    pub mouse_pos: P,
    pub mouse_left_pressed: bool,
    pub mouse_left_released: bool,
    pub mouse_right_pressed: bool,
    pub mouse_right_released: bool,
}

impl InputData
{
    fn new() -> InputData
    {
        InputData {
            char: 0 as char,
            key: Key::Undefined,
            mouse_pos: P::default(),
            mouse_left_pressed: false,
            mouse_left_released: false,
            mouse_right_pressed: false,
            mouse_right_released: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Public struct handling drawing and user input (wraps SFML)
// -----------------------------------------------------------------------------
#[allow(dead_code)]
pub struct Io
{
    window: RenderWindow,
    texture: Texture,
    font: Font,
}

// -----------------------------------------------------------------------------
// Text drawing parameters
// -----------------------------------------------------------------------------
#[allow(dead_code)]
pub enum TextSize
{
    Small,
    Big,
}

#[allow(dead_code)]
pub enum TextAnchorX
{
    Left,
    Mid,
    Right,
}

#[allow(dead_code)]
pub enum TextAnchorY
{
    Top,
    Mid,
    Bottom,
}

// -----------------------------------------------------------------------------
// IO struct responsible for all input/output handling
// -----------------------------------------------------------------------------
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

        let font = Font::from_file("font/DejaVuSans.ttf").unwrap();

        Io {
            window: window,
            texture: texture,
            font: font,
        }
    }

    pub fn clear_screen(&mut self)
    {
        self.window.clear(
            &Color::rgb(10, 10, 10),
        );
    }

    pub fn update_screen(&mut self)
    {
        self.window.display();
    }

    // TODO: Draw what? /Martin
    #[allow(dead_code)]
    pub fn draw(&mut self, src: R, dst: R)
    {
        let mut spr = Sprite::new();
        spr.set_texture(&self.texture, true);
        spr.set_texture_rect(&IntRect::new(src.p0.x, src.p0.y, 16, 16));
        spr.set_position2f(dst.p0.x as f32, dst.p0.y as f32);

        self.window.draw(&spr);
    }

    pub fn draw_text(
        &mut self,
        str: &str,
        x: i32,
        y: i32,
        size: TextSize,
        anchor_x: TextAnchorX,
        anchor_y: TextAnchorY,
    )
    {
        let mut text = Text::new();

        text.set_font(&self.font);

        let text_size = match size
        {
            TextSize::Small => 12,
            TextSize::Big => 16,
        };

        text.set_character_size(text_size);

        text.set_string(str);

        let text_rect = text.local_bounds();

        let text_w = text_rect.width;
        let text_h = text_rect.height;

        let mut origin_x = text_rect.left;
        let mut origin_y = text_rect.top;

        match anchor_x
        {
            TextAnchorX::Left => origin_x += 0.0,
            TextAnchorX::Mid => origin_x += text_w / 2.0,
            TextAnchorX::Right => origin_x += text_w,
        }

        match anchor_y
        {
            TextAnchorY::Top => origin_y += 0.0,
            TextAnchorY::Mid => origin_y += text_h / 2.0,
            TextAnchorY::Bottom => origin_y += text_h,
        }

        origin_x = origin_x.round();
        origin_y = origin_y.round();

        text.set_origin2f(origin_x, origin_y);

        text.set_position2f(x as f32, y as f32);

        self.window.draw(&text);
    }

    pub fn draw_text_p(
        &mut self,
        str: &str,
        p: P,
        text_size: TextSize,
        anchor_x: TextAnchorX,
        anchor_y: TextAnchorY,
    )
    {
        self.draw_text(str, p.x, p.y, text_size, anchor_x, anchor_y);
    }

    pub fn draw_rect(&mut self, r: R)
    {
        let mut rect = RectangleShape::new();

        rect.set_size2f(r.w() as f32, r.h() as f32);

        rect.set_origin2f(0.0, 0.0);

        rect.set_position2f(r.p0.x as f32, r.p0.y as f32);

        rect.set_outline_thickness(1.0);

        rect.set_outline_color(&Color::white());

        rect.set_fill_color(&Color::rgb(64, 64, 64));

        self.window.draw(&rect);
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
                Event::MouseButtonPressed { button, x, y } =>
                {
                    d.mouse_pos = P { x: x, y: y };

                    match button
                    {
                        mouse::Button::Left => d.mouse_left_pressed = true,
                        mouse::Button::Right => d.mouse_right_pressed = true,
                        _ =>
                        {}
                    }
                }
                Event::MouseButtonReleased { button, x, y } =>
                {
                    d.mouse_pos = P { x: x, y: y };

                    match button
                    {
                        mouse::Button::Left => d.mouse_left_pressed = true,
                        mouse::Button::Right => d.mouse_right_pressed = true,
                        _ =>
                        {}
                    }
                }
                Event::MouseMoved { x, y } =>
                {
                    d.mouse_pos = P { x: x, y: y };
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
