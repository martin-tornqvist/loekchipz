#include "geometry.hpp"

#include <cassert>
#include <cmath>

// -----------------------------------------------------------------------------
// Direction
// -----------------------------------------------------------------------------
namespace dir_utils
{

const std::vector<P> cardinals
{
        {-1,  0},
        { 1,  0},
        { 0, -1},
        { 0,  1}
};

const std::vector<P> cardinals_w_center
{
        { 0,  0},
        {-1,  0},
        { 1,  0},
        { 0, -1},
        { 0,  1}
};

const std::vector<P> directions
{
        {-1,  0},
        { 1,  0},
        { 0, -1},
        { 0,  1},
        {-1, -1},
        {-1,  1},
        { 1, -1},
        { 1,  1}
};

const std::vector<P> directions_w_center
{
        { 0,  0},
        {-1,  0},
        { 1,  0},
        { 0, -1},
        { 0,  1},
        {-1, -1},
        {-1,  1},
        { 1, -1},
        { 1,  1}
};

static const std::string compass_dir_names[3][3] =
{
        {"NW", "N", "NE"},
        {"W",  "",  "E",},
        {"SW", "S", "SE"}
};

static const double pi_db = 3.14159265;

static const double angle_45_db = 2 * pi_db / 8;

static const double angle_45_half_db = angle_45_db / 2.0;

static const double edge[4] =
{
        angle_45_half_db + (angle_45_db * 0),
        angle_45_half_db + (angle_45_db * 1),
        angle_45_half_db + (angle_45_db * 2),
        angle_45_half_db + (angle_45_db * 3)
};

Dir dir(const P& offset)
{
        assert(offset.x >= -1 &&
               offset.y >= -1 &&
               offset.x <=  1 &&
               offset.y <=  1);

        if (offset.y == -1)
        {
                return
                        offset.x == -1 ? Dir::up_left :
                        offset.x ==  0 ? Dir::up :
                        offset.x ==  1 ? Dir::up_right :
                        Dir::END;
        }

        if (offset.y == 0)
        {
                return
                        offset.x == -1 ? Dir::left :
                        offset.x ==  0 ? Dir::center :
                        offset.x ==  1 ? Dir::right :
                        Dir::END;
        }

        if (offset.y == 1)
        {
                return
                        offset.x == -1 ? Dir::down_left :
                        offset.x ==  0 ? Dir::down :
                        offset.x ==  1 ? Dir::down_right :
                        Dir::END;
        }

        return Dir::END;
}

P offset(const Dir dir)
{
        assert(dir != Dir::END);

        switch (dir)
        {
        case Dir::down_left:
                return P(-1, 1);

        case Dir::down:
                return P(0, 1);

        case Dir::down_right:
                return P(1, 1);

        case Dir::left:
                return P(-1, 0);

        case Dir::center:
                return P(0, 0);

        case Dir::right:
                return P(1, 0);

        case Dir::up_left:
                return P(-1, -1);

        case Dir::up:
                return P(0, -1);

        case Dir::up_right:
                return P(1, -1);

        case Dir::END:
                return P(0, 0);
        }

        return P(0, 0);
}

std::string compass_dir_name(const P& from_pos, const P& to_pos)
{
        std::string name;

        const P offset(to_pos - from_pos);

        const double angle_db = atan2(-offset.y, offset.x);

        if (angle_db < -edge[2] && angle_db > -edge[3])
        {
                name = "SW";
        }
        else if (angle_db <= -edge[1] && angle_db >= -edge[2])
        {
                name = "S";
        }
        else if (angle_db < -edge[0] && angle_db > -edge[1])
        {
                name = "SE";
        }
        else if (angle_db >= -edge[0] && angle_db <= edge[0])
        {
                name = "E";
        }
        else if (angle_db > edge[0] && angle_db < edge[1])
        {
                name = "NE";
        }
        else if (angle_db >= edge[1] && angle_db <= edge[2])
        {
                name = "N";
        }
        else if (angle_db > edge[2] && angle_db < edge[3])
        {
                name = "NW";
        }
        else
        {
                name = "W";
        }

        return name;
}

std::string compass_dir_name(const Dir dir)
{
        const P& o = offset(dir);

        return compass_dir_names[o.x + 1][o.y + 1];
}

std::string compass_dir_name(const P& offs)
{
        return compass_dir_names[offs.x + 1][offs.y + 1];
}

} // dir_utils

// -----------------------------------------------------------------------------
// Position
// -----------------------------------------------------------------------------
P::P(const Dir dir) :
    x(0),
    y(0)
{
    set(dir_utils::offset(dir));
}

P& P::operator=(const Dir dir)
{
    set(dir_utils::offset(dir));

    return *this;
}

P& P::operator+=(const Dir dir)
{
    *this += dir_utils::offset(dir);

    return *this;
}

P P::operator+(const Dir dir) const
{
    P result(*this);

    result += dir;

    return result;
}

Dir P::to_dir()
{
    return dir_utils::dir(*this);
}
