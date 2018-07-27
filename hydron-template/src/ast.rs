use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub struct Property {
    pub name: Identifier,
    pub value: TokenStream,
}

#[derive(Debug)]
pub enum TagContent {
    Escaped(TokenStream),
    Child(Tag),
}

#[derive(Debug)]
pub struct Tag {
    pub name: Identifier,
    pub properties: Vec<Property>,
    pub content: Vec<TagContent>,
}

pub trait AstVisitor {
    fn visit(&mut self, ast: &Tag) {
        self.visit_tag(ast);
    }

    fn visit_tag(&mut self, tag: &Tag) {
        self.visit_identifier(&tag.name);
        self.visit_tag_properties(&tag.properties);
    }

    fn visit_tag_properties(&mut self, properties: &Vec<Property>) {
        for property in properties {
            self.visit_tag_property(&property);
        }
    }

    fn visit_tag_property(&mut self, property: &Property) {

    }

    fn visit_identifier(&mut self, identifier: &Identifier) {

    }

    fn visit_tag_content(&mut self, content: &TagContent) {
        match content {
            TagContent::Child(ref tag) => self.visit_tag(tag),
            TagContent::Escaped(ref token_stream) => self.visit_escaped(token_stream),
        }
    }

    fn visit_escaped(&mut self, token_stream: &TokenStream) {

    }
}
