use crate::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Start,
    Moving,
    End,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Touch {
    pub id: u64,
    pub phase: TouchPhase,
    /// Relative to the render. Top left is (-1.0, 1.0), bottom right is (1.0, -1.0)
    pub position: Vec2,
    /// Normalized in range [0.0, 1.0]
    pub pressure: Option<f32>,
}
