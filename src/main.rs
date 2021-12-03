use monkeylang::token;

static code: &str = "\
fn main() {
    let x = 5;
    let y = x + 1.25;
    x += 1;
    y += 1;
    let z = \"hello\";
    let a = 'a';
    if a == 'a' {
        println(a);
    }
}";

fn main() {
    let tokens = token::parse(code);
    for i in tokens.iter() {
        println!("{:?}", i);
    }
}
