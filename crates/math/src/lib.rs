mod rect;
mod time;

pub use glam::*;
pub use rect::*;
pub use time::*;

#[macro_export]
macro_rules! inverse_lerp {
    ($start:expr, $end:expr, $value:expr) => {{
        let start = $start as f32;
        let end = $end as f32;
        let value = $value as f32;

        let range = end - start;
        ((value - start) / range)
    }};
}

#[macro_export]
macro_rules! clamped_inverse_lerp {
    ($start:expr, $end:expr, $value:expr) => {
        inverse_lerp!($start, $end, $value).clamp(0.0, 1.0)
    };
}
