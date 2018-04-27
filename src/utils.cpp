#include "utils.hpp"
#include <random>
namespace utils
{
std::random_device rd;
std::mt19937 mt(rd());

int get_random_int(int low, int high)
{
        std::uniform_real_distribution<double> dist(low, high);
        return dist(mt);
}

}
