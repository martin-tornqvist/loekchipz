#include "game.hpp"

#include "io.hpp"
#include "map.hpp"
#include "pathfinding.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------

static void draw_path(const PxPos& pos, std::vector<P> path)
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
                        prev_pos.value.x = path[i - 1].x*32;
                        prev_pos.value.y = path[i - 1].y*32;
                }

                PxPos offset(32 / 2, 32 / 2);

                prev_pos.value.x = prev_pos.value.x + offset.value.x;
                prev_pos.value.y = prev_pos.value.y + offset.value.y;

                PxPos current_pos;
                current_pos.value.x = path[i].x*32 + offset.value.x;
                current_pos.value.y = path[i].y*32 + offset.value.y;
                io::draw_line(
                        prev_pos,
                        current_pos);
        }
}


// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------
std::vector<StateSignal> Game::on_start()
{
        // TODO: These are placeholder sample components
        {
                Entity entity;

                entity.pos = std::make_unique<PosComponent>();
                entity.pos->pos = P(156, 128);

                entity.gfx = std::make_unique<GfxComponent>();
                // entity.gfx->gfx = 'M';
                entity.gfx->tile_id = 3;
                entity.gfx->color = {255, 255, 255};

                entity.movement = std::make_unique<MovementComponent>();

                actors_.push_back(std::move(entity));
        }

        {
                Entity entity;

                entity.pos = std::make_unique<PosComponent>();
                entity.pos->pos = P(0, 0);

                entity.gfx = std::make_unique<GfxComponent>();
                entity.gfx->tile_id = 2;
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

                if (ent.movement)
                {
                        if (!ent.movement->path.empty())
                        {
                                draw_path(PxPos(px_pos), ent.movement->path);
                        }
                }


        }
}

std::vector<StateSignal> Game::update(const InputData& input)
{

        A2<bool> blocked_map(P(50, 50), false);

        for (int x = 0; x < map_.dims().x; ++x)
        {
                for (int y = 0; y < map_.dims().y; ++y)
                {
                        if (!map_.at(x, y).block)
                        {
                                continue;
                        }
                        else if(map_.at(x, y).block->block == true)
                        {
                                blocked_map.set_at(x, y, true);
                        }
                }
        }

        for (auto& e : actors_)
        {
                if (!e.movement)
                {
                        continue;
                }

                if (!e.movement->is_moving)
                {
                        continue;
                }
                // Actor is moving

                auto next_pos = e.movement->path.front();

                e.movement->path.erase(e.movement->path.begin());

                e.pos->pos = P(next_pos.x * 32, next_pos.y*32);

                // reached destination
                if (e.movement->path.empty())
                {
                        e.movement->is_moving = false;
                }


        }

        if (input.mouse_left_pressed)
        {
                // TODO: use constants for TILE_SIZE
                // TODO: Actor access is temporary.
                // TODO: We need to do something about coordinates! This is stupid.
                P selected_map_pos(input.mouse_pos.value.x / 32,
                                   input.mouse_pos.value.y / 32);

                P target_before;

                if (actors_[0].movement->path.empty())
                {
                        target_before = P(-1, -1);
                }
                else
                {
                        target_before = actors_[0].movement->path.back();
                }

                actors_[0].movement->path = pathfind(P(actors_[0].pos->pos.x/32, actors_[0].pos->pos.y/32),
                                                      selected_map_pos,
                                                      blocked_map);

                if (target_before != P(-1, -1))
                {
                        if (selected_map_pos == target_before)
                        {
                                actors_[0].movement->is_moving = true;
                        }
                }
        }

        if (input.c == 'q')
        {
                return {StateSignal().set_pop()};
        }

        return {};
}
