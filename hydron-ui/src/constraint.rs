#[derive(Debug, Copy, Clone)]
pub struct Constraint {
    pub min_width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub max_height: usize,
}

impl Constraint {
    pub fn new(min_width: usize, max_width: usize, min_height: usize, max_height: usize) -> Constraint {
        Constraint {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    pub fn fixed(width: usize, height: usize) -> Constraint {
        Constraint {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.min_width == self.max_width && self.min_height == self.max_height
    }
}
