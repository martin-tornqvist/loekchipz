#include "io.hpp"
#include <iostream>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static SDL_Window* window_ = nullptr;
static SDL_Renderer* renderer_ = nullptr;
static TTF_Font* font_ = nullptr;

static const int screen_w = 1280;
static const int screen_h = 720;

static void cleanup_window()
{
        if (window_)
        {
                SDL_DestroyWindow(window_);
        }
}

static void init_window()
{
        cleanup_window();

        window_ = SDL_CreateWindow(
                "LoekChipz",
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                screen_w,
                screen_h,
                SDL_WINDOW_SHOWN);

        if (!window_)
        {
                std::cerr << "Failed to create SDL window: "
                          << SDL_GetError()
                          << std::endl;

                // TODO: Crash
        }
}

static void cleanup_font()
{
        if (font_)
        {
                TTF_CloseFont(font_);
        }
}

static void init_font()
{
        cleanup_font();

        const std::string font_path = "font/DejaVuSans.ttf";

        const int font_size = 16;

        font_ = TTF_OpenFont(font_path.c_str(), font_size);

        if (!font_)
        {
                std::cerr << "Failed to load load font at "
                          << "'" << font_path << "'"
                          << ":"
                          << std::endl
                          << TTF_GetError()
                          << std::endl;

                // TODO: Crash
        }
}

static void cleanup_renderer()
{
        if (renderer_)
        {
                SDL_DestroyRenderer(renderer_);
        }
}

static void init_renderer()
{
        cleanup_renderer();

        renderer_ = SDL_CreateRenderer(
                window_,
                -1,
                SDL_RENDERER_ACCELERATED);

        if (!renderer_)
        {
                std::cerr << "Failed to create SDL renderer: "
                          << SDL_GetError()
                          << std::endl;

                // TODO: Crash
        }
}

// -----------------------------------------------------------------------------
// io
// -----------------------------------------------------------------------------
namespace io
{

void init()
{
        SDL_Init(SDL_INIT_EVERYTHING);

        TTF_Init();

        init_window();

        init_renderer();

        init_font();
}

void cleanup()
{
        cleanup_font();

        cleanup_renderer();

        cleanup_window();

        TTF_Quit();

        SDL_Quit();
}

void draw_text(
        const std::string& str,
        const PxPos pos,
        const Color color)
{
        SDL_Surface* srf =
                TTF_RenderText_Blended(
                        font_,
                        str.c_str(),
                        {color.r, color.g, color.b, 0});

        if (!srf)
        {
                std::cerr << "Failed to create SDL font surface: "
                          << TTF_GetError()
                          << std::endl;

                // TODO: Crash

                return;
        }

        SDL_Rect sdl_rect;

        sdl_rect.x = pos.value.x;
        sdl_rect.y = pos.value.y;

        sdl_rect.w = srf->w;
        sdl_rect.h = srf->h;

        SDL_Texture* texture = SDL_CreateTextureFromSurface(renderer_, srf);

        SDL_RenderCopy(
                renderer_,
                texture,
                nullptr, // Cropping (unused)
                &sdl_rect);

        SDL_DestroyTexture(texture);

        SDL_FreeSurface(srf);
}

void clear_screen()
{
        SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 0);

        SDL_RenderClear(renderer_);
}

void flip()
{
        SDL_RenderPresent(renderer_);
}

void sleep(const uint32_t duration)
{
        if (duration == 1)
        {
                SDL_Delay(duration);
        }
        else // Duration longer than 1 ms
        {
                const Uint32 wait_until = SDL_GetTicks() + duration;

                while (SDL_GetTicks() < wait_until)
                {
                        SDL_PumpEvents();
                }
        }
}

} // io
