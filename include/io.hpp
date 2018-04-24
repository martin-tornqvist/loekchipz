#ifndef IO_H
#define IO_H

#include <string>

#include "geometry.hpp"

struct InputData
{
        char c = 0;

        // TODO: key code (e.g. up, down, enter, backspace, ...)

        PxPos mouse_pos = {};
        bool mouse_left_pressed = false;
        bool mouse_left_released = false;
        bool mouse_right_pressed = false;
        bool mouse_right_released = false;
};

struct Color
{
        uint8_t r;
        uint8_t g;
        uint8_t b;
};

namespace io
{

void init();

void cleanup();

void draw_text(
        const std::string& str,
        const PxPos pos,
        const Color color);

void clear_screen();

void flip();

InputData read_input();

void sleep(const uint32_t duration);

uint32_t get_ticks();

} // io

#endif // IO_H
