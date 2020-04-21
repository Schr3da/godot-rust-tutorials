#[macro_use]
extern crate gdnative;

use gdnative::Vector2;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::KinematicBody2D)]
struct Player {
    default_velocity: Vector2,
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn _init(_owner: gdnative::KinematicBody2D) -> Self {
        Player {
            default_velocity: Vector2::new(0.0, -1.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: gdnative::KinematicBody2D) {
        owner.set_physics_process(true);
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: gdnative::KinematicBody2D, delta: f64) {
    
        self.velocity = Vector2::new(
            self.velocity.x * delta as f32,
            self.velocity.y - self.default_velocity.y * delta as f32,
        );
        
        match owner.move_and_collide(self.velocity, true, true, false) {
            Some(v) => {
                godot_dbg!("collision detected");
                return;
            },
            _ => { return; },
        };
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
