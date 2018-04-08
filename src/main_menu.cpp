#include "main_menu.hpp"

#include "io.hpp"

void MainMenu::draw()
{
        io::draw_text(
                "Drawing some text over here",
                {0, 0},
                {255, 255, 255});
}

std::vector<StateSignal> MainMenu::update()
{
        io::sleep(3000);

        return {{StateSignalId::pop}};
}
