#ifndef GEOMETRY_HPP
#define GEOMETRY_HPP

#include <vector>
#include <string>

struct P;

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
// P - Position
// -----------------------------------------------------------------------------
struct P
{
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

        P with_offsets(const int x_offset, const int y_offset)
        {
                return P(x + x_offset, y + y_offset);
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

        P scaled_down(const P p) const
        {
                return P(x / p.x, y / p.y);
        }

        P scaled_down(const int x_div, const int y_div) const
        {
                return P(x / x_div, y / y_div);
        }

        P scaled_down(const int  v) const
        {
                return P(x / v, y / v);
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

// Pixel position
struct PxPos
{
        PxPos() {}

        PxPos(int x, int y) :
                value(P(x, y)) {}

        P value = P();
};

// -----------------------------------------------------------------------------
// R - Rectangle
// -----------------------------------------------------------------------------
struct R
{
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

// -----------------------------------------------------------------------------
// Array2 - Dynamic 2d array
// -----------------------------------------------------------------------------
template<typename T>
class Array2
{
public:
        Array2() :
                data_(nullptr),
                dims_(0, 0) {}

        Array2(const P& dims) :
                data_(nullptr),
                dims_()
        {
                resize(dims);
        }

        Array2(const P& dims, const T value) :
                data_(nullptr),
                dims_()
        {
                resize(dims, value);
        }

        Array2(const int w, const int h) :
                data_(nullptr),
                dims_()
        {
                resize(P(w, h));
        }

        Array2(const int w, const int h, const T value) :
                data_(nullptr),
                dims_()
        {
                resize(P(w, h), value);
        }

        Array2(const Array2<T>& other) :
                data_(nullptr),
                dims_()
        {
                resize(other.dims_);

                const size_t size = nr_elements();

                for (size_t idx = 0; idx < size; ++idx)
                {
                        data_[idx] = std::move(other.data_[idx]);
                }
        }

        Array2<T>& operator=(const Array2<T>& other)
        {
                resize(other.dims_);

                const size_t size = nr_elements();

                for (size_t idx = 0; idx < size; ++idx)
                {
                        data_[idx] = other.data_[idx];
                }

                return *this;
        }

        Array2<T>& operator=(const Array2<T>&& other)
        {
                resize(other.dims_);

                const size_t size = nr_elements();

                for (size_t idx = 0; idx < size; ++idx)
                {
                        data_[idx] = std::move(other.data_[idx]);
                }

                return *this;
        }

        ~Array2()
        {
                delete[] data_;
        }

        T* begin() const
        {
                return data_;
        }

        T* end() const
        {
                return data_ + nr_elements();
        }

        void resize(const P& dims)
        {
                dims_ = dims;

                const size_t size = nr_elements();

                delete[] data_;

                data_ = new T[size];
        }

        void resize(const int w, const int h)
        {
                resize(P(w, h));
        }

        void resize(const P& dims, const T value)
        {
                resize(dims);

                std::fill_n(data_, nr_elements(), value);
        }

        void resize(const int w, const int h, const T value)
        {
                resize(P(w, h), value);
        }

        void rotate_cw()
        {
                const P my_dims(dims());

                Array2<T> rotated(my_dims.y, my_dims.x);

                for (int x = 0; x < my_dims.x; ++x)
                {
                        for (int y = 0; y < my_dims.y; ++y)
                        {
                                const size_t my_idx = pos_to_idx(x, y);

                                rotated(my_dims.y - 1 - y, x) = data_[my_idx];
                        }
                }

                *this = rotated;
        }

        void rotate_ccw()
        {
                const P my_dims(dims());

                Array2<T> rotated(my_dims.y, my_dims.x);

                for (int x = 0; x < my_dims.x; ++x)
                {
                        for (int y = 0; y < my_dims.y; ++y)
                        {
                                const size_t my_idx = pos_to_idx(x, y);

                                rotated(y, my_dims.x - 1 - x) = data_[my_idx];
                        }
                }

                *this = rotated;
        }

        void flip_hor()
        {
                const P d(dims());

                for (int x = 0; x < d.x / 2; ++x)
                {
                        for (int y = 0; y < d.y; ++y)
                        {
                                const size_t idx_1 = pos_to_idx(x, y);
                                const size_t idx_2 = pos_to_idx(d.x - 1 - x, y);

                                std::swap(data_[idx_1], data_[idx_2]);
                        }
                }
        }

        void flip_ver()
        {
                const P d(dims());

                for (int x = 0; x < d.x; ++x)
                {
                        for (int y = 0; y < d.y / 2; ++y)
                        {
                                const size_t idx_1 = pos_to_idx(x, y);
                                const size_t idx_2 = pos_to_idx(x, d.y - 1 - y);

                                std::swap(data_[idx_1], data_[idx_2]);
                        }
                }
        }

        T& at(const P& p) const
        {
                return get_element_ref(p);
        }

        T& at(const int x, const int y) const
        {
                return get_element_ref(P(x, y));
        }

        T& operator()(const P& p) const
        {
                return get_element_ref(p);
        }

        T& operator()(const int x, const int y) const
        {
                return get_element_ref(P(x, y));
        }

        // void for_each(std::function<void(T& v)> func)
        // {
        //         const size_t size = nr_elements();

        //         for (size_t idx = 0; idx < size; ++idx)
        //         {
        //                 func(data_[idx]);
        //         }
        // }

        void clear()
        {
                delete[] data_;

                dims_.set(0, 0);
        }

        const P& dims() const
        {
                return dims_;
        }

        size_t nr_elements() const
        {
                return dims_.x * dims_.y;
        }

        bool is_p_inside(const P& p) const
        {
                const bool x_inside = (p.x >= 0) && (p.x < dims_.x);
                const bool y_inside = (p.y >= 0) && (p.y < dims_.y);

                return x_inside && y_inside;
        }

private:
        T& get_element_ref(const P& p) const
        {
                // check_pos(p);

                const size_t idx = pos_to_idx(p);

                return data_[idx];
        }

        size_t pos_to_idx(const P& p) const
        {
                return (p.x * dims_.y) + p.y;
        }

        size_t pos_to_idx(const int x, const int y) const
        {
                return pos_to_idx(P(x, y));
        }

        // void check_pos(const P& p) const
        // {
        //         if ((p.x < 0) ||
        //             (p.y < 0) ||
        //             (p.x >= dims_.x) ||
        //             (p.y >= dims_.y))
        //         {
        //                 ASSERT(false);
        //         }
        // }

        T* data_;
        P dims_;
};

#endif // GEOMETRY_HPP
