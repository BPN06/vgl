use log::info;

use crate::ecs::IgnitionScene;

impl IgnitionScene {
    pub fn entity(&mut self) -> usize {
        if self.available_entities.len() == 1 {
            self.generate_new_entity()
        } else {
            let recycled_entity = self.available_entities.pop().unwrap();
            info!("Recycling old entity ({})", recycled_entity);

            recycled_entity
        }
    }

    pub fn delete(&mut self, entity: usize) {
        info!("Deleting entity ({})", entity);

        self.available_entities.push(entity);
        self.delete_entity_from_each_component_pool(entity);
    }

    /* Utility functions */

    pub fn generate_new_entity(&mut self) -> usize {
        let id = self.available_entities[0];
        self.available_entities[0] += 1;

        info!(
            "Generating new entity: ({} ->) {}",
            id, self.available_entities[0]
        );

        id
    }

    pub fn delete_entity_from_each_component_pool(&mut self, entity: usize) {
        for component_pool in self.component_pools.iter_mut() {
            component_pool.delete_entity(entity);
        }
    }
}
