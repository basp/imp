extern crate imp;

use imp::Lexer;

fn main() {
	let src = b"foo bar quux";
    let lexer = Lexer::new(src);
    println!("ok");
}
