#include <cassert>
#include <iostream>

#include "gui.hpp"
#include "io.hpp"
#include "geometry.hpp"

// -----------------------------------------------------------------------------
// Private
// -----------------------------------------------------------------------------
static void test_button_trigger()
{
        std::cout << __func__ << std::endl;

        Button button(
                "whatever",
                R(500, 1000, 600, 1337),
                {200,25,5},
                {124,14,32});

        InputData input;

        // Click inside the button
        input.mouse_pos.value.set(550, 1200);
        input.mouse_left_pressed = true;

        button.update(input);

        // The button should NOT yet be triggered (only pressed)
        assert(!button.is_triggered());

        // Release the mouse button, still on the same pixel
        input.mouse_left_pressed = false;
        input.mouse_left_released = true;

        button.update(input);

        // The button should now be triggered
        assert(button.is_triggered());
}

static void test_button_not_triggering_release_outside()
{
        std::cout << __func__ << std::endl;

        Button button(
                "whatever",
                R(500, 1000, 600, 1337),
                {100,22,211},
                {40,255,100});

        InputData input;

        // Click inside the button
        input.mouse_pos.value.set(550, 1200);
        input.mouse_left_pressed = true;

        button.update(input);

        // Release the mouse button outside the button
        input.mouse_pos.value.set(0, 0);
        input.mouse_left_pressed = false;
        input.mouse_left_released = true;

        button.update(input);

        // The button should NOT be triggered
        assert(!button.is_triggered());
}

static void test_button_trigger_move_outside_and_back()
{
        std::cout << __func__ << std::endl;

        Button button(
                "whatever",
                R(500, 1000, 600, 1337),
                {100,22,211},
                {40,255,100});

        InputData input;

        // Click inside the button
        input.mouse_pos.value.set(550, 1200);
        input.mouse_left_pressed = true;

        button.update(input);

        // Move the mouse outside, while keeping the mouse button pressed
        input.mouse_pos.value.set(0, 0);

        button.update(input);

        // Move the mouse back inside the button, and release the mouse button
        input.mouse_pos.value.set(550, 1200);
        input.mouse_left_pressed = false;
        input.mouse_left_released = true;

        button.update(input);

        // The button should now be triggered
        assert(button.is_triggered());
}

// -----------------------------------------------------------------------------
// example_tests
// -----------------------------------------------------------------------------
namespace gui_tests
{

void run()
{
        test_button_trigger();

        test_button_not_triggering_release_outside();

        test_button_trigger_move_outside_and_back();
}

} // example_tests
