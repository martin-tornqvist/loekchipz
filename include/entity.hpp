#ifndef ENTITY_HPP
#define ENTITY_HPP

#include <memory>

#include "component.hpp"

struct Entity
{
        std::unique_ptr<GfxComponent> gfx = nullptr;
        std::unique_ptr<PosComponent> pos = nullptr;
};

#endif // ENTITY_HPP
