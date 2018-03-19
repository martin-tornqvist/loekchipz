#include <cassert>
#include <iostream>

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_example_1()
{
        std::cout << __func__ << std::endl;

        assert(true);
}

static void test_example_2()
{
        std::cout << __func__ << std::endl;

        assert(true);
}

// -----------------------------------------------------------------------------
// example_tests
// -----------------------------------------------------------------------------
namespace example_tests
{

void run()
{
        test_example_1();
        test_example_2();
}

} // example_tests
