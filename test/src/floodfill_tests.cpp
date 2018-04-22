#include <cassert>
#include <iostream>
#include "geometry.hpp"
#include "floodfill.hpp"


// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_floodfill_blocking()
{
        std::cout << __func__ << std::endl;

        const P p0(50, 75);

        const P dims(100, 100);

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
        assert(flood.at_p(p0) == 0);

        // Blocked;
        assert(flood.at(51, 75) == flood_value_unreached);

        // Around a blocked area:
        assert(flood.at(52, 75) == 4);
        assert(flood.at(53, 75) == 4);
        assert(flood.at(54, 75) == 5);
}

static void test_floodfill_no_path()
{
        std::cout << __func__ << std::endl;

        const P p0(0, 0);

        const P p1(15, 10);

        const P dims(100, 100);

        A2<bool> blocked(dims);
        blocked.set_at(0,1,true);
        blocked.set_at(1,1,true);
        blocked.set_at(1,0,true);

        auto flood = floodfill(p0, &p1, blocked, nullptr);

        assert(flood.at(15, 10) == -1);

}

static void test_floodfill_travel_lmt_too_low()
{
        std::cout << __func__ << std::endl;

        const P p0(0, 0);

        const P p1(15, 10);

        const P dims(100, 100);

        A2<bool> blocked(dims);

        int limit = 2;

        auto flood = floodfill(p0, &p1, blocked, &limit);

        assert(flood.at(15, 10) == -1);
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
