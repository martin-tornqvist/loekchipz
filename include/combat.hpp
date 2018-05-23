#ifndef COMBAT_HPP
#define COMBAT_HPP

#include <vector>

struct Unit;

namespace combat
{

void run_combat(
        std::vector<Unit>& army_1_units,
        std::vector<Unit>& army_2_units);

}

#endif // COMBAT_HPP
