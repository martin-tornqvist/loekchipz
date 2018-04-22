#include "main_menu.hpp"

#include "io.hpp"
#include "game.hpp"

void MainMenu::draw()
{
        // TODO: Just drawing some placeholder text
        io::draw_text(
                "Drawing some text over here",
                {0, 0},
                {255, 255, 255});
}

std::vector<StateSignal> MainMenu::update()
{
        // TODO: Placeholder, pausing for a while, then popping the main menu
        // and starting the game state
        io::sleep(1000);

        return
        {
                StateSignal().set_pop(),
                StateSignal().set_push(new Game())
        };
}
