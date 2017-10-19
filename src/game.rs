use entity::*;
use io::*;
use states::*;

// -----------------------------------------------------------------------------
// Demo component
// -----------------------------------------------------------------------------
fn operate_pos(data: &mut Data, ent: &Ent, _: &World)
{
    log!("Entity id: {}", ent.id());

    for comp in &ent.comps
    {
        match comp.data
        {
            Data::DemoDelta { dx } =>
            {
                match data
                {
                    &mut Data::DemoPos { ref mut x, .. } =>
                    {
                        *x += dx;
                    }
                    _ =>
                    {}
                }
            }
            _ =>
            {}
        }
    }
}

fn prepare_delta(data: &mut Data, ent: &Ent, _: &World)
{
    log!("Entity id: {}", ent.id());

    match data
    {
        &mut Data::DemoDelta { ref mut dx } =>
        {
            *dx = 1;
        }
        _ =>
        {}
    }
}

// -----------------------------------------------------------------------------
// Game state
// -----------------------------------------------------------------------------
pub struct GameState
{
    world: World,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState { world: World::new() }
    }
} // impl GameState

impl State for GameState
{
    fn name(&self) -> &str
    {
        return "Game";
    }

    fn on_pushed(&mut self)
    {
    }

    fn on_start(&mut self) -> Vec<StateSignal>
    {
        let mut demo_ent = Ent::new(42);

        let demo_pos_comp = Comp::new(
            1337,
            42,
            Data::DemoPos {
                is_frob: false,
                x: 0,
                nr_bars: 9001,
            },
            None,
            Some(operate_pos),
        );

        let demo_delta_comp = Comp::new(
            666,
            42,
            Data::DemoDelta { dx: 0 },
            Some(prepare_delta),
            None,
        );

        demo_ent.comps.push(demo_pos_comp);

        demo_ent.comps.push(demo_delta_comp);

        self.world.ents.push(demo_ent);

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, io: &mut Io)
    {
        io.draw_text(
            "Press 'n' to step entities\nPress 'esc' to quit",
            0,
            0,
            TextSize::Big,
            TextAnchorX::Left,
            TextAnchorY::Top,
        );
    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        if d.key == Key::Esc
        {
            return vec![StateSignal::Pop];
        }

        if d.char == 'n'
        {
            // Prepare
            for ent_idx in 0..self.world.ents.len()
            {
                for comp_idx in 0..self.world.ents[ent_idx].comps.len()
                {
                    if self.world.ents[ent_idx].comps[comp_idx]
                        .prepare
                        .is_none()
                    {
                        continue;
                    }

                    let mut data = self.world.ents[ent_idx].comps[comp_idx]
                        .data
                        .clone();

                    self.world.ents[ent_idx].comps[comp_idx]
                        .prepare
                        .unwrap()(
                        &mut data,
                        &self.world.ents[ent_idx],
                        &self.world,
                    );

                    self.world.ents[ent_idx].comps[comp_idx].data = data;
                }
            }
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {
    }
} // impl State for GameState
