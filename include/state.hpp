#ifndef STATE_HPP
#define STATE_HPP

#include <memory>
#include <vector>

enum StateId
{
        main_menu,
        game,
        END
};

enum StateSignalId
{
        pop,
        push,
        // TODO: Consider adding 'remove', 'pop_until', 'pop_all', ...
};

class State;

// Returned when running states, to trigger the state handling to perform
// actions such as popping the top state or pushing a new state. This is for
// example how a menu state transitions into another screen.
// The reason for using these signals is that we don't want states to be popped
// or pushed arbitrarily at any point in the program, but only at predefined,
// predictable occasions. This makes the state handling more robust and simple.
class StateSignal
{
public:
        StateSignal() {}

        StateSignal set_pop();

        StateSignal set_push(State* const new_state);

        StateSignalId id() const
        {
                return id_;
        }

        // Usage of this depends on signal id
        StateId state_id() const
        {
                return StateId::END;
        }

        // Usage of this depends on signal id
        State* state() const
        {
                return state_;
        }

private:
        StateSignalId id_ = (StateSignalId)0;

        StateId state_id_ = StateId::END;

        State* state_ = nullptr;
};

// Container for a state + meta data
struct StateNode
{
        std::unique_ptr<State> state = nullptr;

        bool is_started = false;
};

// Base class for states
class State
{
public:
        virtual ~State() {}

        virtual StateId id() = 0;

        virtual void on_pushed() {}

        virtual std::vector<StateSignal> on_start()
        {
                return {};
        }

        virtual void on_resume() {}

        virtual void draw() {}

        virtual std::vector<StateSignal> update()
        {
                return {};
        }

        virtual void on_popped() {}
};

class States
{
public:
        States() :
                states_() {}

        std::vector<StateSignal> start();

        void draw();

        std::vector<StateSignal> update();

        void push(std::unique_ptr<State> state);

        void pop();

        bool is_empty();

        void process_signals(std::vector<StateSignal> signals);

private:
        std::vector<StateNode> states_;
};

#endif // STATE_HPP
