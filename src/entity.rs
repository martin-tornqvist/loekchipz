pub type Id = u32;

// -----------------------------------------------------------------------------
// Component entry - Component + meta data
// -----------------------------------------------------------------------------
pub struct CompEntry<T>
{
    pub ent_id: Id,
    pub data: T,
}

impl<T> CompEntry<T>
{
    pub fn new(ent_id: Id, comp: T) -> CompEntry<T>
    {
        CompEntry {
            data: comp,
            ent_id: ent_id,
        }
    }
}

// -----------------------------------------------------------------------------
// Component repo - generic storage for components of a any type
// -----------------------------------------------------------------------------
pub struct CompRepo<T>
{
    pub entries: Vec<CompEntry<T>>,
}

impl<T> CompRepo<T>
{
    pub fn new() -> CompRepo<T>
    {
        CompRepo { entries: vec![] }
    }

    pub fn add(&mut self, ent_id: Id, comp: T)
    {
        self.entries.push(
            CompEntry::new(ent_id, comp),
        );
    }

    // TODO: implement...
    // pub fn copy_for(&mut self, ent_id: Id) -> T

    #[allow(unused)]
    pub fn get_for(&mut self, ent_id: Id) -> &mut T
    {
        let mut result_idx: usize = 0;
        let mut found = false;

        for idx in 0..self.entries.len() {
            let e = &self.entries[idx];

            if e.ent_id != ent_id {
                continue;
            }

            if found {
                panic!("Component found multiple times for entity {}", ent_id);
            }

            result_idx = idx;
            found = true;
        }

        if !found {
            panic!("Could not find component for entity {}", ent_id);
        }

        return &mut self.entries[result_idx].data;
    }

    pub fn try_get_for(&mut self, ent_id: Id) -> Option<&mut T>
    {
        let mut result_idx: usize = 0;
        let mut found = false;

        for idx in 0..self.entries.len() {
            let e = &self.entries[idx];

            if e.ent_id != ent_id {
                continue;
            }

            if found {
                panic!("Component found multiple times for entity {}", ent_id);
            }

            result_idx = idx;
            found = true;
        }

        if found {
            return Some(&mut self.entries[result_idx].data);
        }

        return None;
    }

    // TODO: implement...
    // pub fn copy_all_for(&mut self, ent_id: Id) -> Vec<&mut T>

    #[allow(unused)]
    pub fn get_all_for(&mut self, ent_id: Id) -> Vec<&mut T>
    {
        self.entries
            .iter_mut()
            .filter(|entry| entry.ent_id == ent_id)
            .map(|entry| &mut entry.data)
            .collect()
    }

    #[allow(unused)]
    pub fn contains_for(&self, ent_id: Id) -> bool
    {
        return self.entries
            .iter()
            .find(|e| e.ent_id == ent_id)
            .is_some();
    }
}

// -----------------------------------------------------------------------------
// ID Generator - Creates unique entity id's
// -----------------------------------------------------------------------------
pub struct IdGenerator
{
    counter: Id,
}

impl IdGenerator
{
    pub fn new() -> IdGenerator
    {
        IdGenerator { counter: 0 }
    }

    pub fn create(&mut self) -> Id
    {
        let id = self.counter;

        self.counter += 1;

        return id;
    }
}
