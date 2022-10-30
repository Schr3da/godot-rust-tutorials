mod scene;
mod ecs;

use gdnative::prelude::*;

use crate::scene::*;

fn init(handle: InitHandle) {
    handle.add_class::<HelloBevy>();
}

godot_init!(init);