#ifndef COMPONENT_HPP
#define COMPONENT_HPP

#include "io.hpp"
#include "geometry.hpp"

struct GfxComponent
{
        // TOOD: Use actual (tile) graphics
        char gfx = 0;
        Color color = {0, 0, 0};
};

struct PosComponent
{
        P pos = {-1, -1};
};

#endif // COMPONENT_HPP
