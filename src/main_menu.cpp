#include "main_menu.hpp"

#include "io.hpp"
#include "game.hpp"

std::vector<StateSignal> MainMenu::on_start()
{

        {
                Button button("Start", P(32+11, 32+11), P(64, 32), {10, 213, 10}, {213, 100, 10});
                
                buttons_.push_back(std::move(button));
        }

        {
                Button button("Quit", P(32+11, 66+11), P(64, 32), {10, 213, 10}, {213, 100, 10});
                
                buttons_.push_back(std::move(button));
        }
        
        return {};
}

void MainMenu::draw()
{
        for (auto& elem : buttons_)
        {
                elem.draw();
        }    
}

std::vector<StateSignal> MainMenu::update(const InputData& input)
{
        for (auto& elem : buttons_)
        {
                elem.update(input);
                if (elem.get_text() == "Start" && elem.is_triggered())
                {
                        return {StateSignal().set_push(new Game)};
                }
                else if ( elem.get_text() == "Quit" && elem.is_triggered())
                {
                        return {StateSignal().set_pop()};
                }
        }
        
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
