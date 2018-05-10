#include "map.hpp"
#include <memory>
#include "random.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static const int map_w = 50;
static const int map_h = 50;

// -----------------------------------------------------------------------------
// map
// -----------------------------------------------------------------------------
namespace map
{

Array2<Entity> generate()
{
        Array2<Entity> map(P(map_w, map_h));

        for(int x = 0; x < map_w; ++x)
        {
                for(int y = 0; y < map_h; ++y)
                {
                        Entity e;

                        e.pos = std::make_unique<P>();

                        e.pos->set(x, y);

                        e.gfx = std::make_unique<components::Gfx>();

                        e.gfx->tile_id = 1;

                        e.gfx->color = {255, 255, 255};

                        e.terrain = std::make_unique<components::Terrain>();

                        if (rnd::one_in(6))
                        {
                                e.gfx->tile_id = 4;

                                e.terrain->is_blocking = true;
                        }

                        map(x, y) = std::move(e);
                }
        }

        return map;
}

void update_blocked(
        const Array2<Entity>& terrain,
        Array2<bool>& blocked_ref)
{
        for (const auto& e : terrain)
        {
                blocked_ref(*e.pos) = e.terrain->is_blocking;
        }
}

} // map
