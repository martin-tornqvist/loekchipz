#include "game.hpp"

#include "io.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------
std::vector<StateSignal> Game::on_start()
{
        // TODO: These are placeholder sample components
        {
                Entity entity;

                entity.pos = std::make_unique<PosComponent>();
                entity.pos->pos = P(0, 0);

                entity.gfx = std::make_unique<GfxComponent>();
                entity.gfx->gfx = '@';
                entity.gfx->color = {255, 255, 255};

                actors_.push_back(std::move(entity));
        }

        {
                Entity entity;

                entity.pos = std::make_unique<PosComponent>();
                entity.pos->pos = P(256, 128);

                entity.gfx = std::make_unique<GfxComponent>();
                entity.gfx->gfx = 'M';
                entity.gfx->color = {255, 255, 255};

                actors_.push_back(std::move(entity));
        }

        return {};
}

void Game::draw()
{
        // Draw actors
        for (auto& ent : actors_)
        {
                if (!ent.gfx ||
                    !ent.pos)
                {
                        continue;
                }

                // TODO: This is a placeholder hack until we use real graphics
                std::string str = "X";
                str[0] = ent.gfx->gfx;

                // TODO: Do an actual conversion between map and screen pixel
                // coordinates
                PxPos px_pos;
                px_pos.value = ent.pos->pos;

                io::draw_text(str, px_pos, ent.gfx->color);
        }
}

std::vector<StateSignal> Game::update(const InputData& input)
{
        if (input.c == 'q')
        {
                return {StateSignal().set_pop()};
        }

        return {};
}
