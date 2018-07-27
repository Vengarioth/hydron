use proc_macro2::TokenStream;
use ::ast::*;

#[derive(Debug)]
pub struct CodePrinter {
    indentation: usize,
    result: String,
}

impl CodePrinter {
    pub fn new() -> CodePrinter {
        CodePrinter {
            indentation: 0,
            result: "".to_string(),
        }
    }

    pub fn generate(&mut self, tag: &Tag) -> String {
        self.visit(tag);
        self.result.to_string()
    }

    fn print(&mut self, content: &str) {
        self.result.push_str(content);
    }

    fn println(&mut self) {
        self.result.push_str("\r\n");
    }

    fn push_indentation(&mut self) {
        self.indentation += 4;
    }

    fn pop_indentation(&mut self) {
        self.indentation -= 4;
    }

    fn print_indentation(&mut self) {
        for _ in 0..self.indentation {
            self.print(" ");
        }
    }
}

fn to_virtual_element_name(name: &str) -> String {
    let mut chars = name.chars();
    let mut s: String = match chars.next() {
        None => panic!("Element with invalid name"),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    };
    s.push_str("Element");
    s
}

impl AstVisitor for CodePrinter {
    fn visit_tag(&mut self, tag: &Tag) {

        if tag.name.name.chars().next().unwrap().is_lowercase() {
            let element_name = to_virtual_element_name(&tag.name.name);

            self.print_indentation();
            self.print(&format!("Box::new({} {{", element_name));

            self.push_indentation();
            self.println();
            self.visit_tag_properties(&tag.properties);

            self.println();
            self.print_indentation();
            self.print("children: vec![");

            self.println();
            self.push_indentation();
            for c in &tag.content {
                self.visit_tag_content(&c);
                self.print(",");

            }
            self.pop_indentation();

            self.println();
            self.print_indentation();
            self.print("],");

            self.pop_indentation();

            self.println();
            self.print_indentation();
            self.print("})");
            return;
        }

        self.print_indentation();
        self.print(&format!("{} {{", tag.name.name));

        self.push_indentation();
        self.println();
        self.visit_tag_properties(&tag.properties);
        self.pop_indentation();

        self.push_indentation();
        self.print_indentation();
        self.print("children: vec![");
        self.println();
        
        self.push_indentation();
        for c in &tag.content {
            self.visit_tag_content(&c);
            self.print(",");

        }
        self.println();
        self.pop_indentation();
        self.print_indentation();
        self.print("],");
        self.println();
        self.pop_indentation();

        self.print_indentation();
        self.print("}");
    }

    fn visit_escaped(&mut self, token_stream: &TokenStream) {
        self.println();
        self.print_indentation();
        self.print(&format!("{}", token_stream));
    }

    fn visit_tag_property(&mut self, property: &Property) {
        self.print_indentation();
        self.print(&format!("{}: {},", property.name.name, property.value));
        self.println();

    }
}
