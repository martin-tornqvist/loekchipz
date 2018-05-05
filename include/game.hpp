#ifndef GAME_HPP
#define GAME_HPP

#include <vector>

#include "state.hpp"
#include "entity.hpp"
#include "geometry.hpp"

struct Entity;

class Game : public State
{
public:
        Game() {}

        ~Game() {}

        StateId id() override
        {
                return StateId::game;
        }

        std::vector<StateSignal> on_start() override;

        void draw() override;

        std::vector<StateSignal> update(const InputData& input) override;

private:
        std::vector<Entity> actors_ = {};

        Array2<Entity> terrain_ = {};
};

#endif // GAME_HPP
