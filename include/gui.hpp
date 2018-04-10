#ifndef GUI_HPP
#define GUI_HPP

#include <string>

#include "geometry.hpp"

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
        Button(const std::string& text, const R& coords);

        Button(const std::string& text, const P& center_pos, const P& dims);

        void update(const InputData& input) override;

        void draw() override;

        bool is_triggered() const
        {
                return is_triggered_;
        }

protected:
        std::string text_;
        R coords_ = R();
        bool is_pressed_ = false;
        bool is_triggered_ = false;
};

namespace gui
{

__attribute__((unused))
const int button_wide_w = 180;

__attribute__((unused))
const int button_wide_h = 32;

} // gui

#endif // GUI_HPP
