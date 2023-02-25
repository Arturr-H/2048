use std::fmt::Debug;

/* Imports */
use rand::{self, Rng};

/* Types */
type Brick = u16;

/* Main */
pub struct Board<const N: usize> {
    /// Two-dim array of pieces
    pieces: [[Brick; N]; N],
}

/* Direction for movement */
pub enum Direction {
    Up, Right, Down, Left
}

/* Method impls */
impl<const N: usize> Board<N> {
    /// Constructor
    pub fn new() -> Self {
        Self { pieces: [[0; N]; N] }
    }
    
    /// Getter
    pub fn get(&self, x: usize, y: usize) -> Option<&Brick> {
        self.pieces.get(y)?.get(x)
    }
    
    /// Setter
    pub fn set(&mut self, x: usize, y: usize, to: Brick) -> Option<()> {
        *self.pieces.get_mut(y)?.get_mut(x)? = to;
        Some(())
    }

    /// Merge vertical
    /// 
    /// `direction`: -1 = up, 1 = down
    fn merge_vertical(&mut self, x: usize, y: usize, direction: isize) -> Option<()> {
        let original_piece = self.get(x, y).unwrap();
        if original_piece == &0 { return None; };
        let mut coords = (x, y);

        'main: while let Some(piece) = self.get(coords.0, match coords.1.checked_add_signed(direction) {
            Some(e) => e,
            None => break 'main
        }) {
            if piece == &0 {
                coords.1 = coords.1.checked_add_signed(direction)?;
            }else if piece == original_piece {
                self.set(coords.0, coords.1.checked_add_signed(direction)?, piece*2);
                self.set(x, y, 0);
                return None;
            }else {
                break;
            }
        }
    
        self.set(coords.0, coords.1, *original_piece);
        if coords.1 != y { self.set(x, y, 0); };
        Some(())
    }

    /// Merge horizontal
    /// 
    /// `direction`: -1 = left, 1 = right
    fn merge_horizontal(&mut self, x: usize, y: usize, direction: isize) -> Option<()> {
        let original_piece = self.get(x, y).unwrap();
        if original_piece == &0 { return None; };
        let mut coords = (x, y);

        'main: while let Some(piece) = self.get(match coords.0.checked_add_signed(direction) {
            Some(e) => e,
            None => break 'main
        }, coords.1) {
            if piece == &0 {
                coords.0 = coords.0.checked_add_signed(direction)?;
            }else if piece == original_piece {
                self.set(coords.0.checked_add_signed(direction)?, coords.1, piece*2);
                self.set(x, y, 0);
                return None;
            }else {
                break;
            }
        }
    
        self.set(coords.0, coords.1, *original_piece);
        if coords.0 != x { self.set(x, y, 0); };
        Some(())
    }
}

/* Debug impl */
impl<const N: usize> Debug for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut end = String::new();

        for row in self.pieces {
            for piece in row {
                end += &(piece.to_string() + &" ");
            }
            end.push('\n');
        }
        write!(f, "{}", end)
    }
}
