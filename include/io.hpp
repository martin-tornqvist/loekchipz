#ifndef IO_H
#define IO_H

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

struct Color
{
        short int r;
        short int g;
        short int b;
};

namespace io
{

void init();

void cleanup();

void draw_char(char c, int x, int y, Color color);

void clear_screen();

void flip();

} // io

#endif // IO_H
