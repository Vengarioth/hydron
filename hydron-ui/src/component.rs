pub struct TestElement {
    pub width: usize,
    pub height: usize,
    pub children: Vec<Box<VirtualElement>>,
}

impl VirtualElement for TestElement {
    fn foo(&mut self) {

    }
}

pub trait VirtualElement {
    fn foo(&mut self);
}

pub trait Component {
    fn render(&mut self) -> Box<VirtualElement>;
}
