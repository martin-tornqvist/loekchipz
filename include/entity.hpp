#ifndef ENTITY_HPP
#define ENTITY_HPP

#include <memory>

#include "component.hpp"

struct Entity
{
        // Should this entity be removed? Note that most of the game systems
        // should never (need to) check this flag - they should only care about
        // the existence of relevant components. The only place were this flag
        // should matter is when removing entities marked for discarding.
        bool discard = false;

        std::unique_ptr<components::Gfx> gfx = nullptr;
        std::unique_ptr<P> pos = nullptr;
        std::unique_ptr<components::Movable> movable = nullptr;
        std::unique_ptr<components::Markable> markable = nullptr;
        std::unique_ptr<components::Terrain> terrain = nullptr;
        std::unique_ptr<components::Army> army = nullptr;
};

#endif // ENTITY_HPP
