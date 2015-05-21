extern crate imp;

use imp::cmd::lexer::Lexer;

fn main() {
    let src = b"foo bar quux";
    let lexer = Lexer::new(src);
    let _: Vec<_> = lexer.collect();
    println!("ok");
}
