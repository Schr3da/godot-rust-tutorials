use bevy_ecs::prelude::*;
use gdnative::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct SceneEntity {
    pub x: f32,
    pub y: f32,
}

pub struct Ecs {
    world: World,
}

impl Default for Ecs {
    fn default() -> Self {
        Ecs {
            world: World::default(),
        }
    }
}

impl Ecs {
    pub fn spawn_some_entities(&mut self) {
        self.world.spawn().insert(SceneEntity { x: 0.0, y: 0.0 });
        self.world.spawn().insert(SceneEntity { x: 40.0, y: 40.0 });
        self.world.spawn().insert(SceneEntity { x: 80.0, y: 80.0 });
    }

    pub fn get_entities(&mut self) -> Vec<SceneEntity>{
      let mut query = self.world.query::<&SceneEntity>();

      let mut result = Vec::new();


      for entity in query.iter(&self.world) {
        godot_print!("{:?}", entity);
        result.push(entity.clone());
      }

      return result;
    }
}
