#include "io.hpp"
#include "state.hpp"
#include "main_menu.hpp"

int main()
{
        io::init();

        States states;

        states.push(std::make_unique<MainMenu>());

        // This is the "poor man's framerate limiter"!
        const uint32_t min_render_interval_ms = 20;

        uint32_t next_rendering_ms = 0;

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

                // To avoid 100% CPU usage from the game logic
                io::sleep(1);

                // Is it time to run rendering yet?
                const uint32_t current_ticks_ms = io::get_ticks();

                if (current_ticks_ms >= next_rendering_ms)
                {
                        // Draw
                        io::clear_screen();

                        states.draw();

                        io::flip();

                        next_rendering_ms =
                                current_ticks_ms +
                                min_render_interval_ms;
                }

        } // state loop

        io::cleanup();

        return 0;
}
