#ifndef MAIN_MENU_HPP
#define MAIN_MENU_HPP

#include "state.hpp"

class MainMenu : public State
{
public:
        ~MainMenu() {}

        StateId id() override
        {
                return StateId::main_menu;
        }

        void draw() override;

        std::vector<StateSignal> update(const InputData& input) override;
};

#endif // MAIN_MENU_HPP
