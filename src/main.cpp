#include "io.hpp"
#include "state.hpp"
#include "main_menu.hpp"

int main()
{
        io::init();

        States states;

        states.push(std::make_unique<MainMenu>());

        while (true)
        {
                if (states.is_empty())
                {
                        break;
                }

                // Start current state (if not already started)
                {
                        auto signals = states.start();

                        if (!signals.empty())
                        {
                                states.process_signals(std::move(signals));

                                continue;
                        }
                }

                // Draw
                io::clear_screen();

                states.draw();

                io::flip();

                // Update
                {
                        const InputData input = io::read_input();

                        auto signals = states.update(input);

                        if (!signals.empty())
                        {
                                states.process_signals(std::move(signals));

                                continue;
                        }
                }

                io::sleep(1);

        } // state loop

        io::cleanup();

        return 0;
}
