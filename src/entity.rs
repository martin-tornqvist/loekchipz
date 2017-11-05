use std::any::{Any, TypeId};
use std::collections::hash_map::HashMap;
use std::ops::Index;

pub type Id = u32;

// A relationship between an entity and a component
#[derive(Clone)]
pub struct EntMeta<T>
{
    pub ent_id: Id,
    pub comp: T,
}

// Creates unique entity id's
pub struct IdGenerator
{
    id_counter: Id,
}

impl IdGenerator
{
    pub fn new() -> IdGenerator
    {
        IdGenerator { id_counter: 0 }
    }

    pub fn create(&mut self) -> Id
    {
        let id = self.id_counter;

        self.id_counter += 1;

        return id;
    }
}

// Container for entities and components
pub struct World
{
    comps: HashMap<TypeId, Box<Any>>,
    ents: HashMap<Id, HashMap<TypeId, Box<Any>>>,
}

impl World
{
    pub fn new() -> World
    {
        World {
            comps: HashMap::new(),
            ents: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert<T>(&mut self, id: Id, comp: T)
    where
        T: 'static,
    {
        let comps_vec = self.comps
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Vec::new() as Vec<EntMeta<T>>))
            .downcast_mut::<Vec<EntMeta<T>>>()
            .expect("downcast to Vec<EntMeta<T>");

        let em = EntMeta {
            ent_id: id,
            comp: comp,
        };

        comps_vec.push(em);

        let ent_comps_map = self.ents.entry(id).or_insert(
            HashMap::new(),
        );

        let ent_comps_vec = ent_comps_map
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Vec::new() as Vec<*mut T>))
            .downcast_mut::<Vec<*mut T>>()
            .expect("downcase to Vec<*mut T>");

        let v = &mut comps_vec
            .last_mut()
            .expect("last component to exist")
            .comp;
        ent_comps_vec.push(v);
    }

    #[allow(dead_code)]
    pub fn find<T>(&self) -> Vec<EntMeta<T>>
    where
        T: Clone + 'static,
    {
        self.comps
            .get(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_ref::<Vec<EntMeta<T>>>()
            .expect("downcast to Vec<(Id, T)>")
            .iter()
            .map(|meta| meta.clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn find_mut<T>(&mut self) -> Vec<&mut EntMeta<T>>
    where
        T: 'static,
    {
        self.comps
            .get_mut(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_mut::<Vec<EntMeta<T>>>()
            .expect("downcast to Vec<(Id, &T)>")
            .iter_mut()
            .collect()
    }

    #[allow(dead_code)]
    pub fn contains<T>(&self) -> bool
    where
        T: 'static,
    {
        self.comps.contains_key(
            &TypeId::of::<T>(),
        )
    }

    #[allow(dead_code)]
    pub fn remove<T>(&mut self) -> bool
    where
        T: 'static,
    {
        let result = self.comps
            .remove(&TypeId::of::<T>())
            .is_some();

        for (_, v) in self.ents.iter_mut() {
            if v.contains_key(&TypeId::of::<T>()) {
                let result2 = v.remove(&TypeId::of::<T>()).is_some();
                debug_assert_eq!(result, result2);
            }
        }

        result
    }

    #[allow(dead_code)]
    pub fn find_for<T>(&self, id: Id) -> Vec<T>
    where
        T: Clone + 'static,
    {
        self.ents
            .get(&id)
            .expect("entity to exist")
            .get(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_ref::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .iter()
            .map(|&c| unsafe { &*c }.clone())
            .collect()
    }

    #[allow(dead_code)]
    pub fn find_for_mut<T>(&mut self, id: Id) -> Vec<&mut T>
    where
        T: 'static,
    {
        self.ents
            .get_mut(&id)
            .expect("entity to exist")
            .get_mut(&TypeId::of::<T>())
            .expect("components for T to exist")
            .downcast_mut::<Vec<*mut T>>()
            .expect("downcast to Vec<*mut T>")
            .iter_mut()
            .map(|&mut c| unsafe { &mut *c })
            .collect()
    }

    #[allow(dead_code)]
    pub fn get<T>(&self, id: Id) -> T
    where
        T: Clone + 'static,
    {
        unsafe {
            &**self.ents
                .get(&id)
                .expect("entity to exist")
                .get(&TypeId::of::<T>())
                .expect("components for T to exist")
                .downcast_ref::<Vec<*mut T>>()
                .expect("downcast to Vec<*mut T>")
                .index(0)
        }.clone()
    }

    #[allow(dead_code)]
    pub fn get_mut<T>(&mut self, id: Id) -> &mut T
    where
        T: Clone + 'static,
    {
        unsafe {
            &mut **self.ents
                .get_mut(&id)
                .expect("entity to exist")
                .get_mut(&TypeId::of::<T>())
                .expect("components for T to exist")
                .downcast_mut::<Vec<*mut T>>()
                .expect("downcast to Vec<*mut T>")
                .index(0)
        }
    }

    #[allow(dead_code)]
    pub fn find_ents_for_type<T>(&self) -> Vec<Id>
    where
        T: 'static,
    {
        self.ents
            .iter()
            .filter(|pair| pair.1.contains_key(&TypeId::of::<T>()))
            .map(|pair| *pair.0)
            .collect()

    }
}
