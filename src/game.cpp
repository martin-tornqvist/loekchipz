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

void Game::draw_path(const PxPos& pos, std::vector<P> path)
{
        for (unsigned int i = 0; i < path.size(); ++i)
        {
                PxPos prev_pos;
                if (i == 0)
                {
                        prev_pos = pos;
                }
                else
                {
                        prev_pos.value = path[i - 1];
                }

                PxPos offset(32 / 2, 32 / 2);

                prev_pos.value.x = prev_pos.value.x + offset.value.x;
                prev_pos.value.y = prev_pos.value.y + offset.value.y;

                PxPos current_pos;
                current_pos.value.x = path[i].x + offset.value.x;
                current_pos.value.y = path[i].y + offset.value.y;
                io::draw_line(
                        prev_pos,
                        current_pos);
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
