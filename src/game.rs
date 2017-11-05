use entity::*;
use io::{Io, Key};
// use map::Map;
use states::*;

#[derive(Clone)]
pub struct DemoCompName
{
    pub name: &'static str,
}

#[derive(Clone)]
pub struct DemoCompAge
{
    pub age: u32,
}

pub struct GameState
{
    world: World,
    ent_id_generator: IdGenerator,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            world: World::new(),
            ent_id_generator: IdGenerator::new(),
        }
    }
}

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
        // Create a new entity id
        let ent_id = self.ent_id_generator.create();

        // Add a name component for this id to the world
        self.world.insert(
            ent_id,
            DemoCompName { name: "Bosse" },
        );

        // Add an age component for this id to the world
        self.world.insert(
            ent_id,
            DemoCompAge { age: 1337 },
        );

        // Print the name and age for the entity
        {
            let names = self.world.find_for::<DemoCompName>(
                ent_id,
            );

            let ages = self.world.find_for::<DemoCompAge>(
                ent_id,
            );

            println!("Hi, I am entity {}!", ent_id);

            println!("My name is {}.", names[0].name);

            println!("I am {} years old.", ages[0].age);
        }

        // Update the age
        {
            let mut ages = self.world.find_for_mut::<DemoCompAge>(
                ent_id,
            );

            ages[0].age += 1;
        }

        // Print the new age
        {
            let ages = self.world.find_for::<DemoCompAge>(
                ent_id,
            );

            println!("Now I am {} years old!", ages[0].age);
        }

        // Print names for all entities
        {
            let names = self.world.find::<DemoCompName>();

            for name in names {
                println!(
                    "This is name '{}', for entity '{}'.",
                    name.comp.name,
                    name.ent_id
                );
            }
        }

        return Vec::new();
    }

    fn on_resume(&mut self)
    {
    }

    fn draw(&mut self, _: &mut Io)
    {

    }

    fn update(&mut self, io: &mut Io) -> Vec<StateSignal>
    {
        let d = io.read();

        if d.key == Key::Esc {
            return vec![StateSignal::Pop];
        }

        return Vec::new();
    }

    fn on_popped(&mut self)
    {

    }
} // impl State for GameState
