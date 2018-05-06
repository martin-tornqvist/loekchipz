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
        if (e.markable && e.markable->is_marked)
        {
                e.markable->draw_mark(*e.pos);
        }

        if (e.movable && !e.movable->path.empty())
        {
                draw_path(e.movable->path, *e.pos);
        }

        e.gfx->draw(*e.pos);
}

static void step_movable(Entity& entity)
{
        if (entity.movable->is_moving)
        {
                entity.pos->set(entity.movable->step());
        }
}

static void step_all_movables(std::vector<Entity>& entities)
{
        for (auto& e : entities)
        {
                if (e.pos && e.movable)
                {
                        step_movable(e);
                }
        }
}

static void unmark_all_entities(std::vector<Entity>& entities)
{
        for (auto& e : entities)
        {
                if (e.markable)
                {
                        e.markable->is_marked = false;
                }
        }
}

static void mark_entities_at(std::vector<Entity>& entities, const P& pos)
{
        for (auto& e : entities)
        {
                if (!e.markable)
                {
                        continue;
                }

                if (e.pos && *e.pos == pos)
                {
                        e.markable->is_marked = true;
                }
        }
}

static void update_movable_for_player_move_order(
        components::Movable& movable,
        const P& pos,
        const P& target,
        const Array2<bool>& blocked)
{
        movable.is_moving = false;

        const auto old_path = movable.path;

        movable.path = pathfind(pos, target, blocked);

        if (movable.path != old_path)
        {
                // We have a new path, which must be confirmed
                // by the player - stop moving
                movable.is_moving = false;

                return;
        }

        if (movable.path.empty())
        {
                return;
        }

        // OK, we have a confirmed, non-empty path

        movable.is_moving = true;
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

                entity.markable = std::make_unique<components::Markable>();

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
        }
}

std::vector<StateSignal> Game::update(const InputData& input)
{
        Array2<bool> blocked(terrain_.dims());

        map::update_blocked(terrain_, blocked);

        // TODO: use constants for TILE_SIZE
        // TODO: We need to do something about coordinates! This is stupid.

        if (input.mouse_left_pressed)
        {
                unmark_all_entities(actors_);

                const P selected_map_pos =
                        input.mouse_pos.value.scaled_down(32, 32);

                mark_entities_at(actors_, selected_map_pos);
        }

        if (input.mouse_right_pressed)
        {
                const P selected_map_pos =
                        input.mouse_pos.value.scaled_down(32, 32);

                for (auto& actor : actors_)
                {
                        if (actor.movable &&
                            actor.markable &&
                            actor.markable->is_marked)
                        {
                                update_movable_for_player_move_order(
                                        *actor.movable,
                                        *actor.pos,
                                        selected_map_pos,
                                        blocked);
                        }
                }
        }

        step_all_movables(actors_);

        if (input.c == 'q')
        {
                return {StateSignal().set_pop()};
        }

        return {};
}
