#include "pathfinding.hpp"
#include "floodfill.hpp"

std::vector<P> pathfind(
        const P& p0,
        const P& p1,
        const Array2<bool>& blocked)
{
        auto flood = floodfill(
                p0,
                nullptr,
                blocked,
                nullptr);

        return pathfind(p0, p1, flood);
}

std::vector<P> pathfind(
        const P& p0,
        const P& p1,
        const Array2<int>& flood)
{
        std::vector<P> path;

        if (p0 == p1)
        {
                // Origin and target is the same position
                return path;
        }

        if (flood.at(p1) == flood_value_unreached)
        {
                // No path exists
                return path;
        }

        // The path length will be equal to a the flood value at the target cell
        path.resize(flood.at(p1), P(-1, -1));

        // We start at the target cell
        auto p = p1;

        // Number of steps from p0 to the current position
        auto current_dist_from_p0 = flood.at(p);

        path[(current_dist_from_p0 - 1)] = p;

        // Find the way back to the origin
        for (unsigned int i = 0; i < path.size() - 1; ++i)
        {
                // Origin not yet reached, find the next step
                for (auto const & d : dir_utils::directions)
                {
                        auto adj_p = p + d;

                        if (!flood.is_p_inside(adj_p))
                        {
                                // This position is outside the map
                                continue;
                        }

                        auto adj_v = flood.at(adj_p);

                        if (adj_v == flood_value_unreached)
                        {
                                // This position is blocked
                                continue;
                        }

                        auto cur_v = flood.at(p);

                        if (adj_v >= cur_v)
                        {
                                // This position is not closer to the origin
                                continue;
                        }

                        // Ok, this is a good step!
                        p = adj_p;

                        current_dist_from_p0 = adj_v;

                        path[(current_dist_from_p0 - 1)] = p;

                        break;
                } // Offset loop
        } // Path loop

        return path;
}
