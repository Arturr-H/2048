/* Imports */


/* Types */
type Brick = u16;

/* Main */
struct Board<const N: usize> {
    /// Two-dim array of pieces
    pieces: [[Brick; N]; N],
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
}
