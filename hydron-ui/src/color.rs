#[derive(Debug, Copy, Clone)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Rgba {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Rgba { r: r, g: g, b: b, a: a, }
    }
}
