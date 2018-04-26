#include "game.hpp"

#include "io.hpp"
#include "map.hpp"

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
                entity.gfx->tile_id = 2;
                entity.gfx->color = {255, 255, 255};

                actors_.push_back(std::move(entity));
        }

        {
                Entity entity;

                entity.pos = std::make_unique<PosComponent>();
                entity.pos->pos = P(256, 128);

                entity.gfx = std::make_unique<GfxComponent>();
                // entity.gfx->gfx = 'M';
                entity.gfx->tile_id = 3;
                entity.gfx->color = {255, 255, 255};

                actors_.push_back(std::move(entity));
        }

        map_ = map::generate_map();

        return {};
}

void Game::draw()
{
        // Draw map
        for (auto& ent : map_.data)
        {
                if (!ent.gfx ||
                    !ent.pos)
                {
                        continue;
                }

                int id = ent.gfx->tile_id;

                PxPos px_pos;
                px_pos.value = ent.pos->pos;

                io::draw_tile(id, px_pos, ent.gfx->color);
        }

        // Draw actors
        for (auto& ent : actors_)
        {
                if (!ent.gfx ||
                    !ent.pos)
                {
                        continue;
                }

                int id = ent.gfx->tile_id;
                // TODO: Do an actual conversion between map and screen pixel
                // coordinates
                PxPos px_pos;
                px_pos.value = ent.pos->pos;

                //io::draw_text(str, px_pos, ent.gfx->color);
                io::draw_tile(id, px_pos, ent.gfx->color);

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
