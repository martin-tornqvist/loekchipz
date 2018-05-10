#ifndef RANDOM_HPP
#define RANDOM_HPP

#include <random>
#include <algorithm>

struct Range
{
        Range() :
                min(-1),
                max(-1) {}

        Range(const int min, const int max) :
                min(min),
                max(max) {}

        Range(const Range& other) :
                Range(other.min, other.max) {}

        int len() const
        {
                return max - min + 1;
        }

        bool is_in_range(const int v) const
        {
                return
                        (v >= min) &&
                        (v <= max);
        }

        void set(const int min_val, const int max_val)
        {
                min = min_val;
                max = max_val;
        }

        Range& operator/=(const int v)
        {
                min /= v;
                max /= v;
                return *this;
        }

        int roll() const;

        int min, max;
};

struct Fraction
{
        Fraction() :
                num(-1),
                den(-1) {}

        Fraction(const int num, const int den) :
                num(num),
                den(den) {}

        void set(const int num, const int den)
        {
                this->num = num;
                this->den = den;
        }

        Fraction& operator=(const Fraction& other)
        {
                num = other.num;
                den = other.den;

                return *this;
        }

        bool roll() const;

        int num, den;
};

//------------------------------------------------------------------------------
// Random number generation
//------------------------------------------------------------------------------
namespace rnd
{

extern std::mt19937 rng;

void seed();

void seed(uint32_t seed);

bool coin_toss();

bool fraction(const int num, const int den);

bool one_in(const int n);

// Can be called with any range (positive or negative), v2 does not have to be
// bigger than v1.
int range(const int v1, const int v2);

// NOTE: "p" shall be within [0.0, 1.0]
int range_binom(const int v1, const int v2, const double p);

bool percent(const int pct_chance);

int weighted_choice(const std::vector<int> weights);

template <typename T>
T element(const std::vector<T>& v)
{
        const size_t idx = range(0, v.size() - 1);

        return v[idx];
}

template <typename T>
size_t idx(const std::vector<T>& v)
{
        return range(0, v.size() - 1);
}

template <typename T>
void shuffle(std::vector<T>& v)
{
        std::shuffle(begin(v), end(v), rng);
}

} // rnd

#endif // RANDOM_HPP
