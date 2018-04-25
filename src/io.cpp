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
static SDL_Texture* tiles_ = nullptr;

static const int screen_w = 1280;
static const int screen_h = 720;

static SDL_Event sdl_event_;

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


static void cleanup_tiles()
{
        if (tiles_)
        {
                SDL_DestroyTexture(tiles_);
        }
}

static void init_tiles()
{
        SDL_Surface* srf = IMG_Load("gfx/tiles.png");
    
        if(srf == NULL)
                std::cout << "Failed to open " << "tiles.png" << std::endl;
    
        SDL_SetColorKey(srf,SDL_TRUE, SDL_MapRGB(srf->format, 255, 0, 255 ) );
        
        tiles_ = SDL_CreateTextureFromSurface(renderer_, srf);

        SDL_FreeSurface(srf);
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

        init_tiles();
}

void cleanup()
{
        cleanup_tiles();
        
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

void draw_tile(
        const int id,
        const PxPos pos,
        const Color color)
{
        SDL_Rect src = {0, 0, 32, 32}; // TODO: Use const values

        // todo: fix
        src.x = 32 * id;
        
        SDL_Rect dest;
        dest.x = pos.value.x;
        dest.y = pos.value.y;
        dest.w = src.w;
        dest.h = src.h;

        SDL_SetRenderDrawColor(renderer_, color.r, color.g, color.b, 255);
        //SDL_RenderFillRect(renderer_, &dest); // Todo: should we fill in the bg? neh?
        SDL_RenderCopy(renderer_, tiles_, &src, &dest);
        
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

InputData read_input()
{
        InputData input;

        SDL_GetMouseState(
                &input.mouse_pos.value.x,
                &input.mouse_pos.value.y);

        const bool did_poll_event = SDL_PollEvent(&sdl_event_);

        if (!did_poll_event)
        {
                return input;
        }

        switch (sdl_event_.type)
        {
        case SDL_WINDOWEVENT:
        {
                switch (sdl_event_.window.event)
                {
                case SDL_WINDOWEVENT_FOCUS_GAINED:
                case SDL_WINDOWEVENT_RESTORED:
                case SDL_WINDOWEVENT_EXPOSED:
                {
                        // TODO
                }
                break;

                case SDL_WINDOWEVENT_SIZE_CHANGED:
                {
                        // TODO
                }
                break;
                } // window event switch
        }
        break; // SDL_WINDOWEVENT

        case SDL_QUIT:
        {
                // TODO
        }
        break;

        case SDL_KEYUP:
        {
                // TODO
        }
        break;

        case SDL_KEYDOWN:
        {
                // TODO
        }
        break;

        case SDL_TEXTINPUT:
        {
                const char c = sdl_event_.text.text[0];

                if (c >= 33 && c <= 126)
                {
                        input.c = c;
                }
        }
        break;

        case SDL_MOUSEBUTTONDOWN:
        {
                switch (sdl_event_.button.button)
                {
                case SDL_BUTTON_LEFT:
                        input.mouse_left_pressed = true;
                        break;

                case SDL_BUTTON_RIGHT:
                        input.mouse_right_pressed = true;
                        break;

                default:
                        break;
                }
        }
        break;

        case SDL_MOUSEBUTTONUP:
        {
                switch (sdl_event_.button.button)
                {
                case SDL_BUTTON_LEFT:
                        input.mouse_left_released = true;
                        break;

                case SDL_BUTTON_RIGHT:
                        input.mouse_right_released = true;
                        break;

                default:
                        break;
                }
        }
        break;

        default:
        {
        }
        break;

        } // event switch

        return input;
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

uint32_t get_ticks()
{
        return SDL_GetTicks();
}

} // io
