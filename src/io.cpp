#include "io.hpp"
#include <iostream>

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static SDL_Window* window;
static SDL_Renderer* renderer;
static SDL_Texture* font;

namespace g
{

const int screen_width = 1280;
const int screen_height = 720;
const int font_w = 16;
const int font_h = 16;

}

// -----------------------------------------------------------------------------
// io
// -----------------------------------------------------------------------------
namespace io
{

void init()
{

        SDL_Init(SDL_INIT_VIDEO);

        window = SDL_CreateWindow(
                "LoekChipz",
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                g::screen_width,
                g::screen_height,
                0);

        renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

        SDL_Surface* loaded_surface = IMG_Load("gfx/16x16.png");

        if (!loaded_surface)
        {
                std::cout << "Could not open "
                          << "16x16.png"
                          << std::endl
                          << SDL_GetError()
                          << std::endl;
        }

        SDL_SetColorKey(
                loaded_surface,
                SDL_TRUE,
                SDL_MapRGB(loaded_surface->format, 255, 255, 255));

        font = SDL_CreateTextureFromSurface(renderer, loaded_surface);

        SDL_FreeSurface(loaded_surface);
}

void cleanup()
{
        SDL_DestroyTexture(font);
        SDL_DestroyRenderer(renderer);
        SDL_DestroyWindow(window);
        SDL_Quit();
}

void draw_char(char c, int x, int y, Color color)
{
        SDL_Rect src_rct;

        if (c >= ' ' && c <= '/')
        {
                src_rct.y = 0;
                src_rct.x = ((int)c - 32) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;
        }
        else if (c >= '0' && c <= '?')
        {
                src_rct.y = 1*g::font_h;
                src_rct.x = ((int)c - 48) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;

        }
        else if (c >= '@' && c <= 'o')
        {
                src_rct.y = 2  *g::font_h;
                src_rct.x = ((int)c - 64) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;
        }
        else if (c >= 'p' && c <= '_')
        {
                src_rct.y = 3 * g::font_h;
                src_rct.x = ((int)c - 80) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;
        }
        else if (c >= '\'' && c <= 'o')
        {
                src_rct.y = 4 * g::font_h;
                src_rct.x = ((int)c - 96) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;
        }
        else if (c >= 'p' && c <= '~')
        {
                src_rct.y = 5 * g::font_h;
                src_rct.x = ((int)c - 112) * g::font_w;
                src_rct.w = g::font_w;
                src_rct.h = g::font_h;
        }

        SDL_Rect dst_rct;

        dst_rct.x = x*g::font_w;
        dst_rct.y = y*g::font_h;
        dst_rct.w = g::font_w;
        dst_rct.h = g::font_h;

        SDL_Rect rect = {dst_rct.x, dst_rct.y, dst_rct.w, dst_rct.h};

        SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, 255);

        SDL_RenderFillRect(renderer, &rect);

        SDL_RenderCopy(renderer, font, &src_rct, &dst_rct);
}  // draw_char

void clear()
{
        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        SDL_RenderClear(renderer);
}

void flip()
{
        SDL_RenderPresent(renderer);
}

} // io
