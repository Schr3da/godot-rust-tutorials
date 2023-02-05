use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::{components::prelude::Position, entities::prelude::SceneEntity};

pub fn scene_entity_position_update_system(query: Query<(&SceneEntity, With<Position>)>) {
    godot_print!("scene_entity_position_update_system start");

    for entity in query.iter() {
        godot_print!("{:?}", entity.0.name);
    }

    godot_print!("scene_entity_position_update_system start");
}
