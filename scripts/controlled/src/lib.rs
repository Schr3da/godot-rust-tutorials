#[macro_use]
extern crate gdnative;

use gdnative::Input;
use gdnative::GlobalConstants;


#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Sprite)]
struct Controlled {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[gdnative::methods]
impl Controlled {
    fn _init(_owner: gdnative::Sprite) -> Self {
        Controlled {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: gdnative::Sprite) {
        godot_warn!("_ready called");
    }

    #[export]
    unsafe fn _input(&mut self, mut owner: gdnative::Sprite, event: Option<gdnative::InputEvent>) {
        let inputRef = Input::godot_singleton();
        
        if inputRef.is_key_pressed(GlobalConstants::KEY_A) {
            self.left = true;
        }
    
        if inputRef.is_key_pressed(GlobalConstants::KEY_D) {
            self.right = true;
        }

        if inputRef.is_key_pressed(GlobalConstants::KEY_W) {
            self.up = true;
        }

        if inputRef.is_key_pressed(GlobalConstants::KEY_S) {
            self.left = true;
        }

    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Sprite, delta: f64) {
        let mut pos = owner.get_position();
        
        if self.left {
            pos.x -= 1.0;
        }

        if self.right {
            pos.x += 1.0;
        }

        if self.up {
            pos.y -= 1.0;
        }

        if self.down {
            pos.y += 1.0;
        }

        owner.set_position(pos);
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Controlled>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
