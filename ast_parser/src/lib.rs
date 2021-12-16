#![feature(core_panic)]
extern crate proc_macro;
use pyo3::prelude::*;
use proc_macro::TokenStream;
use std::error::Error;
use syn::{parse_macro_input, DeriveInput};

fn get_py_code() -> String {
    std::fs::read_to_string("/Users/tobias/projects/regex/codeparser.py").unwrap()
}

fn get_py_code1(data: &str, struct_name: &str) -> String {
    let mut code = String::new();
    code.push_str(format!("parse_and_gen({}, \"{}\")", data, struct_name).as_str());
    code
}

#[proc_macro_attribute]
pub fn parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let old_item = item.clone();
    let input = parse_macro_input!(item as DeriveInput);
    let code = Python::with_gil(|py| -> Result<String, Box<dyn Error>> {
        py.run(&get_py_code(), None, None)?;
        let code: String = py.eval(&get_py_code1(&attr.to_string(), &input.ident.to_string()), None, None)?.extract()?;
        Ok(code)
    }).unwrap();
    let mut define = old_item.to_string();
    define.push_str("\n");
    define.push_str(&code);
    let stream: proc_macro::TokenStream = define.parse().unwrap();
    // panic!("{}", code);
    // panic!{"{}", item.to_string()}
    stream
}