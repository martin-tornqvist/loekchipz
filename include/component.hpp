#ifndef COMPONENT_HPP
#define COMPONENT_HPP

#include "io.hpp"
#include "geometry.hpp"

struct GfxComponent
{
        // TOOD: Use actual (tile) graphics
        char gfx = 0;
        int tile_id = 0;
        Color color = {0, 0, 0};
};

struct PosComponent
{
        P pos = {-1, -1};
};

struct BlockComponent
{
        bool block = false;
};

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Weffc++"
struct MovementComponent
{
        std::vector<P> path;
        bool is_moving = false;
};
#pragma GCC diagnostic pop

#endif // COMPONENT_HPP
