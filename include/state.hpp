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
};

class State;

// Returned when running states, to trigger the state handling to perform
// actions such as popping the top state or pushing a new state. This is for
// example how a menu state transitions into another screen.
struct StateSignal
{
        StateSignalId id = (StateSignalId)0;

        // Use depends on signal id
        StateId state_id = StateId::END;

        // Use depends on signal id
        State* state = nullptr;
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
