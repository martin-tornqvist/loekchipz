// TODO: Don't pass SDL stuff here, find a better way to read user input

use sdl2;

// -----------------------------------------------------------------------------
// State - individual states of different types implements this trait
// -----------------------------------------------------------------------------
pub trait State
{
    fn on_pushed(&mut self) -> StateFinished;

    fn on_start(&mut self) -> StateFinished;

    fn draw(&mut self);

    // TODO: Don't pass SDL stuff here, find a better way to read user input
    fn update(&mut self, sdl_context: &sdl2::Sdl) -> StateFinished;

    fn on_popped(&mut self);
}

// -----------------------------------------------------------------------------
// State action return value
// -----------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum StateFinished
{
    No,
    Yes,
}

// -----------------------------------------------------------------------------
// StateNode - stored by the State struct
// -----------------------------------------------------------------------------
struct StateNode
{
    state: Box<State>,
    is_started: bool,
}

// -----------------------------------------------------------------------------
// States - stores and manages states
// -----------------------------------------------------------------------------
pub struct States
{
    nodes: Vec<Box<StateNode>>,
}

impl States
{
    pub fn new() -> States
    {
        States { nodes: vec![] }
    }

    pub fn is_empty(&self) -> bool
    {
        self.nodes.is_empty()
    }

    fn top_idx(&self) -> usize
    {
        assert!(!self.nodes.is_empty());

        let n = self.nodes.len();

        n - 1
    }

    fn top(&mut self) -> &mut Box<StateNode>
    {
        let i = self.top_idx();

        &mut self.nodes[i]
    }

    #[allow(dead_code)]
    pub fn start(&mut self) -> StateFinished
    {
        let top = self.top();

        if top.is_started
        {
            return StateFinished::No;
        }

        let state_finished = top.state.on_start();

        top.is_started = true;

        return state_finished;
    }

    #[allow(dead_code)]
    pub fn draw(&mut self)
    {
        // TODO: We might want to enable drawing states on top of other states,
        //       if so, add a parameter such as "draw_overlayed" for the states,
        //       and iterate backwards over the state vector here, until (and
        //       including) the first state which shall NOT be drawn overlayed.
        self.top().state.draw();
    }

    // TODO: Don't pass SDL stuff here, find a better way to read user input
    #[allow(dead_code)]
    pub fn update(&mut self, sdl_context: &sdl2::Sdl) -> StateFinished
    {
        self.top().state.update(&sdl_context)
    }

    #[allow(dead_code)]
    pub fn push(&mut self, state: Box<State>)
    {
        let node = Box::new(StateNode {
            state: state,
            is_started: false,
        });

        self.nodes.push(node);

        self.top().state.on_pushed();
    }

    #[allow(dead_code)]
    pub fn pop(&mut self)
    {
        if self.is_empty()
        {
            return;
        }

        self.nodes
            .pop()
            .unwrap()
            .state
            .on_popped();
    }
} // impl States
