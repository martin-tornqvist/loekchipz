#include "game.hpp"

#include "io.hpp"
#include "map.hpp"
#include "pathfinding.hpp"
#include "component.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void draw_path(std::vector<P> path, const P& origin)
{
        for (size_t i = 0; i < path.size(); ++i)
        {
                P prev_pos;

                if (i == 0)
                {
                        prev_pos = origin;
                }
                else
                {
                        prev_pos = path[i - 1];
                }

                // TODO: Some repetition of scaling and offsetting here...
                PxPos prev_px_pos;

                prev_px_pos.value =
                        prev_pos.scaled_up(32, 32)
                        .with_x_offset(32 / 2)
                        .with_y_offset(32 / 2);

                const P& current_pos = path[i];

                PxPos current_px_pos;

                current_px_pos.value =
                        current_pos.scaled_up(32, 32)
                        .with_x_offset(32 / 2)
                        .with_y_offset(32 / 2);

                io::draw_line(prev_px_pos, current_px_pos);
        }
}

static void draw_entity(const Entity& e)
{
        e.gfx->draw(*e.pos);
}

static void step_movable(Entity& entity)
{
        if (entity.pos &&
            entity.movable &&
            entity.movable->is_moving)
        {
                entity.pos->set(entity.movable->step());
        }
}

static void step_all_movables(std::vector<Entity>& entities)
{
        for (auto& e : entities)
        {
                step_movable(e);
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

                entity.pos = std::make_unique<P>();
                entity.pos->set(8, 8);

                entity.gfx = std::make_unique<components::Gfx>();
                // entity.gfx->gfx = 'M';
                entity.gfx->tile_id = 3;
                entity.gfx->color = {255, 255, 255};

                entity.movable = std::make_unique<components::Movable>();

                actors_.push_back(std::move(entity));
        }

        {
                Entity entity;

                entity.pos = std::make_unique<P>();
                entity.pos->set(0, 0);

                entity.gfx = std::make_unique<components::Gfx>();
                entity.gfx->tile_id = 2;
                entity.gfx->color = {255, 255, 255};

                actors_.push_back(std::move(entity));
        }

        terrain_ = map::generate();

        return {};
}

void Game::draw()
{
        for (auto& e : terrain_)
        {
                draw_entity(e);
        }

        for (auto& e : actors_)
        {
                draw_entity(e);

                if (e.movable && !e.movable->path.empty())
                {
                        draw_path(e.movable->path, *e.pos);
                }
        }
}

std::vector<StateSignal> Game::update(const InputData& input)
{
        Array2<bool> blocked(terrain_.dims());

        map::update_blocked(terrain_, blocked);

        if (input.mouse_left_pressed)
        {
                // TODO: use constants for TILE_SIZE
                // TODO: Actor access is temporary.
                // TODO: We need to do something about coordinates! This is stupid.

                auto& actor = actors_[0];

                if (actor.movable)
                {
                        const P selected_map_pos =
                                input.mouse_pos.value.scaled_down(32, 32);

                        if (!actor.movable->path.empty())
                        {
                                const P target_before =
                                        actor.movable->path.back();

                                if (selected_map_pos == target_before)
                                {
                                        actor.movable->is_moving = true;
                                }
                        }

                        actor.movable->path =
                                pathfind(*actor.pos, selected_map_pos, blocked);
                }
        }

        step_all_movables(actors_);

        if (input.c == 'q')
        {
                return {StateSignal().set_pop()};
        }

        return {};
}
