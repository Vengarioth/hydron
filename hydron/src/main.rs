#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate libc;
extern crate gl;
extern crate glutin;
extern crate cgmath;

extern crate hydron_ui;
extern crate hydron_template;

mod renderer;

use glutin::dpi::*;
use glutin::GlContext;
use hydron_ui::*;
use hydron_ui::elements::*;
use hydron_ui::style::*;
use hydron_template::template;

use renderer::*;

#[derive(Debug)]
struct TestComponent {
    pub foo: usize,
    pub children: Vec<TestComponent>,
}

impl hydron_ui::Component for TestComponent {
    fn render(&mut self) -> Box<VirtualElement> {
        template!{
            <test width={100} height={100}>
                <test width={100} height={100} />
            </test>
        }
    }
}

fn main() {
    let foo = 15;
    let template = template!{
        <TestComponent foo={foo}>
            <TestComponent foo={30} />
        </TestComponent>
    };
    println!("{:#?}", template);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("hydron")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    }

    let mut renderer = Renderer::new();
    let mut surface = Surface::new(Size::new(1024, 768));

    let root_index = surface.insert(Box::new(BoxElement::new()));
    surface.set_root(root_index);
    surface.set_style(root_index, Style::new(Background::Color(Rgba::new(1.0, 1.0, 1.0, 1.0)), TRBL::new(Border::None, Border::None, Border::None, Border::None)));

    let index_0 = surface.insert(Box::new(FlexElement::new(Axis::Horizontal)));
    surface.set_parent(index_0, root_index);

    let index_1 = surface.insert(Box::new(FixedElement::new(200, 100)));
    surface.set_style(index_1, Style::new(
        Background::Color(Rgba::new(1.0, 0.0, 0.0, 1.0)),
        TRBL::new(Border::None, Border::None, Border::None, Border::None)
    ));

    let index_2 = surface.insert(Box::new(FixedElement::new(200, 200)));
    surface.set_style(index_2, Style::new(
        Background::Color(Rgba::new(0.0, 1.0, 0.0, 1.0)),
        TRBL::new(Border::None, Border::None, Border::None, Border::None)
    ));

    let index_3 = surface.insert(Box::new(FixedElement::new(200, 300)));
    surface.set_style(index_3, Style::new(
        Background::Color(Rgba::new(0.0, 0.0, 1.0, 1.0)),
        TRBL::new(Border::None, Border::None, Border::None, Border::None)
    ));

    let index_4 = surface.insert(Box::new(FixedElement::new(200, 400)));
    surface.set_style(index_4, Style::new(
        Background::Color(Rgba::new(0.5, 0.5, 0.5, 1.0)),
        TRBL::new(Border::None, Border::None, Border::None, Border::None)
    ));

    surface.set_parent(index_1, index_0);
    surface.set_parent(index_2, index_0);
    surface.set_parent(index_3, index_0);
    surface.set_parent(index_4, index_0);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                        let new_size = Size::new(logical_size.width as usize, logical_size.height as usize);
                        surface.resize(new_size);
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        surface.layout();
        let commands = surface.paint();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.render(surface.get_size(), commands);

        gl_window.swap_buffers().unwrap();
    }
}
