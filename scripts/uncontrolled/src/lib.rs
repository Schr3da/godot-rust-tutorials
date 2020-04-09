
#[macro_use]
extern crate gdnative;

use gdnative::init::property::{EnumHint, IntHint, StringHint};

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Sprite)]
struct Uncontrolled {}

#[gdnative::methods]
impl Uncontrolled {
    fn _init(_owner: gdnative::Sprite) -> Self {
        Uncontrolled {}
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: gdnative::Sprite) {
        godot_warn!("_ready called");
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Sprite, delta: f64) {
        let mut pos = owner.get_position();

        if (pos.x > 100.0) {
            pos.x -= 1.0;
        }

        owner.set_position(pos);
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Uncontrolled>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
