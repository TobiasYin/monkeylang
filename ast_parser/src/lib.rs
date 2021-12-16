#![feature(core_panic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use std::error::Error;

use proc_macro2::TokenStream as TokenStream2;
use pyo3::Python;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{DeriveInput, parse2};

fn get_py_code() -> String {
    std::fs::read_to_string("/Users/tobias/projects/regex/codeparser.py").unwrap()
}

fn get_py_code1(data: &str, struct_name: &str, is_enum: bool) -> String {
    let is_enum_str = if is_enum { "True" } else { "False" };
    let mut code = String::new();
    code.push_str(format!("parse_and_gen({}, \"{}\", {})", data, struct_name, is_enum_str).as_str());
    code
}

#[proc_macro_attribute]
pub fn parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(_parser(TokenStream2::from(attr), TokenStream2::from(item)))
}

fn _parser(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let mut input: DeriveInput = parse2(item.clone()).unwrap();
    let struct_name = input.ident.to_string();
    let mut is_enum = false;
    let mut has_bad = false;
    let mut attr_str = attr.to_string();
    if let syn::Data::Enum(ref mut data) = input.data {
        is_enum = true;
        let mut kind = Vec::new();
        for var in data.variants.iter() {
            if var.ident.to_string().starts_with("Bad"){
                has_bad = true;
            }else {
                kind.push("Ast::".to_string() + &var.ident.to_string());
            }
        }
        if attr_str.is_empty(){
            attr_str =  format!("\"{}\"", kind.join(" | "));
        }
        if !has_bad{
            let name = "Bad".to_string() + &struct_name;
            data.variants.push(syn::Variant {
                ident: syn::Ident::new(&name, input.ident.span()),
                attrs: vec![],
                fields: syn::Fields::Unit,
                discriminant: None,
            });
        }

    }
    let code = Python::with_gil(|py| -> Result<String, Box<dyn Error>> {
        py.run(&get_py_code(), None, None)?;
        let code: String = py.eval(&get_py_code1(&attr_str, &input.ident.to_string(), is_enum), None, None)?.extract()?;
        Ok(code)
    }).unwrap();
    let mut stream = TokenStream2::new();
    input.to_tokens(&mut stream);
    let stream2: TokenStream2 = code.parse().unwrap();
    stream.append_all(stream2);

    if is_enum {
        let name = syn::Ident::new(&("Bad".to_string() + &struct_name), input.ident.span());
        let struct_name = syn::Ident::new( &struct_name, input.ident.span());
        let s = quote!{
            impl Default for #struct_name{
                fn default() -> Self {
                    #struct_name::#name
                }
            }
        };
        stream.append_all(s);
    }

    stream
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use crate::_parser;

    #[test]
    fn it_works() {
        let attr = quote! {
            "Ast::PlusExpression | Ast::MinusExpression"
        };
        let input = quote! {
            pub enum Expression{
                PlusExpression(PlusExpression),
                MinusExpression(MinusExpression),
                BadExpression(String),
            }
        };
        println!("{}", _parser(TokenStream2::from(attr), TokenStream2::from(input)).to_string());
    }
}
