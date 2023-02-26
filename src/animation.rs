/* Imports */
use serde_derive::Serialize;

/* Animation step */
#[derive(Serialize)]
pub struct AnimationStep {
    from_x: usize,
    from_y: usize,
    to_x:   usize,
    to_y:   usize
}

/* JS object impls */
impl AnimationStep {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        Self { from_x, from_y, to_x, to_y }
    }

    pub fn parsed(&self) -> String {
        serde_json::to_string(self).unwrap_or(String::new())
    }
}
