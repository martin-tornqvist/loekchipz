#ifndef GEOMETRY_HPP
#define GEOMETRY_HPP

#include <vector>
#include <string>

class P;

// -----------------------------------------------------------------------------
// Direction
// -----------------------------------------------------------------------------
enum class Dir
{
    down_left   = 1,
    down        = 2,
    down_right  = 3,
    left        = 4,
    center      = 5,
    right       = 6,
    up_left     = 7,
    up          = 8,
    up_right    = 9,
    END         = 10
};

namespace dir_utils
{

extern const std::vector<P> cardinals;
extern const std::vector<P> cardinals_w_center;
extern const std::vector<P> directions;
extern const std::vector<P> directions_w_center;

Dir dir(const P& offset_values);

P offset(const Dir dir);

std::string compass_dir_name(const P& from_pos, const P& to_pos);

std::string compass_dir_name(const Dir dir);

std::string compass_dir_name(const P& offs);

} // dir_utils

// -----------------------------------------------------------------------------
// Position
// -----------------------------------------------------------------------------
class P
{
public:
        P() :
                x(0),
                y(0) {}

        P(const int x, const int y) :
                x(x),
                y(y) {}

        P(const P& p) :
                x(p.x),
                y(p.y) {}

        // Construct from a direction -> offsets (e.g. 1, -1)
        explicit P(const Dir dir);

        P& operator=(const P p)
        {
                x = p.x;
                y = p.y;

                return *this;
        }

        // Assign from a direction -> offsets (e.g. 1, -1)
        P& operator=(const Dir dir);

        P& operator+=(const P p)
        {
                x += p.x;
                y += p.y;

                return *this;
        }

        // Add a direction offset (e.g. 1, -1)
        P& operator+=(const Dir dir);

        P& operator-=(const P p)
        {
                x -= p.x;
                y -= p.y;

                return *this;
        }

        P& operator++()
        {
                ++x;
                ++y;

                return *this;
        }

        P& operator--()
        {
                --x;
                --y;

                return *this;
        }

        P operator+(const P p) const
        {
                return P(x + p.x, y + p.y);
        }

        P operator+(const int v) const
        {
                return P(x + v, y + v);
        }

        P operator+(const Dir dir) const;

        P operator-(const P p) const
        {
                return P(x - p.x, y - p.y);
        }

        P operator-(const int v) const
        {
                return P(x - v, y - v);
        }

        P with_x_offset(const int offset) const
        {
                return P(x + offset, y);
        }

        P with_y_offset(const int offset) const
        {
                return P(x, y + offset);
        }

        P scaled_up(const P p) const
        {
                return P(x * p.x, y * p.y);
        }

        P scaled_up(const int x_factor, const int y_factor) const
        {
                return P(x * x_factor, y * y_factor);
        }

        P scaled_up(const int v) const
        {
                return P(x * v, y * v);
        }

        P scaled_down(const int  v) const
        {
                return P(x / v, y / v);
        }

        P scaled_down(const P p) const
        {
                return P(x / p.x, y / p.y);
        }

        bool operator==(const P p) const
        {
                return
                        (x == p.x) &&
                        (y == p.y);
        }

        bool operator!=(const P p) const
        {
                return
                        (x != p.x) ||
                        (y != p.y);
        }

        bool operator!=(const int v) const
        {
                return
                        (x != v) ||
                        (y != v);
        }

        P signs() const
        {
                return P((x == 0) ? 0 : (x > 0) ? 1 : -1,
                         (y == 0) ? 0 : (y > 0) ? 1 : -1);
        }

        void set(const int new_x, const int new_y)
        {
                x = new_x;
                y = new_y;
        }

        void set(const P p)
        {
                x = p.x;
                y = p.y;
        }

        void swap(P& p)
        {
                P tmp(p);

                p = *this;

                set(tmp);
        }

        bool is_adjacent(const P p)
        {
                // Do not count the same position as adjacent
                if (p == *this)
                {
                        return false;
                }

                const auto d = *this - p;

                const bool x_adj = (d.x >= -1) && (d.x <= 1);
                const bool y_adj = (d.y >= -1) && (d.y <= 1);

                return x_adj && y_adj;
        }

        // NOTE: This assumes that both x and y is -1, 0, or 1
        Dir to_dir();

        int x, y;
};

struct PxPos
{
        PxPos() {}

        PxPos(int x, int y) :
                value(P(x, y)) {}

        P value = P();
};

#endif // GEOMETRY_HPP
