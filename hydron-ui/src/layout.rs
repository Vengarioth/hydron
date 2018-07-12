use super::rendering::command::*;
use super::constraint::*;
use super::surface::*;
use super::size::*;
use super::rect::*;
use super::style::*;

#[derive(Debug, Copy, Clone)]
pub enum LayoutResult {
    Done(Size),
    LayoutChild(usize, Constraint),
}

pub trait LayoutElement {

    #[allow(unused)]
    fn paint(&mut self, rect: Rect, context: &mut RenderContext, style: &Style) {
        match style.background {
            Background::Color(color) => context.add_command(Command::DrawRect(rect, color)),
            _ => (),
        }
    }

    #[allow(unused)]
    fn layout(&mut self, constraint: Constraint, context: &mut LayoutContext, children: &mut Vec<usize>, requested: Option<Size>) -> LayoutResult {
        match children.len() {
            0 => LayoutResult::Done(Size::new(constraint.min_width, constraint.min_height)),
            1 => {
                if let Some(size) = requested {
                    context.position_child(children[0], 0, 0);
                    LayoutResult::Done(size)
                } else {
                    LayoutResult::LayoutChild(children[0], Constraint::new(0, constraint.max_width, 0, constraint.max_height))
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }    
}
