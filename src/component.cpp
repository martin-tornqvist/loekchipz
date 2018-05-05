#include "component.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// components
// -----------------------------------------------------------------------------
namespace components
{

void Gfx::draw(const P& pos) const
{
        PxPos px_pos;

        px_pos.value = pos.scaled_up(32, 32);

        io::draw_tile(
                tile_id,
                px_pos,
                color);
}

P Movable::step()
{
        const P next_pos = path.front();

        path.erase(path.begin());

        // Reached destination?
        if (path.empty())
        {
                is_moving = false;
        }

        return next_pos;
}

}
