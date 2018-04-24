#include "main_menu.hpp"

#include "io.hpp"
#include "game.hpp"

void MainMenu::draw()
{
        // TODO: Just drawing some placeholder text
        io::draw_text(
                "(n) to start new game",
                {32, 32},
                {255, 255, 255});

        io::draw_text(
                "(q) to quit",
                {32, 64},
                {255, 255, 255});
}

std::vector<StateSignal> MainMenu::update(const InputData& input)
{
        if (input.c == 'n')
        {
                return {StateSignal().set_push(new Game)};
        }

        if (input.c == 'q')
        {
                return {StateSignal().set_pop()};
        }

        return {};
}
