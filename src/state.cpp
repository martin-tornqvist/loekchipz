#include "state.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// StateSignal
// -----------------------------------------------------------------------------
StateSignal StateSignal::set_pop()
{
        id_ = StateSignalId::pop;

        return *this;
}

StateSignal StateSignal::set_push(State* const new_state)
{
        id_ = StateSignalId::push;
        state_ = new_state;

        return *this;
}

// -----------------------------------------------------------------------------
// States
// -----------------------------------------------------------------------------
std::vector<StateSignal> States::start()
{
        auto& top = states_.back();

        if (top.is_started)
        {
                return {};
        }

        auto signals = top.state->on_start();

        top.is_started = true;

        return signals;
}

void States::draw()
{
        // TODO: We might want to enable drawing states on top of other states,
        // if so, add a parameter such as "draw_overlayed" for the states, and
        // iterate backwards over the state vector here, until (and including)
        // the first state which shall NOT be drawn overlayed.

        states_.back().state->draw();
}

std::vector<StateSignal> States::update(const InputData& input)
{
        auto signals = states_.back().state->update(input);

        return signals;
}

void States::push(std::unique_ptr<State> state)
{
        StateNode new_node;

        new_node.is_started = false;

        new_node.state = std::move(state);

        states_.push_back(std::move(new_node));
}

void States::pop()
{
        if (is_empty())
        {
                return;
        }

        states_.back().state->on_popped();

        states_.pop_back();

        if (is_empty())
        {
                return;
        }

        // Resume or start the new top node (if any)
        auto& new_top_node = states_.back();

        if (new_top_node.is_started)
        {
                new_top_node.state->on_resume();
        }
        else // Not already started
        {
                start();
        }
}

bool States::is_empty()
{
        return states_.empty();
}

void States::process_signals(std::vector<StateSignal> signals)
{
        for (const auto& sig : signals)
        {
                switch (sig.id())
                {
                case StateSignalId::pop:
                        pop();
                        break;

                case StateSignalId::push:
                        auto new_state = std::unique_ptr<State>(sig.state());
                        push(std::move(new_state));
                        break;
                }
        }
}
