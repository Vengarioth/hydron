use super::rendering::command::*;
use super::constraint::*;
use super::layout::*;
use super::size::*;
use super::rect::*;
use super::style::*;
use ::std::mem::swap;

pub struct LayoutContext {
    data: Vec<Rect>,
}

impl LayoutContext {
    pub fn new() -> LayoutContext {
        LayoutContext {
            data: vec![],
        }
    }

    fn insert(&mut self, rect: Rect) {
        self.data.push(rect);
    }

    fn get_rect(&self, index: usize) -> Rect {
        self.data[index]
    }

    fn set_size(&mut self, index: usize, width: usize, height: usize) {
        self.data[index].width = width;
        self.data[index].height = height;
    }

    pub fn get_child_size(&mut self, index: usize) -> Size {
        let rect = self.data[index];
        Size::new(rect.width, rect.height)
    }

    pub fn position_child(&mut self, index: usize, x: usize, y: usize) {
        self.data[index].x = x;
        self.data[index].y = y;
    }
}

pub struct RenderContext {
    commands: Vec<Command>,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            commands: vec![],
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    fn get_commands(&mut self) -> Vec<Command> {
        let mut swapped = vec![];
        swap(&mut self.commands, &mut swapped);
        swapped
    }
}

#[allow(unused)]
pub struct Surface {
    size: Size,
    root_index: Option<usize>,
    elements: Vec<Box<LayoutElement>>,
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    styles: Vec<Style>,
    layout_context: LayoutContext,
    render_context: RenderContext,
    free_list: Vec<usize>,
}

#[allow(unused)]
impl Surface {
    pub fn new(size: Size) -> Surface {
        Surface {
            size,
            root_index: None,
            elements: vec![],
            children: vec![],
            parents: vec![],
            styles: vec![],
            layout_context: LayoutContext::new(),
            render_context: RenderContext::new(),
            free_list: vec![],
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_style(&mut self, index: usize, style: Style) {
        self.styles[index] = style;
    }

    pub fn paint(&mut self) -> Vec<Command> {
        if let Some(i) = self.root_index {
            let mut index_stack: Vec<usize> = vec![];
            index_stack.push(i);

            while index_stack.len() > 0 {
                let index = index_stack.pop().unwrap();
                let rect = self.layout_context.get_rect(index);

                let mut node = &mut self.elements[index];
                let style = &self.styles[index];
                node.paint(rect, &mut self.render_context, &style);

                index_stack.extend(self.children[index].iter());
            }
        }

        self.render_context.get_commands()
    }

    pub fn layout(&mut self) {
        if let Some(i) = self.root_index {
            let mut index_stack: Vec<usize> = vec![];
            let mut constraint_stack: Vec<Constraint> = vec![];
            let mut result_stack: Vec<Size> = vec![];

            index_stack.push(i);
            self.layout_context.set_size(i, self.size.width, self.size.height);
            self.layout_context.position_child(i, 0, 0);
            constraint_stack.push(Constraint::fixed(self.size.width, self.size.height));

            while index_stack.len() > 0 {
                let mut index = index_stack.pop().unwrap();
                let mut constraint = constraint_stack.pop().unwrap();

                let mut node = &mut self.elements[index];
                let mut children = &mut self.children[index];

                let mut size = result_stack.pop();

                match node.layout(constraint, &mut self.layout_context, children, size) {
                    LayoutResult::LayoutChild(child_index, child_constraint) => {

                        index_stack.push(index);
                        constraint_stack.push(constraint);

                        index_stack.push(child_index);
                        constraint_stack.push(child_constraint);
                    },
                    LayoutResult::Done(result_size) => {
                        self.layout_context.set_size(index, result_size.width, result_size.height);
                        result_stack.push(result_size);
                    },
                }
            }
        }
    }

    pub fn set_root(&mut self, index: usize) {
        self.root_index = Some(index);
    }

    
    pub fn insert(&mut self, element: Box<LayoutElement>) -> usize {
        if let Some(index) = self.free_list.pop() {
            self.elements[index] = element;
            self.styles[index] = Style::empty();
            self.layout_context.set_size(index, 0, 0);
            self.layout_context.position_child(index, 0, 0);
            return index;
        }

        let index = self.elements.len();
        self.layout_context.insert(Rect::new(0, 0, 0, 0));
        self.elements.push(element);
        self.children.push(vec![]);
        self.parents.push(None);
        self.styles.push(Style::empty());
        index
    }

    pub fn remove(&mut self, index: usize) {
        let mut stack = vec![];
        stack.push(index);

        while stack.len() > 0 {
            let index = stack.pop().unwrap();

            if let Some(root_index) = self.root_index {
                if root_index == index {
                    self.root_index = None;
                }
            }

            if let Some(parent_index) = self.parents[index] {
                self.children[parent_index].remove_item(&index);
            }

            stack.extend(self.children[index].iter());
            self.children[index].clear();

            self.free_list.push(index);
        }
    }

    pub fn set_parent(&mut self, child_index: usize, parent_index: usize) {
        self.children[parent_index].push(child_index);
        self.parents[child_index] = Some(parent_index);
    }
}
