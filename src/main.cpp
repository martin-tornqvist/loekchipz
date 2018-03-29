#include "io.hpp"
#include "state.hpp"

int main()
{
        io::init();

        States states;

        while (true)
        {
                if (states.is_empty())
                {
                        break;
                }

                {
                        auto signals = states.start();

                        if (!signals.empty())
                        {
                                states.process_signals(std::move(signals));

                                continue;
                        }
                }

                io::clear_screen();

                states.draw();

                io::flip();

                {
                        auto signals = states.update();

                        if (!signals.empty())
                        {
                                states.process_signals(std::move(signals));

                                continue;
                        }
                }

                // TODO:
                // io::sleep(1);

        } // state loop

        io::cleanup();

        return 0;
}
