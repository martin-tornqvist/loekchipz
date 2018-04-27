#include "io.hpp"

namespace io
{

void init()
{

}

void cleanup()
{

}

void draw_text(
        const std::string& str,
        const PxPos pos,
        const Color color)
{
        (void)str;
        (void)pos;
        (void)color;
}

void draw_text(
        const std::string& str,
        const PxPos pos,
        const Color color,
        const R& rect)
{
        (void)str;
        (void)pos;
        (void)color;
        (void)rect;
}

void draw_tile(
        const int id,
        const PxPos pos,
        const Color color)
{
        (void)id;
        (void)pos;
        (void)color;
}

void draw_rect(
        const R& rect,
        const Color& color)
{
        (void)rect;
        (void)color;
}

void draw_line(
        const PxPos& p0,
        const PxPos& p1)
{
        (void)p0;
        (void)p1;
}

void clear_screen()
{

}

void flip()
{

}

void sleep(const uint32_t duration)
{
        (void)duration;
}

} // io
