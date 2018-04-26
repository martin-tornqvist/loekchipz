#ifndef GUI_HPP
#define GUI_HPP

#include <string>

#include "geometry.hpp"
#include "io.hpp"

struct InputData;

// Base class for gui elements
class GuiElement
{
public:
        virtual ~GuiElement() {}

        virtual void update(const InputData& input)
        {
                (void)input;
        }

        virtual void draw() {}
};

class Button: public GuiElement
{
public:
        Button(const std::string& text,
               const R& coords,
               const Color text_color,
               const Color frame_color);

        Button(const std::string& text,
               const P& center_pos,
               const P& dims,
               const Color text_color,
               const Color frame_color);

        void update(const InputData& input) override;

        void draw() override;

        bool is_triggered() const
        {
                return is_triggered_;
        }

        std::string get_text()
        {
                return text_;
        }

protected:
        std::string text_;
        R coords_ = R();
        bool is_pressed_ = false;
        bool is_triggered_ = false;
        Color text_color_;
        Color frame_color_;
};

namespace gui
{

__attribute__((unused))
const int button_wide_w = 180;

__attribute__((unused))
const int button_wide_h = 32;

} // gui

#endif // GUI_HPP
