#[macro_use]
extern crate gdnative;

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
    unsafe fn _ready(&mut self, _owner: gdnative::Sprite) {
        godot_warn!("_ready called");
    }

    #[export]
    unsafe fn _input(&mut self, _owner: gdnative::Sprite, event: Option<gdnative::InputEvent>) {
        
        let e = match event {
            Some(e) => { e },
            None => { return; }
        };
        
        match e.cast::<gdnative::InputEventKey>() {
            Some(v) => {
                let key_code = v.get_scancode();
                let value = v.is_pressed();

                if key_code == GlobalConstants::KEY_A {
                    self.left = value;
                }
                    
                if key_code == GlobalConstants::KEY_D {
                    self.right = value;
                }

                if key_code == GlobalConstants::KEY_W {
                    self.up = value;
                }

                if key_code == GlobalConstants::KEY_S {
                    self.down = value;
                }
            },
            _ => {}, 
        };
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Sprite, _delta: f64) {
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
