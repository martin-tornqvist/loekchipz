use geometry::*;
use io::*;

// Could possibly be loaded from external data files later instead
const BUTTON_WIDE_W: i32 = 180;
const BUTTON_WIDE_H: i32 = 32;

pub trait GuiElement
{
    fn update(&mut self, input: &InputData);

    fn draw(&self, io: &mut Io);
}

pub struct Button
{
    coords: R,
    text: String,
    pub is_pressed: bool,
    pub is_triggered: bool,
}

impl Button
{
    pub fn new(coords: R, text: &str) -> Button
    {
        Button {
            coords: coords,
            text: String::from(text),
            is_pressed: false,
            is_triggered: false,
        }
    }

    pub fn new_dims(pos: P, dims: P, text: &str) -> Button
    {
        let x0 = pos.x - (dims.x - 1) / 2;
        let y0 = pos.y - (dims.y - 1) / 2;

        let x1 = x0 + dims.x - 1;
        let y1 = y0 + dims.y - 1;

        let coords = R::new(P::new(x0, y0), P::new(x1, y1));

        return Button::new(coords, text);
    }

    pub fn new_wide(pos: P, text: &str) -> Button
    {
        return Button::new_dims(
            pos,
            P::new(BUTTON_WIDE_W, BUTTON_WIDE_H),
            text,
        );
    }
}

impl GuiElement for Button
{
    fn update(&mut self, input: &InputData)
    {
        self.is_triggered = false;

        let is_inside = self.coords.is_p_inside(input.mouse_pos);

        if input.mouse_left_pressed {
            if is_inside {
                self.is_pressed = true;
            }
        } else if input.mouse_left_released {
            if is_inside {
                self.is_triggered = true;
            } else {
                self.is_pressed = false;
            }
        }
    }

    fn draw(&self, io: &mut Io)
    {
        io.draw_rect(self.coords);

        io.draw_text_p(
            &self.text,
            self.coords.center(),
            TextSize::Big,
            TextAnchorX::Mid,
            TextAnchorY::Mid,
        );
    }
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button()
    {
        let mut button = Button::new(
            R {
                p0: P::new(500, 1000),
                p1: P::new(600, 1337),
            },
            "Whatever",
        );

        let mut input = InputData::new();

        input.mouse_left_pressed = true;

        // Top left pixel clicked - should be pressed
        {
            button.is_pressed = false;
            input.mouse_pos = P::new(500, 1000);
            button.update(&input);
            assert!(button.is_pressed);
        }

        // One pixel to the left of the top right pixel - should NOT be pressed
        {
            button.is_pressed = false;
            input.mouse_pos = P::new(499, 1000);
            button.update(&input);
            assert_eq!(button.is_pressed, false);
        }

        // Bottom right pixel clicked - should be pressed
        {
            button.is_pressed = false;
            input.mouse_pos = P::new(600, 1337);
            button.update(&input);
            assert!(button.is_pressed);
        }
    }

    #[test]
    fn test_button_dims()
    {
        // Width/height button
        {
            let button =
                Button::new_dims(P::new(500, 1000), P::new(3, 4), "Whatever");

            // Check width & height
            assert_eq!(button.coords.w(), 3);
            assert_eq!(button.coords.h(), 4);

            // Check coordinates
            assert_eq!(button.coords.p0.x, 499);
            assert_eq!(button.coords.p0.y, 999);
            assert_eq!(button.coords.p1.x, 501);
            assert_eq!(button.coords.p1.y, 1002);
        }

        // Custom coordinates button
        {
            let button = Button::new(
                R {
                    p0: P::new(0, 0),
                    p1: P::new(3, 100),
                },
                "Whatever",
            );

            // Check width & height
            assert_eq!(button.coords.w(), 4);
            assert_eq!(button.coords.h(), 101);
        }
    }
}
