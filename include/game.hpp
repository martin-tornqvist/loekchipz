#ifndef GAME_HPP
#define GAME_HPP

#include "state.hpp"
#include "entity.hpp"
#include "geometry.hpp"

struct Entity;

class Game : public State
{
public:
        Game() :
                actors_(),
                map_() {}

        ~Game() {}

        StateId id() override
        {
                return StateId::game;
        }

        std::vector<StateSignal> on_start() override;

        void draw() override;

        std::vector<StateSignal> update(const InputData& input) override;

private:

        void draw_path(const PxPos& pos, std::vector<P> path);

        std::vector<Entity> actors_;

        A2<Entity> map_;
};

#endif // GAME_HPP
