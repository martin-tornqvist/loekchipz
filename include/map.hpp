#ifndef MAP_HPP
#define MAP_HPP

#include "geometry.hpp"
#include "entity.hpp"

namespace map
{

Array2<Entity> generate();

void update_blocked(
        const Array2<Entity>& terrain,
        Array2<bool>& blocked_ref);

}

#endif // MAP_HPP
