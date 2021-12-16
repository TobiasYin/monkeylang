use monkeylang::ast::types::{parse_fn};
use monkeylang::token;
// static code: &str = "\
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
const code: &str = "fn test_func(a:int, c:string) {}";
// static code: &str = "fn test_func() {}";

fn main() {
    let mut tokens = token::parse(code);
    // for i in tokens.iter() {
    //     println!("{:?}", i);
    // }
    parse_fn(&mut tokens).unwrap();
}
