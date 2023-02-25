/* Imports */

/* Main */
struct Board<const N: usize> {
    /// Two-dim array of pieces
    pieces: [[u16; N]; N],
}

/* Method impls */
impl<const N: usize> Board<N> {
    /// Constructor
    pub fn new() -> Self {
        Self { pieces: [[0; N]; N] }
    }
}
