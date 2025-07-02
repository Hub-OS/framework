#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}
