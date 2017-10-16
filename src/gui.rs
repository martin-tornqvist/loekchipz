use geometry::*;
use io::*;

pub trait GuiElement
{
    fn update(&mut self, input_data: &InputData);

    fn draw(&self, io: &mut Io);
}

pub struct Button
{
    px_coords: R,
    text: String,
    pub is_pressed: bool,
    pub is_triggered: bool,
}

impl Button
{
    pub fn new(px_coords: R, text: &str) -> Button
    {
        Button {
            px_coords: px_coords,
            text: String::from(text),
            is_pressed: false,
            is_triggered: false,
        }
    }
}

impl GuiElement for Button
{
    fn update(&mut self, input_data: &InputData)
    {
        self.is_triggered = false;

        if input_data.mouse_left_pressed &&
            self.px_coords.is_p_inside(
                input_data.mouse_pos,
            )
        {
            self.is_pressed = true;
            self.is_triggered = true;
        }
    }

    fn draw(&self, io: &mut Io)
    {
        io.draw_rect(self.px_coords);

        io.draw_text_p(
            &self.text,
            self.px_coords.center(),
            TextSize::Big,
            TextAnchorX::Mid,
            TextAnchorY::Mid,
        );
    }
}
