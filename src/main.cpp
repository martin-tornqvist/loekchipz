#include "io.hpp"

int main(){

        io::init();

        for (int i = 0; i < 1000; ++i)
        {
                io::clear();

                io::draw_char('@', 10, 10, Color{100, 30, 30});

                io::flip();
        }

        io::cleanup();

        return 0;
}
