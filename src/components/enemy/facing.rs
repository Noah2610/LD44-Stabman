#[derive(Clone, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

impl Default for Facing {
    fn default() -> Self {
        Facing::Left
    }
}
