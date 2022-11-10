mod lexer;

fn main() {
    let src = "(+ 1 2)";
    let lexer_tokens = lexer::lexing(src);
}
