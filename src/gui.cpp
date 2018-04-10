#include "gui.hpp"

#include "io.hpp"

// -----------------------------------------------------------------------------
// Button
// -----------------------------------------------------------------------------
Button::Button(const std::string& text, const R& coords) :
        text_(text),
        coords_(coords) {}

Button::Button(const std::string& text, const P& center_pos, const P& dims) :
        text_(text),
        coords_()
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
        // TODO: Draw rectangle (add functionality to the io namespace)

        // TODO: Draw text centered horizontally and vertically (add
        // functionality to the io namespace)
}
