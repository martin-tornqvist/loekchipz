#include <cassert>
#include <iostream>
#include "geometry.hpp"
#include "pathfinding.hpp"
#include "floodfill.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_pathfinding()
{
        std::cout << __func__ << std::endl;

        const P p0(50, 75);

        const P dims(100, 100);

        Array2<bool> blocked(dims);

        blocked.at(51, 74) = true;
        blocked.at(51, 75) = true;
        blocked.at(51, 76) = true;
        blocked.at(51, 77) = true;

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
        // 2 1 1 # 5 5 5
        //
        // 2 2 2 # 4 5 6
        // 3 3 3 3 4 5 6

        // Test a simple path
        const P p1 = P(52, 75);

        const auto path = pathfind(p0, p1, blocked);

        assert(path.size() == 4);
        assert(path[0] == P(50, 74));
        assert(path[1] == P(51, 73));
        assert(path[2] == P(52, 74));
        assert(path[3] == p1);

        // Test that no path exists to a blocked cell
        const P p1_blocked = P(51, 77);

        const auto path_blocked = pathfind(p0, p1_blocked, blocked);

        assert(path_blocked.size() == 0);
}


// -----------------------------------------------------------------------------
// pathfinding_tests
// -----------------------------------------------------------------------------
namespace pathfinding_tests
{

void run()
{
        test_pathfinding();
}

} // pathfinding_tests
