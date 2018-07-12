use ::layout::*;
use ::constraint::*;
use ::size::*;
use ::surface::*;

pub struct FixedElement {
    width: usize,
    height: usize,
}

impl FixedElement {
    pub fn new(width: usize, height: usize) -> FixedElement {
        FixedElement {
            width,
            height,
        }
    }
}

impl LayoutElement for FixedElement {
    fn layout(&mut self, constraint: Constraint, context: &mut LayoutContext, children: &mut Vec<usize>, requested: Option<Size>) -> LayoutResult {
        let constrained_width = self.width.min(constraint.max_width);
        let constrained_height = self.height.min(constraint.max_height);
        match children.len() {
            0 => LayoutResult::Done(Size::new(constrained_width, constrained_height)),
            1 => {
                if let Some(size) = requested {
                    context.position_child(children[0], 0, 0);
                    LayoutResult::Done(Size::new(constrained_width, constrained_height))
                } else {
                    LayoutResult::LayoutChild(children[0], Constraint::new(0, constrained_width, 0, constrained_height))
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
