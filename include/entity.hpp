#ifndef ENTITY_HPP
#define ENTITY_HPP

#include <memory>

#include "component.hpp"

struct Entity
{
        std::unique_ptr<components::Gfx> gfx = nullptr;
        std::unique_ptr<P> pos = nullptr;
        std::unique_ptr<components::Movable> movable = nullptr;
        std::unique_ptr<components::Markable> markable = nullptr;
        std::unique_ptr<components::Terrain> terrain = nullptr;
};

#endif // ENTITY_HPP
