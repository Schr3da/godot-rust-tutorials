mod scene;
mod ecs;
mod components;
mod entities;
mod systems;
mod scheduler;

use gdnative::prelude::*;

use crate::scene::*;

fn init(handle: InitHandle) {
    handle.add_class::<HelloBevy>();
}

godot_init!(init);