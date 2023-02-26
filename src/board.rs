/* Imports */
use std::fmt::Debug;
use js_sys::Math;
use wasm_bindgen::prelude::*;
use crate::animation::AnimationStep;

/* Types */
type Brick = u16;

/* Constants */
const SIZE: usize = 4usize;

/* Main */
#[wasm_bindgen]
pub struct Board {
    /// Two-dim array of pieces
    pieces: Vec<Vec<Brick>>,
}

/* Direction for movement */
#[wasm_bindgen]
#[repr(u8)]
pub enum Direction {
    Up, Right, Down, Left
}

/* Method impls */
#[wasm_bindgen]
impl Board {
    /// Constructor
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { pieces: vec![vec![0; SIZE]; SIZE] }
    }

    /// Set one random piece
    pub fn set_random(&mut self) -> Vec<usize> {
        let mut empty_tiles = Vec::new();

        for (y, row) in self.pieces.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item == &0 {
                    empty_tiles.push((x, y));
                };
            };
        };

        if empty_tiles.is_empty() { return vec![]; };
        let index = Math::random() * empty_tiles.len() as f64;
        let coord = empty_tiles[index as usize];
        let value = if Math::random() > 0.25 { 2u16 } else { 4u16 };
        self.set(coord.0, coord.1, value);

        vec![coord.0, coord.1, value as usize]
    }

    /// Getter
    fn get(&self, x: usize, y: usize) -> Option<Brick> {
        self.pieces.get(y)?.get(x).and_then(|e| Some(*e))
    }
    
    /// Setter
    pub fn set(&mut self, x: usize, y: usize, to: Brick) {
        match match self.pieces.get_mut(y) {
            Some(e) => e,
            None => return
        }.get_mut(x) {
            Some(e) => *e = to,
            None => return
        };
    }

    /// Merge all pieces in direction
    pub fn merge_all(&mut self, direction: Direction) -> String {
        let mut steps = Vec::new();

        match direction {
            Direction::Down => {
                /* Begin from bottom to up */
                for y in (0..SIZE).rev() {
                    for x in 0..SIZE {
                        match self.merge_vertical(x, y, 1) {
                            Some(e) => steps.push(e),
                            None => ()
                        };
                    }
                }
            },
            Direction::Up => {
                /* Begin from top to bottom */
                for y in 0..SIZE {
                    for x in 0..SIZE {
                        match self.merge_vertical(x, y, -1) {
                            Some(e) => steps.push(e),
                            None => ()
                        };
                    }
                }
            },
            Direction::Left => {
                /* Begin from top to bottom */
                for y in 0..SIZE {

                    /* Left to right */
                    for x in 0..SIZE {
                        match self.merge_horizontal(x, y, -1) {
                            Some(e) => steps.push(e),
                            None => ()
                        };
                    }
                }
            },
            Direction::Right => {
                /* Begin from top to bottom */
                for y in 0..SIZE {

                    /* Right to left */
                    for x in (0..SIZE).rev() {
                        match self.merge_horizontal(x, y, 1) {
                            Some(e) => steps.push(e),
                            None => ()
                        };
                    }
                }
            },
        }

        serde_json::to_string(&steps).unwrap_or(String::new())
    }

    /// Merge vertical
    /// 
    /// `direction`: -1 = up, 1 = down
    fn merge_vertical(&mut self, x: usize, y: usize, direction: isize) -> Option<AnimationStep> {
        let original_piece = self.get(x, y).unwrap();
        if original_piece == 0 { return None; };
        let mut coords = (x, y);

        'main: while let Some(piece) = self.get(coords.0, match coords.1.checked_add_signed(direction) {
            Some(e) => e,
            None => break 'main
        }) {
            if piece == 0 {
                coords.1 = coords.1.checked_add_signed(direction)?;
            }else if piece == original_piece {
                self.set(coords.0, coords.1.checked_add_signed(direction)?, piece*2);
                self.set(x, y, 0);

                return Some(AnimationStep::new(x, y, coords.0, coords.1.checked_add_signed(direction)?, true));
            }else {
                break;
            }
        }
    
        self.set(coords.0, coords.1, original_piece);
        if coords.1 != y {
            self.set(x, y, 0);
            return Some(AnimationStep::new(x, y, coords.0, coords.1, false));
        };

        None
    }

    /// Merge horizontal - returns animation steps
    /// 
    /// `direction`: -1 = left, 1 = right
    fn merge_horizontal(&mut self, x: usize, y: usize, direction: isize) -> Option<AnimationStep> {
        let original_piece = self.get(x, y).unwrap();
        if original_piece == 0 { return None; };
        let mut coords = (x, y);

        'main: while let Some(piece) = self.get(match coords.0.checked_add_signed(direction) {
            Some(e) => e,
            None => break 'main
        }, coords.1) {
            if piece == 0 {
                coords.0 = coords.0.checked_add_signed(direction)?;
            }else if piece == original_piece {
                self.set(coords.0.checked_add_signed(direction)?, coords.1, piece*2);
                self.set(x, y, 0);

                return Some(AnimationStep::new(x, y, coords.0.checked_add_signed(direction)?, coords.1, true));
            }else {
                break;
            }
        }
    
        self.set(coords.0, coords.1, original_piece);
        if coords.0 != x {
            self.set(x, y, 0);
            return Some(AnimationStep::new(x, y, coords.0, coords.1, false));
        };

        None
    }

    /// Get all pieces as a one-dim vector (used in js-side)
    pub fn get_pieces(&self) -> Vec<Brick> {
        self.pieces.iter().flatten().map(|e| *e).collect()
    }
}

/* Debug impl */
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut end = String::new();

        for row in &self.pieces {
            for piece in row {
                end += &format!("{:^4}", piece.to_string());
            }
            end.push('\n');
            end.push('\n');
        }
        write!(f, "{}", end)
    }
}
