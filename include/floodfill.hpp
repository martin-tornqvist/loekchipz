#ifndef FLOODFILL_HPP
#define FLOODFILL_HPP

#include "geometry.hpp"

const int flood_value_unreached = -1;

Array2<int> floodfill(
        const P p0,
        const P* const p1,
        const Array2<bool>& blocked,
        const int* const travel_limit);

#endif // FLOODFILL_HPP
