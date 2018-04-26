#ifndef MAIN_MENU_HPP
#define MAIN_MENU_HPP

#include "state.hpp"
#include "gui.hpp"

class MainMenu : public State
{
public:
        MainMenu() :
                buttons_() {}
        
        ~MainMenu() {}

        StateId id() override
        {
                return StateId::main_menu;
        }

        std::vector<StateSignal> on_start() override;
        
        void draw() override;

        std::vector<StateSignal> update(const InputData& input) override;

        std::vector<Button> buttons_;
        
};

#endif // MAIN_MENU_HPP
