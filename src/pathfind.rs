use floodfill::*;
use geometry::*;

pub fn pathfind(p0: P, p1: P, flood: &A2<i32>) -> Vec<P>
{
    let mut path: Vec<P> = Vec::new();

    if p0 == p1
    {
        // Origin and target is the same positionx
        return path;
    }

    if flood.copy_from_p(p1) == FLOOD_VALUE_UNREACHED
    {
        // No path exists
        return path;
    }

    // The path length will be equal to the flood value at the target cell
    path.resize(flood.copy_from_p(p1) as usize, P { x: -1, y: -1 });

    // We start at the target cell
    let mut p = p1;

    // Number of steps from p0 to the current position
    let mut current_dist_from_p0 = flood.copy_from_p(p);

    path[(current_dist_from_p0 - 1) as usize] = p;

    // Let's find the way back to the origin
    for _ in 0..path.len()
    {
        // Check if origin reached
        for d in OFFSETS_CARDINAL_FIRST_NO_CENTER.iter()
        {
            let adj_p = p + *d;

            if adj_p == p0
            {
                // Origin reached - thanks and goodbye!
                return path;
            }

        } // Offset loop

        // Origin not yet reached, find the next step
        for d in OFFSETS_CARDINAL_FIRST_NO_CENTER.iter()
        {
            let adj_p = p + *d;

            if !flood.is_p_inside(adj_p)
            {
                // This position is outside the map
                continue;
            }

            let adj_v = flood.copy_from_p(adj_p);

            if adj_v == FLOOD_VALUE_UNREACHED
            {
                // This position is blocked
                continue;
            }

            let cur_v = flood.copy_from_p(p);

            if adj_v >= cur_v
            {
                // This position is not closer to the origin
                continue;
            }

            // OK, this is a good step!
            p = adj_p;

            current_dist_from_p0 = adj_v;

            path[(current_dist_from_p0 - 1) as usize] = p;

            break;

        } // Offset loop

    } // Path loop

    assert!(false);

    return path;
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfind()
    {
        let p0 = P { x: 50, y: 75 };

        let dims = P { x: 100, y: 100 };

        let mut blocked: A2<bool> = A2::new_default(dims);

        *blocked.at(51, 74) = true;
        *blocked.at(51, 75) = true;
        *blocked.at(51, 76) = true;
        *blocked.at(51, 77) = true;

        let flood = floodfill(p0, None, &blocked, None);

        // Expected flood values, where:
        //
        // @ = origin (p0)
        // # = blocked positions
        //
        // 2 2 2 2 3 4 5
        //
        // 2 1 1 # 3 4 5
        //
        // 2 1 @ # 4 4 5
        //
        // 2 1 1 # 5 5 5
        //
        // 2 2 2 # 4 5 6

        // 3 3 3 3 4 5 6

        // Test a simple path
        let p1 = P { x: 52, y: 75 };

        let path = pathfind(p0, p1, &flood);

        assert_eq!(path.len(), 4);

        assert_eq!(path[0], P { x: 50, y: 74 });
        assert_eq!(path[1], P { x: 51, y: 73 });
        assert_eq!(path[2], P { x: 52, y: 74 });
        assert_eq!(path[3], p1);

        // Test that no path exists to a blocked cell
        let p1_blocked = P { x: 51, y: 77 };

        let path_blocked = pathfind(p0, p1_blocked, &flood);

        assert_eq!(path_blocked.len(), 0);
    }
}
