use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::components::prelude::*;
use crate::entities::prelude::*;

pub fn player_entity_update_system(query: Query<(&PlayerEntity, With<Position>)>) {
    godot_print!("player_entity_update_system start");

    for (player, _position) in query.iter() {
        godot_print!("{:?}", player);
    }

    godot_print!("player_entity_update_system end");
}
