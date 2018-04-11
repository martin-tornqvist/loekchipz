#include "gui_tests.hpp"
#include "floodfill_tests.hpp"
int main(int argc, char** argv)
{
        (void)argc;
        (void)argv;

        gui_tests::run();
        floodfill_tests::run();
}
