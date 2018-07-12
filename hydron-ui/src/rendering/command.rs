use ::rect::*;
use ::color::*;

pub enum Command {
    DrawRect(Rect, Rgba),
}
