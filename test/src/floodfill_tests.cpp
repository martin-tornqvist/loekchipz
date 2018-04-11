#include <cassert>
#include <iostream>
#include "geometry.hpp"
#include "floodfill.hpp"

const int FLOOD_VALUE_UNREACHED = -1;


// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_floodfill_1()
{
        P p0;
        p0.x = 50;
        p0.y = 75;

        P dims;
        dims.x = 100;
        dims.y = 100;

        A2<bool> blocked(dims);
 
        blocked.set_at(51, 74, true);
        blocked.set_at(51, 75, true);
        blocked.set_at(51, 76, true);
        
        auto flood = flood_fill(p0, nullptr, blocked, nullptr);

        // Expected flood values, where:
        //
        // @ = origin (p0)
        // # = blocked positions
        //
        // 2 2 2 2 3 4 5
        //
        // 2 1 1 # 3 4 5
        //
        // 2 1 @ # 4 4 5
        //
        // 2 1 1 # 3 4 5
        //
        // 2 2 2 2 4 4 5
        
        // Starting position:
        assert(flood.copy_from_p(p0) == 0);
        // Blocked;
        assert(flood.copy_from(51, 75) == FLOOD_VALUE_UNREACHED);
        // Around a blocked area:
        assert(flood.copy_from(52, 75) == 4);
        assert(flood.copy_from(53, 75) == 4);
        assert(flood.copy_from(54, 75) == 5);
}


// -----------------------------------------------------------------------------
// floodfill_tests
// -----------------------------------------------------------------------------
namespace floodfill_tests
{

void run()
{
        test_floodfill_1();
}

} // floodfill_tests
