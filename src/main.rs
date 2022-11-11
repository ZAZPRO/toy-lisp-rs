mod lexer;
mod object;
mod parser;

fn main() {
    let src = "(+ 1 2)";
    let mut lexer_tokens = lexer::lexing(src);
    lexer_tokens.reverse();
    let parset_objects = parser::parse(&mut lexer_tokens);
    println!("{:?}", parset_objects);
}
