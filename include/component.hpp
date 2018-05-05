#ifndef COMPONENT_HPP
#define COMPONENT_HPP

#include "io.hpp"
#include "geometry.hpp"

// -----------------------------------------------------------------------------
// components
// -----------------------------------------------------------------------------
namespace components
{

struct Gfx
{
        void draw(const P& pos) const;

        // TOOD: Use actual (tile) graphics
        char gfx = 0;
        int tile_id = 0;
        Color color = {0, 0, 0};
};

struct Terrain
{
        bool is_blocking = false;
};

struct Movable
{
        P step();

        std::vector<P> path = {};
        bool is_moving = false;
};

} // components

#endif // COMPONENT_HPP
