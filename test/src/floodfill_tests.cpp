#include <cassert>
#include <iostream>
#include "geometry.hpp"
#include "floodfill.hpp"


// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_floodfill_blocking()
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
        
        auto flood = floodfill(p0, nullptr, blocked, nullptr);

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
        assert(flood.copy_from(51, 75) == flood_value_unreached);
        // Around a blocked area:
        assert(flood.copy_from(52, 75) == 4);
        assert(flood.copy_from(53, 75) == 4);
        assert(flood.copy_from(54, 75) == 5);
}

static void test_floodfill_no_path()
{
        P p0;
        p0.x = 0;
        p0.y = 0;

        P *p1 = new P(15, 10);
        
        P dims;
        dims.x = 100;
        dims.y = 100;

        A2<bool> blocked(dims);
        blocked.set_at(0,1,true);
        blocked.set_at(1,1,true);
        blocked.set_at(1,0,true);

        auto flood = floodfill(p0, p1, blocked, nullptr);

        assert(flood.copy_from(15, 10) == -1);
        
}

static void test_floodfill_travel_lmt_too_low()
{
        P p0;
        p0.x = 0;
        p0.y = 0;

        P *p1 = new P(15, 10);
        
        P dims;
        dims.x = 100;
        dims.y = 100;

        A2<bool> blocked(dims);

        int *lmt = new int(2);
        
        auto flood = floodfill(p0, p1, blocked, lmt);

        assert(flood.copy_from(15, 10) == -1);
}

// -----------------------------------------------------------------------------
// floodfill_tests
// -----------------------------------------------------------------------------
namespace floodfill_tests
{

void run()
{
        test_floodfill_blocking();
        test_floodfill_no_path();
        test_floodfill_travel_lmt_too_low();
}

} // floodfill_tests
