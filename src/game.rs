use entity::*;
use io::{Io, Key};
use states::*;

#[derive(Clone)]
pub struct NameComp
{
    pub name: &'static str,
}

#[derive(Clone)]
pub struct AgeComp
{
    pub age: u32,
}

pub struct GameState
{
    names: CompRepo<NameComp>,
    ages: CompRepo<AgeComp>,
    ent_id_generator: IdGenerator,
}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            names: CompRepo::new(),
            ages: CompRepo::new(),
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
        {
            // Create a new entity id
            let ent_id = self.ent_id_generator.create();

            // Add a name component for this id
            self.names.insert(
                ent_id,
                NameComp { name: "Bosse" },
            );

            // Add an age component for this id
            self.ages.insert(
                ent_id,
                AgeComp { age: 1337 },
            );
        }

        {
            // Create another entity id
            let ent_id = self.ent_id_generator.create();

            // Add a name component for this id
            self.names.insert(
                ent_id,
                NameComp { name: "Berit" },
            );

            // Add an age component for this id
            self.ages.insert(
                ent_id,
                AgeComp { age: 9000 },
            );
        }

        // Print the name and age for both entities
        {
            for ent_id in 0..self.ent_id_generator.nr_ents() {
                let name_comp = self.names.get_for(ent_id);
                let age_comp = self.ages.get_for(ent_id);

                println!("Hi, I am entity {}.", ent_id);
                println!("My name is {}.", name_comp.name);
                println!("I am {} years old.", age_comp.age);
            }
        }

        // Print the name and age for both entities - alternative method using
        // "find_all_for" instead (this function should only be used when
        // multiple results are allowed)
        {
            for ent_id in 0..self.ent_id_generator.nr_ents() {
                let name_comps = self.names.get_all_for(ent_id);
                let age_comps = self.ages.get_all_for(ent_id);

                println!("Hi, I am entity {}.", ent_id);
                println!("My name is {}.", name_comps[0].name);
                println!("I am {} years old.", age_comps[0].age);
            }
        }

        // Update the age for entity 0
        {
            let mut age_comp = self.ages.get_for(0);

            age_comp.age += 1;
        }

        // Update the age for entity 1
        {
            let mut age_comp = self.ages.get_for(1);

            age_comp.age += 75000;
        }

        // Print names and ages for all entities
        {
            for e in &self.names.entries {
                println!("Entity '{}' has name '{}'.", e.ent_id, e.comp.name);
            }

            for e in &self.ages.entries {
                println!("Entity '{}' has age '{}'.", e.ent_id, e.comp.age);
            }
        }

        // Check if some entities has a name
        {
            // Entity 0
            if self.names.contains_for(0) {
                println!("Entity 0 has a name.");
            } else {
                println!("Entity 0 does NOT have a name.");
            }

            // Entity 42 (doesn't even exist anywhere)
            if self.names.contains_for(42) {
                println!("Entity 42 has a name.");
            } else {
                println!("Entity 42 does NOT have a name.");
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
