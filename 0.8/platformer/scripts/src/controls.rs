#[derive(PartialEq)]
pub enum Direction {
    None,
    Left,
    Right,
}

pub struct KeyboardControls {
    pub direction: Direction,
    pub jump: bool,
    pub left: bool,
    pub right: bool,
}

impl KeyboardControls {

    pub fn new() -> Self {
        KeyboardControls {
            direction: Direction::None,
            jump: false,
            left: false,
            right: false,
        }
    }

}
