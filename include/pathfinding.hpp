#ifndef PATHFINDING_HPP
#define PATHFINDING_HPP

#include <vector>
#include "geometry.hpp"

std::vector<P> pathfind(
        const P& p0,
        const P& p1,
        const A2<int>& flood);

#endif
