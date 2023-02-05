use gdnative::prelude::*;

use crate::ecs::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloBevy{
  pub ecs: Ecs,
}

#[methods]
impl HelloBevy {
    fn new(_owner: &Node) -> Self {
        HelloBevy {
          ecs: Ecs::default()
        }
    }

    #[method]
    fn _ready(&mut self) {
        self.ecs.spawn_some_entities();
        let entities = self.ecs.get_entities();
        godot_print!("Currently there are {} entities in the ecs", entities.len())
    }

    #[method]
    fn _process(&mut self, _: f64) {
      self.ecs.update_world();
    }
}

