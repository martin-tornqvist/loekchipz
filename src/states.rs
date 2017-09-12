use io::Io;

// -----------------------------------------------------------------------------
// State - individual states of different types implements this trait
// -----------------------------------------------------------------------------
pub trait State
{
    // Returns a descriptive state name (for debug purposes)
    fn name(&self) -> &str;

    fn on_pushed(&mut self);

    fn on_start(&mut self) -> Vec<StateSignal>;

    fn on_resume(&mut self);

    fn draw(&mut self, io: &mut Io);

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>;

    fn on_popped(&mut self);
}

// -----------------------------------------------------------------------------
// State signals
// -----------------------------------------------------------------------------
pub enum StateSignal
{
    Pop,
    Push
    { state: Box<State> },
}

// -----------------------------------------------------------------------------
// StateNode - stored by the state handler
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

    pub fn process_signals(&mut self, signals: Vec<StateSignal>)
    {
        for sig in signals
        {
            match sig
            {
                StateSignal::Pop =>
                {
                    log!("Received StateSignal::Pop");
                    self.pop();
                }
                StateSignal::Push { state } =>
                {
                    log!(
                        "Received StateSignal::Push, with state '{}'",
                        state.name()
                    );
                    self.push(state);
                }
            }
        }
    }

    pub fn start(&mut self) -> Vec<StateSignal>
    {
        let top = self.top();

        if top.is_started
        {
            return Vec::new();
        }

        log!("Starting state '{}'", top.state.name());

        let signals = top.state.on_start();

        top.is_started = true;

        return signals;
    }

    pub fn draw(&mut self, io: &mut Io)
    {
        // TODO: We might want to enable drawing states on top of other states,
        //       if so, add a parameter such as "draw_overlayed" for the states,
        //       and iterate backwards over the state vector here, until (and
        //       including) the first state which shall NOT be drawn overlayed.
        self.top().state.draw(io);
    }

    pub fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let signals = self.top().state.update(io);

        return signals;
    }

    pub fn push(&mut self, state: Box<State>)
    {
        log!("Pushing state '{}'", state.name());

        let node = Box::new(StateNode {
            state: state,
            is_started: false,
        });

        self.nodes.push(node);

        self.top().state.on_pushed();
    }

    pub fn pop(&mut self)
    {
        if self.is_empty()
        {
            return;
        }

        log!(
            "Popping state '{}'",
            self.nodes.last().unwrap().state.name()
        );

        self.nodes
            .pop()
            .unwrap()
            .state
            .on_popped();

        // Resume the new top state
        if !self.nodes.is_empty() && self.top().is_started
        {
            let top = self.top();

            log!("Resuming state '{}'", top.state.name());

            top.state.on_resume();
        }
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
} // impl States
