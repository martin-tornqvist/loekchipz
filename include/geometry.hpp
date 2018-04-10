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

// -----------------------------------------------------------------------------
// Rectangle
// -----------------------------------------------------------------------------
class R
{
public:
        R() :
                p0(P()),
                p1(P()) {}

        R(const P p0, const P p1) :
                p0(p0),
                p1(p1) {}

        R(const int x0, const int y0, const int x1, const int y1) :
                p0(P(x0, y0)),
                p1(P(x1, y1)) {}

        R(const R& r) :
                p0(r.p0),
                p1(r.p1) {}

        int w() const
        {
                return p1.x - p0.x + 1;
        }

        int h() const
        {
                return p1.y - p0.y + 1;
        }

        int area() const
        {
                return w() * h();
        }

        P dims() const
        {
                return P(w(), h());
        }

        int min_dim() const
        {
                return std::min(w(), h());
        }

        int max_dim() const
        {
                return std::max(w(), h());
        }

        P center() const
        {
                return P((p0.x + p1.x) / 2,
                         (p0.y + p1.y) / 2);
        }

        bool is_pos_inside(const P p) const
        {
                return
                        (p.x >= p0.x) &&
                        (p.y >= p0.y) &&
                        (p.x <= p1.x) &&
                        (p.y <= p1.y);
        }

        R with_offset(const P offset) const
        {
                return R(
                        p0 + P(offset.x, offset.y),
                        p1 + P(offset.x, offset.y));
        }

        R with_offset(const int x_offset, const int y_offset) const
        {
                return R(
                        p0 + P(x_offset, y_offset),
                        p1 + P(x_offset, y_offset));
        }

        R scaled_up(const int x_factor, const int y_factor) const
        {
                return R(
                        p0.scaled_up(x_factor, y_factor),
                        p1.scaled_up(x_factor, y_factor));
        }

        P p0;
        P p1;
};

#endif // GEOMETRY_HPP
