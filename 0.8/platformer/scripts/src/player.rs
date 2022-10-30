use crate::controls::{KeyboardControls, Direction};

use gdnative::{GlobalConstants, Vector2, InputEvent, InputEventKey, KinematicBody2D};

const GRAVITY: f32 = 1000.0;
const MOVEMENT_SPEED: f32 = 5.0;
const JUMP_SPEED: f32 = 400.0;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::KinematicBody2D)]
struct Player {
    is_jumping: bool,
    controls: KeyboardControls,
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn _init(_owner: KinematicBody2D) -> Self {
        Player {
            is_jumping: false,
            controls: KeyboardControls::new(),
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, mut owner: KinematicBody2D) {
        owner.set_physics_process(true);
    }

    #[export]
    unsafe fn _input(&mut self, _owner: KinematicBody2D, event: Option<InputEvent>) {
        let e = match event {
            Some(e) => { e },
            None => { return; }
        };
        
        match e.cast::<InputEventKey>() {
            Some(v) => {
                let key_code = v.get_scancode();
                let value = v.is_pressed();

                if key_code == GlobalConstants::KEY_A {
                    self.controls.direction = Direction::Left;
                    self.controls.left = value;
                }

                if key_code == GlobalConstants::KEY_D {
                    self.controls.direction = Direction::Right;
                    self.controls.right = value;

                }

                if self.is_jumping == false && key_code == GlobalConstants::KEY_SPACE {
                    self.is_jumping = true;
                    self.controls.jump = value;
                }
            },
            _ => {}
        };
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: KinematicBody2D, delta: f64) {
        
        if self.controls.left {
            self.velocity.x -= MOVEMENT_SPEED;
        }

        if self.controls.right {
            self.velocity.x += MOVEMENT_SPEED;
        }

        if self.controls.left == self.controls.right {
            self.velocity.x = 0.0;
        }

        if self.controls.jump && owner.is_on_floor() {
            self.velocity.y = -JUMP_SPEED;
        }

        if self.is_jumping && owner.is_on_floor() {
            self.is_jumping = false;
        }
        
        self.velocity.y += GRAVITY * delta as f32;
        self.velocity = owner.move_and_slide(self.velocity, Vector2::new(0.0, -1.0), true, 4, 0.785398, true)
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
