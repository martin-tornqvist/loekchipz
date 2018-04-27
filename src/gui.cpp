#include "gui.hpp"

#include "io.hpp"

// -----------------------------------------------------------------------------
// Button
// -----------------------------------------------------------------------------
Button::Button(const std::string& text,
               const R& coords,
               const Color text_color,
               const Color frame_color) :
        text_(text),
        coords_(coords),
        text_color_(text_color),
        frame_color_(frame_color){}

Button::Button(const std::string& text,
               const P& center_pos,
               const P& dims,
               const Color text_color,
               const Color frame_color) :
        text_(text),
        coords_(),
        text_color_(text_color),
        frame_color_(frame_color)
{
        const P p0(center_pos.x - (dims.x - 1) / 2,
                   center_pos.y - (dims.y - 1) / 2);

        const P p1(p0 + dims - 1);

        coords_ = R(p0, p1);
}

void Button::update(const InputData& input)
{
        is_triggered_ = false;

        const bool is_inside = coords_.is_pos_inside(input.mouse_pos.value);

        if (input.mouse_left_pressed)
        {
                if (is_inside)
                {
                        is_pressed_ = true;
                }
        }
        else if (input.mouse_left_released)
        {
                if (is_inside)
                {
                        is_triggered_ = true;
                }
                else
                {
                        is_pressed_ = false;
                }
        }
}

void Button::draw()
{
        io::draw_rect(coords_, frame_color_);
        // TODO: Draw text centered horizontally and vertically (add
        // functionality to the io namespace)
        io::draw_text(text_, {coords_.p0.x, coords_.p0.y}, text_color_, coords_);
}
