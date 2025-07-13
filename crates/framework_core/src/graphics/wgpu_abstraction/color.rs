#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
    pub const ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
    pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub const fn from_rgb_u8s(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }

    pub const fn from_rgba_u8s(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    pub fn to_linear(mut self) -> Self {
        self.r = to_linear(self.r);
        self.g = to_linear(self.g);
        self.b = to_linear(self.b);

        self
    }

    pub fn to_srgb(mut self) -> Self {
        self.r = to_srgb(self.r);
        self.g = to_srgb(self.g);
        self.b = to_srgb(self.b);

        self
    }

    pub const fn multiply_color(mut self, value: f32) -> Self {
        self.r *= value;
        self.g *= value;
        self.b *= value;
        self
    }

    pub const fn multiply_alpha(mut self, a: f32) -> Self {
        self.a *= a;
        self
    }

    const fn const_mul(mut self, rhs: f32) -> Self {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
        self
    }

    pub const fn lerp(start: Color, end: Color, progress: f32) -> Self {
        let multiplied_start = start.const_mul(1.0 - progress);
        let multiplied_end = end.const_mul(progress);

        Color {
            r: multiplied_end.r + multiplied_start.r,
            g: multiplied_end.g + multiplied_start.g,
            b: multiplied_end.b + multiplied_start.b,
            a: multiplied_end.a + multiplied_start.a,
        }
    }
}

// https://entropymine.com/imageworsener/srgbformula/
fn to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(v: f32) -> f32 {
    if v <= 0.0031308 {
        v * 12.92
    } else {
        (1.055 * v.powf(1.0 / 2.4)) - 0.055
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        self.const_mul(rhs)
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from_rgba_u8s(r, g, b, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgb_u8s(r, g, b)
    }
}
