use ::layout::*;
use ::constraint::*;
use ::size::*;
use ::surface::*;

pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn major(&self, coords: (usize, usize)) -> usize {
        match self {
            Axis::Horizontal => coords.0,
            Axis::Vertical => coords.1,
        }
    }

    fn minor(&self, coords: (usize, usize)) -> usize {
        match self {
            Axis::Horizontal => coords.1,
            Axis::Vertical => coords.0,
        }
    }

    fn pack(&self, major: usize, minor: usize) -> (usize, usize) {
        match self {
            Axis::Horizontal => (major, minor),
            Axis::Vertical => (minor, major),
        }
    }
}

pub struct FlexElement {
    direction: Axis,
    ix: usize,
    major_per_flex: usize,
    minor: usize,
}

impl FlexElement {
    pub fn new(direction: Axis) -> FlexElement {
        FlexElement {
            direction,
            ix: 0,
            major_per_flex: 0,
            minor: 0,
        }
    }
}

impl LayoutElement for FlexElement {
    fn layout(&mut self, constraint: Constraint, context: &mut LayoutContext, children: &mut Vec<usize>, requested: Option<Size>) -> LayoutResult {
        if let Some(size) = requested {
            let minor = self.direction.minor((size.width, size.height));
            self.minor = self.minor.max(minor);
            self.ix += 1;

            if self.ix == children.len() {
                let mut major = 0;
                for child in children {
                    let child_size = context.get_child_size(*child);
                    let child_major_size = self.direction.major((child_size.width, child_size.height));
                    let padding = if self.major_per_flex > child_major_size {
                        (self.major_per_flex - child_major_size) / 2
                    } else {
                        0
                    };
                    let (x, y) = self.direction.pack(major + padding, 0);
                    context.position_child(*child, x, y);
                    major += self.major_per_flex;
                }
                let max_major = self.direction.major((constraint.max_width, constraint.max_height));
                let (w, h) = self.direction.pack(max_major, self.minor);
                return LayoutResult::Done(Size::new(w, h));
            }
        } else {
            if children.is_empty() {
                return LayoutResult::Done(Size::new(constraint.min_width, constraint.min_height));
            }

            self.ix = 0;
            self.minor = self.direction.minor((constraint.min_width, constraint.min_height));
            let max_major = self.direction.major((constraint.max_width, constraint.max_height));
            self.major_per_flex = max_major / children.len();
        }

        let child_constraint = match self.direction {
            Axis::Horizontal => Constraint::new(self.major_per_flex, self.major_per_flex, constraint.min_height, constraint.max_height),
            Axis::Vertical => Constraint::new(constraint.min_width, constraint.max_width, self.major_per_flex, self.major_per_flex),
        };
        LayoutResult::LayoutChild(children[self.ix], child_constraint)
    }
}
