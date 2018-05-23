#include "combat.hpp"

#include <algorithm>
#include <iostream>
#include <vector>

#include "random.hpp"
#include "unit.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------

// TODO: This value should vary per unit type instead (and probably also
// modified by terrain, active spells, etc)
static const int nr_attacks_allowed = 3;

struct ArmyCombatData
{
        // Pointer to the unit vector of an army
        std::vector<Unit>* units = nullptr;

        // Indexes into the units vector above, to control the order of
        // attacking (note that the same index may appear multiple times in the
        // attack index vector)
        std::vector<size_t> units_attack_indexes = {};

        // Indexes into the units vector above, to track which units the other
        // side can attack (this is purely an optimization - it is cheaper to
        // erase elements from this vector when a unit dies, than to find a unit
        // which can be attacked in the units vector for each attack)
        std::vector<size_t> defender_index_bucket = {};
};

static std::vector<size_t> get_units_attack_indexes(
        const std::vector<Unit>& units)
{
        std::vector<size_t> units_attack_indexes;

        units_attack_indexes.reserve(
                nr_attacks_allowed * units.size());

        for (size_t i = 0; i < units.size(); ++i)
        {
                for (int attack_nr = 0;
                     attack_nr < nr_attacks_allowed;
                     ++attack_nr)
                {
                        units_attack_indexes.push_back(i);
                }
        }

        rnd::shuffle(units_attack_indexes);

        return units_attack_indexes;
}

static ArmyCombatData init_army_combat_data(std::vector<Unit>& units)
{
        ArmyCombatData data;

        data.units = &units;

        data.units_attack_indexes = get_units_attack_indexes(units);;

        data.defender_index_bucket.resize(units.size());

        std::iota(
                begin(data.defender_index_bucket),
                end(data.defender_index_bucket),
                0);

        return data;
}

static void run_attack_on_unit(
        const Unit& unit_attacking,
        Unit& unit_defending)
{
        const int dmg = rnd::range(
                1,
                unit_attacking.strength);

        if (dmg > 0)
        {
                unit_defending.strength -= dmg;
        }
}

static void run_attack_on_random_unit(
        const Unit& unit_attacking,
        std::vector<size_t>& defender_index_bucket,
        std::vector<Unit>& defender_units)
{
        const size_t defender_bucket_idx =
                rnd::idx(defender_index_bucket);

        const size_t defender_idx =
                defender_index_bucket[defender_bucket_idx];

        auto& unit_defending = defender_units[defender_idx];

        run_attack_on_unit(unit_attacking, unit_defending);

        // If killed, remove from bucket of possible defenders
        if (unit_defending.strength <= 0)
        {
                defender_index_bucket.erase(
                        begin(defender_index_bucket) +
                        defender_bucket_idx);
        }
}

static void run_attack_iteration_for_army(
        const size_t attack_iteration,
        const ArmyCombatData& attacker_army_combat_data,
        ArmyCombatData& defender_army_combat_data)
{
        const size_t nr_attacks_left =
                attacker_army_combat_data.units_attack_indexes.size();

        if (attack_iteration < nr_attacks_left)
        {
                const size_t attack_unit_idx =
                        attacker_army_combat_data
                        .units_attack_indexes[attack_iteration];

                Unit& unit_attacking =
                        (*attacker_army_combat_data.units)
                        [attack_unit_idx];

                if (unit_attacking.strength > 0)
                {
                        run_attack_on_random_unit(
                                unit_attacking,
                                defender_army_combat_data.defender_index_bucket,
                                *defender_army_combat_data.units);
                }
        }
}

static void run_attack_iterations(
        const size_t nr_attack_iterations,
        ArmyCombatData& army_1_combat_data,
        ArmyCombatData& army_2_combat_data)
{
        for (size_t i = 0; i < nr_attack_iterations; ++i)
        {
                // Randomize which army who gets to attack first
                ArmyCombatData* first_attacker;
                ArmyCombatData* first_defender;

                if (rnd::coin_toss())
                {
                        first_attacker = &army_1_combat_data;
                        first_defender = &army_2_combat_data;
                }
                else
                {
                        first_attacker = &army_2_combat_data;
                        first_defender = &army_1_combat_data;
                }

                // First attacker runs their attack iteration
                run_attack_iteration_for_army(
                        i,
                        *first_attacker,
                        *first_defender);

                if (first_defender->defender_index_bucket.empty())
                {
                        break;
                }

                // Switch and let the other side run their attack iteration
                run_attack_iteration_for_army(
                        i,
                        *first_defender,
                        *first_attacker);

                if (first_attacker->defender_index_bucket.empty())
                {
                        break;
                }
        }
}

// -----------------------------------------------------------------------------
// combat
// -----------------------------------------------------------------------------
namespace combat
{

void run_combat(
        std::vector<Unit>& army_1_units,
        std::vector<Unit>& army_2_units)
{
        std::cout << "----------------------------------------"
                  << std::endl
                  << "FIGHT!!!"
                  << std::endl
                  << "----------------------------------------"
                  << std::endl;

        auto army_1_combat_data = init_army_combat_data(army_1_units);
        auto army_2_combat_data = init_army_combat_data(army_2_units);

        const size_t nr_attack_iterations = std::max(
                army_1_combat_data.units_attack_indexes.size(),
                army_2_combat_data.units_attack_indexes.size());

        run_attack_iterations(
                nr_attack_iterations,
                army_1_combat_data,
                army_2_combat_data);

        // TODO: Used only for debug printing - to be removed
        auto get_nr_living_units = [](const std::vector<Unit>& units) {
                const int result = std::count_if(
                        begin(units),
                        end(units),
                        [](const auto& unit) {
                                return unit.strength > 0;
                        });

                return result;
        };

        std::cout << "Army 1 size: "
                  << get_nr_living_units(army_1_units)
                  << std::endl
                  << "Army 2 size: "
                  << get_nr_living_units(army_2_units)
                  << std::endl
                  << std::endl;
} // run_combat_phase

} // combat
