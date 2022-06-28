use crate::ecs::{ComponentPool, Scene};
use std::any::TypeId;

impl Scene {
    pub fn component_exists<G: 'static>(&mut self) -> bool {
        self.component_indices.contains_key(&TypeId::of::<G>())
    }
}

impl<G> ComponentPool<G> {
    pub fn iter(&self) -> impl Iterator<Item = &G> {
        let (left, _right) = self.component_array.split_at(self.num_components);

        left.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut G> {
        let (left, _right) = self.component_array.split_at_mut(self.num_components);

        left.iter_mut()
    }
}

pub trait PoolToolbox {
    fn swap_entities(&mut self, entity: usize, entity_destination: usize);
    fn swap_components(&mut self, component: usize, component_destination: usize);
    fn swap(
        &mut self,
        entity: usize,
        entity_destination: usize,
        component: usize,
        component_destination: usize,
    );
}

impl<G: 'static> PoolToolbox for ComponentPool<G> {
    fn swap_entities(&mut self, entity: usize, entity_destination: usize) {
        let component = self.sparse_array[entity] as usize;
        let component_destination = self.sparse_array[entity_destination] as usize;

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap_components(&mut self, component: usize, component_destination: usize) {
        let entity = self.packed_array[component];
        let entity_destination = self.packed_array[component_destination];

        self.swap(entity, entity_destination, component, component_destination);
    }

    fn swap(
        &mut self,
        entity: usize,
        entity_destination: usize,
        component: usize,
        component_destination: usize,
    ) {
        self.sparse_array.swap(entity, entity_destination);
        self.packed_array.swap(component, component_destination);
        self.component_array.swap(component, component_destination);
    }
}