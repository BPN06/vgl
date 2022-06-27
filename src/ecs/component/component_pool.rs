#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComponentPool<G> {
    pub num_components: usize,

    pub sparse_array: Vec<i32>,
    pub packed_array: Vec<usize>,
    pub component_array: Vec<G>,
}

impl<G> ComponentPool<G> {
    pub fn new_with_entity(entity: usize, component: G) -> Self {
        let mut sparse_array = Vec::with_capacity(entity + 1);
        Self::add_entity_to_sparse_array(entity, 0, &mut sparse_array);

        Self {
            num_components: 1,

            sparse_array,
            packed_array: vec![entity],
            component_array: vec![component],
        }
    }

    pub fn assign_component(&mut self, entity: usize, component: G) {
        Self::add_entity_to_sparse_array(entity, self.num_components, &mut self.sparse_array);

        self.packed_array.push(entity);
        self.component_array.push(component);
        self.num_components += 1;
    }

    /* Utility functions */

    pub fn add_entity_to_sparse_array(entity: usize, value: usize, sparse_array: &mut Vec<i32>) {
        Self::prolong_sparse_array(entity, sparse_array);
        sparse_array[entity] = value as i32;
    }

    pub fn prolong_sparse_array(entity: usize, sparse_array: &mut Vec<i32>) {
        if entity + 1 > sparse_array.len() {
            sparse_array.resize(entity + 1, -1);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &G> {
        let (left, _right) = self.component_array.split_at(self.num_components);

        left.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut G> {
        let (left, _right) = self.component_array.split_at_mut(self.num_components);

        left.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::component::component_pool::ComponentPool;
    use crate::ecs::component::component_pool_trait::ComponentPoolTrait;

    #[test]
    fn adding_an_entity_to_sparse_array_fills_the_gaps() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(5, 1, &mut sparse_array);

        assert_eq!(vec![-1, -1, 0, -1, -1, 1], sparse_array);
    }

    #[test]
    fn recycling_an_entity_in_sparse_array_does_not_resize_it_incorrectly() {
        let mut sparse_array = vec![-1, -1, 0];
        ComponentPool::<i32>::add_entity_to_sparse_array(0, 1, &mut sparse_array);

        assert_eq!(vec![1, -1, 0], sparse_array);
    }

    #[test]
    fn iterator_does_not_go_over_disabled_components() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

        component_pool.disable_entity(2);

        assert_eq!(vec![&128, &64], component_pool.iter().collect::<Vec<_>>());
    }

    #[test]
    fn iterator_goes_over_reenabled_components() {
        let mut component_pool = ComponentPool::new_with_entity(2, 32);
        component_pool.assign_component(4, 64);
        component_pool.assign_component(5, 128);

        component_pool.disable_entity(2);
        component_pool.disable_entity(5);

        component_pool.enable_entity(5);

        assert_eq!(vec![&64, &128], component_pool.iter().collect::<Vec<_>>());
    }
}
