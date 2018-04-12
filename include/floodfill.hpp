#ifndef FLOODFILL_HPP
#define FLOODFILL_HPP

#include "geometry.hpp"

const int flood_value_unreached = -1;

A2<int> floodfill(P p0, P *p1, A2<bool> blocked, int *travel_lmt);

#endif
