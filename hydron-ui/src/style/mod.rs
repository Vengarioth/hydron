use ::color::*;

#[derive(Debug)]
pub struct TRBL<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T> TRBL<T> {
    pub fn new(top: T, right: T, bottom: T, left: T) -> TRBL<T> {
        TRBL {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Border {
    None,
    Color(Rgba),
}

#[derive(Debug, Copy, Clone)]
pub enum Background {
    None,
    Color(Rgba),
    Image(usize),
}

#[derive(Debug)]
pub struct Style {
    pub background: Background,
    pub border: TRBL<Border>,
}

impl Style {
    pub fn empty() -> Style {
        Style {
            background: Background::None,
            border: TRBL::new(Border::None, Border::None, Border::None, Border::None),
        }
    }
    pub fn new(background: Background, border: TRBL<Border>) -> Style {
        Style {
            background,
            border,
        }
    }
}
