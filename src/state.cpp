#include "state.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// States
// -----------------------------------------------------------------------------
std::vector< std::unique_ptr<StateSignal> > States::start()
{
        return {};
}

void States::draw()
{

}

std::vector< std::unique_ptr<StateSignal> > States::update()
{
        return {};
}

void States::push(std::unique_ptr<State> state)
{
        (void)state;
}

void States::pop()
{

}

bool States::is_empty()
{
        return states_.empty();
}

void States::process_signals(
        std::vector< std::unique_ptr<StateSignal> > signals)
{
        (void)signals;
}
