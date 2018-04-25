#include "map.hpp"
#include <memory>

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
const int map_width = 50;
const int map_height = 50;

// -----------------------------------------------------------------------------
// map
// -----------------------------------------------------------------------------
namespace map
{

A2<Entity> generate_map()
{
        A2<Entity> map(P(map_width, map_height));
        for(int y = 0; y < map_height; ++y)
        {
                for(int x = 0; x < map_width; ++x)
                {
                        Entity e;
                        e.pos = std::make_unique<PosComponent>();
                        e.pos->pos = P(x*32, y*32);
                        e.gfx = std::make_unique<GfxComponent>();
                        e.gfx->tile_id = 1;
                        e.gfx->color = {255, 255, 255};
                        map.set_at(x,y, std::move(e));
                }
        }
        
        
        return map;
}

}
