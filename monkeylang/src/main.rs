use monkeylang::ast::types::{parse_demo, parse_fn};
use monkeylang::token;
// static CODE: &str = "\
// fn main() {
//     let x = 5;
//     let y = x + 1.25;
//     x += 1;
//     y += 1;
//     let z = \"hello\";
//     let a = 'a';
//     if a == 'a' {
//         println(a);
//     }
// }";
// const CODE: &str = "fn test_func(a:int, c:string) { 1 + 2; }";
const CODE: &str = "1 + 2;";
// static CODE: &str = "fn test_func() {}";

fn main() {
    let mut tokens = token::parse(CODE);
    // for i in tokens.iter() {
    //     println!("{:?}", i);
    // }
    // parse_fn(&mut tokens).unwrap();
    parse_demo(&mut tokens).unwrap();
}
