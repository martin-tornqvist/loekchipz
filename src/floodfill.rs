use geometry;
use geometry::A2;
use geometry::P;
use geometry::R;

#[allow(dead_code)]
pub fn floodfill(
    p0: P,
    p1: Option<P>,
    blocked: &A2<bool>,
    travel_lmt: Option<i32>,
) -> A2<i32>
{
    let mut result: A2<i32> = A2::new_default(blocked.dims());

    // Vector of positions to travel to
    let mut positions: Vec<P> = Vec::new();

    // In the worst case we need to visit every position, reserve elements
    // for this to avoid lots of expensive resizing
    positions.reserve(result.size());

    // Instead of removing evaluated positions from the vector, we track
    // which index to try next (cheaper than erasing front elements).
    let mut next_p_idx: usize = 0;

    let mut path_exists = true;

    let bounds = R {
        p0: P { x: 1, y: 1 },
        p1: P {
            x: result.w() - 2,
            y: result.h() - 2,
        },
    };

    let mut p = p0;

    let mut v: i32 = 0;

    let mut is_at_tgt = false;

    let mut done = false;

    while !done
    {
        // Flood around the current position, and add those positions to the
        // list of positions to travel to
        for d in geometry::OFFSETS.iter()
        {
            let new_p = p + *d;

            // Not inside the bounds?
            if !bounds.is_p_inside(new_p)
            {
                continue;
            }

            // Blocked?
            if blocked.cpy_from_p(new_p)
            {
                continue;
            }

            // Already visited?
            if *result.at_p(new_p) != 0
            {
                continue;
            }

            // This is the origin?
            if new_p == p0
            {
                continue;
            }

            v = result.cpy_from_p(p);

            if travel_lmt.is_none() || (v < travel_lmt.unwrap())
            {
                *result.at_p(new_p) = v + 1;
            }

            // Reached the target?
            if p1.is_some() && (new_p == p1.unwrap())
            {
                is_at_tgt = true;

                break;
            }

            if p1.is_none() || is_at_tgt
            {
                positions.push(new_p);
            }

        } // Offset loop

        if p1.is_some()
        {
            if positions.len() == next_p_idx
            {
                path_exists = false;
            }

            if is_at_tgt || !path_exists
            {
                done = true;
            }
        }
        else if positions.len() == next_p_idx
        {
            done = true;
        }

        if travel_lmt.is_some() && (v == travel_lmt.unwrap())
        {
            done = true;
        }

        if p1.is_some() || !is_at_tgt
        {
            if positions.len() == next_p_idx
            {
                // No more positions to evaluate
                path_exists = false;
            }
            else
            // There are more positions to evaluate
            {
                p = positions[next_p_idx];

                next_p_idx += 1;
            }
        }

    } // flood_loop

    return result;
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood()
    {
        // p0: P,
        // p1: Option<P>,
        // blocked: &A2<bool>,
        // travel_lmt: Option<i32>,

        let p0 = P { x: 50, y: 75 };

        let dims = P { x: 512, y: 256 };

        let mut blocked: A2<bool> = A2::new_default(dims);

        *blocked.at(51, 74) = true;
        *blocked.at(51, 75) = true;
        *blocked.at(51, 76) = true;

        let flood = floodfill(p0, None, &blocked, None);

        // Expected flood values, where
        // @ = origin (p0)
        // # = blocked positions
        //
        // 2 2 2 2 3 4 5
        //
        // 2 1 1 # 3 4 5
        //
        // 2 1 @ # 4 4 5
        //
        // 2 1 1 # 3 4 5
        //
        // 2 2 2 2 4 4 5
        //
        assert_eq!(flood.cpy_from(50, 75), 0);
        assert_eq!(flood.cpy_from(51, 75), 0); // Blocked
        assert_eq!(flood.cpy_from(52, 75), 4); // Around a blocked area
        assert_eq!(flood.cpy_from(53, 75), 4);
        assert_eq!(flood.cpy_from(54, 75), 5);
    }
}
