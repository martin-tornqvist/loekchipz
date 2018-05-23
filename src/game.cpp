#include "game.hpp"

#include "combat.hpp"
#include "component.hpp"
#include "io.hpp"
#include "map.hpp"
#include "pathfinding.hpp"
#include "random.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------

// TODO: (Most of) these functions should be moved to other files eventually

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

static void run_combats(std::vector<Entity>& entities)
{
        for (size_t idx_1 = 0; idx_1 < entities.size(); ++idx_1)
        {
                for (size_t idx_2 = idx_1 + 1; idx_2 < entities.size(); ++idx_2)
                {
                        auto& e1 = entities[idx_1];
                        auto& e2 = entities[idx_2];

                        // TODO: Also check if they are enemies

                        // TODO: If additional armies are here, which are
                        // hostile to both army 1 and 2, they should wait until
                        // army 1 and 2 are completely finished (one of the
                        // armies is killed or has fled), then fight the
                        // winner. There should never be multiple combats going
                        // on at the same position.

                        if (!e1.army ||
                            !e2.army ||
                            !e1.pos ||
                            !e2.pos ||
                            (*e1.pos != *e2.pos))
                        {
                                continue;
                        }

                        // These are two armies at the same position

                        combat::run_combat(
                                e1.army->units,
                                e2.army->units);

                        // TODO: Some duplicated code here, fix
                        for (auto it = begin(e1.army->units);
                             it != end(e1.army->units); )
                        {
                                if (it->strength <= 0)
                                {
                                        it = e1.army->units.erase(it);
                                }
                                else
                                {
                                        ++it;
                                }
                        }

                        for (auto it = begin(e2.army->units);
                             it != end(e2.army->units); )
                        {
                                if (it->strength <= 0)
                                {
                                        it = e2.army->units.erase(it);
                                }
                                else
                                {
                                        ++it;
                                }
                        }

                        if (e1.army->units.empty())
                        {
                                e1.army.reset();

                                e1.discard = true;
                        }

                        if (e2.army->units.empty())
                        {
                                e2.army.reset();

                                e2.discard = true;
                        }

                }
        }
}

static void remove_entities_marked_for_discarding(std::vector<Entity>& entities)
{
        for (auto it = begin(entities); it != end(entities); )
        {
                if (it->discard)
                {
                        entities.erase(it);
                }
                else
                {
                        ++it;
                }
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

                entity.markable = std::make_unique<components::Markable>();

                entity.army = std::make_unique<components::Army>();

                entity.army->units.assign(1000, {"foo", 10});

                actors_.push_back(std::move(entity));
        }

        {
                Entity entity;

                entity.pos = std::make_unique<P>();
                entity.pos->set(0, 0);

                entity.gfx = std::make_unique<components::Gfx>();
                entity.gfx->tile_id = 2;
                entity.gfx->color = {255, 255, 255};

                entity.army = std::make_unique<components::Army>();

                entity.army->units.assign(1000, {"foo", 10});

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

        // TODO: We don't have any game turn handling yet, so for now we just
        // press "t" to execute a combat for all meeting armies
        if (input.c == 't')
        {
                run_combats(actors_);
        }

        remove_entities_marked_for_discarding(actors_);

        return {};
}
