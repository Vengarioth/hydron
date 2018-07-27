#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate nom;

mod ast;
mod code_printer;
mod parser;

use proc_macro::TokenStream;
use ast::AstVisitor;

#[proc_macro]
pub fn template(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    println!("{}", input);
    let tree: Vec<proc_macro2::TokenTree> = input.into_iter().collect();

    let ast = parser::parse(&tree);

    println!("{:#?}", ast);

    let mut printer = code_printer::CodePrinter::new();

    println!("");
    let result = printer.generate(&ast);
    println!("");

    println!("{}", result);

    result.parse().unwrap()
}
