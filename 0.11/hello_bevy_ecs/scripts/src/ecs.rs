use bevy_ecs::prelude::*;

use super::components::prelude::*;
use super::entities::prelude::*;
use super::scheduler::prelude::*;

pub struct Ecs {
    world: World,
    main_scheduler: MainScheduler,
}

impl Default for Ecs {
    fn default() -> Self {
        Ecs {
            world: World::default(),
            main_scheduler: MainScheduler::default(),
        }
    }
}

impl Ecs {
    pub fn spawn_some_entities(&mut self) {
        self.world
            .spawn()
            .insert(SceneEntity {
                name: "First Entity".to_string(),
            })
            .insert(Position { x: 40.0, y: 40.0 });

        self.world
            .spawn()
            .insert(SceneEntity {
                name: "Second Entity".to_string(),
            })
            .insert(Position { x: 80.0, y: 80.0 });

        self.world
            .spawn()
            .insert(SceneEntity {
                name: "Third Entity".to_string(),
            })
            .insert(Position { x: 0.0, y: 0.0 });

        self.world
            .spawn()
            .insert(PlayerEntity {})
            .insert(Position { x: 0.0, y: 0.0 });
    }

    pub fn get_entities(&mut self) -> Vec<SceneEntity> {
        let mut query = self.world.query::<&SceneEntity>();

        let mut result = Vec::new();

        for entity in query.iter(&self.world) {
            result.push(entity.clone());
        }

        return result;
    }

    pub fn update_world(&mut self) {
        self.main_scheduler.run(&mut self.world);
    }
}
