use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum LexerToken<'a> {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    #[regex(r"-?[0-9]+\.[0-9]*", |lex| lex.slice().parse())]
    Float(f64),

    #[regex("#t|#f", |lex| if lex.slice() == "#t" {true} else {false})]
    Bool(bool),

    #[token("+")]
    OpAdd,

    #[token("-")]
    OpSub,

    #[token("*")]
    OpMul,

    #[token("/")]
    OpDiv,

    #[token(">")]
    OpGreater,

    #[token("<")]
    OpSmaller,

    #[token("==")]
    OpEqual,

    #[token("!=")]
    OpNotEqual,

    #[token("(")]
    ParenthesesOpen,

    #[token(")")]
    ParenthesesClose,

    #[regex("def|lambda", |lex| lex.slice())]
    Keyword(&'a str),

    #[token("if")]
    If,

    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*", priority = 2, callback = |lex| lex.slice())]
    Name(&'a str),
}

pub fn lexing<'a>(str: &'a str) -> Vec<LexerToken<'a>> {
    let lex = LexerToken::lexer(&str);
    lex.collect()
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_add_int() {
        let mut lex = LexerToken::lexer("(+ 1 2)");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::OpAdd));
        assert_eq!(lex.slice(), "+");

        assert_eq!(lex.next(), Some(LexerToken::Integer(1)));
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(LexerToken::Integer(2)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_add_float() {
        let mut lex = LexerToken::lexer("(+ 1. 2.)");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::OpAdd));
        assert_eq!(lex.slice(), "+");

        assert_eq!(lex.next(), Some(LexerToken::Float(1.0)));
        assert_eq!(lex.slice(), "1.");

        assert_eq!(lex.next(), Some(LexerToken::Float(2.0)));
        assert_eq!(lex.slice(), "2.");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_sub_float() {
        let mut lex = LexerToken::lexer("(- -1. 2.)");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::OpSub));
        assert_eq!(lex.slice(), "-");

        assert_eq!(lex.next(), Some(LexerToken::Float(-1.0)));
        assert_eq!(lex.slice(), "-1.");

        assert_eq!(lex.next(), Some(LexerToken::Float(2.0)));
        assert_eq!(lex.slice(), "2.");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_define_main() {
        let mut lex = LexerToken::lexer("(def main \n(lambda () + 1 2))");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::Keyword("def")));
        assert_eq!(lex.slice(), "def");

        assert_eq!(lex.next(), Some(LexerToken::Name("main")));
        assert_eq!(lex.slice(), "main");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::Keyword("lambda")));
        assert_eq!(lex.slice(), "lambda");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), Some(LexerToken::OpAdd));
        assert_eq!(lex.slice(), "+");

        assert_eq!(lex.next(), Some(LexerToken::Integer(1)));
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(LexerToken::Integer(2)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_bool() {
        let mut lex = LexerToken::lexer("(if (== #t #f) (print 2))");
        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::If));
        assert_eq!(lex.slice(), "if");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::OpEqual));
        assert_eq!(lex.slice(), "==");

        assert_eq!(lex.next(), Some(LexerToken::Bool(true)));
        assert_eq!(lex.slice(), "#t");

        assert_eq!(lex.next(), Some(LexerToken::Bool(false)));
        assert_eq!(lex.slice(), "#f");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::Keyword("print")));
        assert_eq!(lex.slice(), "print");

        assert_eq!(lex.next(), Some(LexerToken::Integer(2)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), None);
    }

    // #[test]
    // fn test_all() {
    //     let mut lex = LexerToken::lexer(
    //         "(def circle-area (lambda () (
    //             (def pi 3.14)
    //             (def r 10.0)
    //             (def sqr (lambda (r) (* r r)))
    //             (def area (lambda (r) (* pi (sqr r))))
    //             )))
    //             (def main (lambda () (if (< (circle-area) 2) 2))",
    //     );

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::Keyword("def")));
    //     assert_eq!(lex.slice(), "def");

    //     assert_eq!(lex.next(), Some(LexerToken::Name("circle-area")));
    //     assert_eq!(lex.slice(), "circle-area");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::Keyword("lambda")));
    //     assert_eq!(lex.slice(), "lambda");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
    //     assert_eq!(lex.slice(), ")");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::Keyword("def")));
    //     assert_eq!(lex.slice(), "def");

    //     assert_eq!(lex.next(), Some(LexerToken::Name("pi")));
    //     assert_eq!(lex.slice(), "pi");

    //     assert_eq!(lex.next(), Some(LexerToken::Float(3.14)));
    //     assert_eq!(lex.slice(), "3.14");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
    //     assert_eq!(lex.slice(), ")");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::Keyword("def")));
    //     assert_eq!(lex.slice(), "def");

    //     assert_eq!(lex.next(), Some(LexerToken::Name("r")));
    //     assert_eq!(lex.slice(), "r");

    //     assert_eq!(lex.next(), Some(LexerToken::Float(10.0)));
    //     assert_eq!(lex.slice(), "10.0");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesClose));
    //     assert_eq!(lex.slice(), ")");

    //     assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
    //     assert_eq!(lex.slice(), "(");

    //     assert_eq!(lex.next(), Some(LexerToken::Keyword("def")));
    //     assert_eq!(lex.slice(), "def");

    //     assert_eq!(lex.next(), Some(LexerToken::Name("sqr")));
    //     assert_eq!(lex.slice(), "sqr");

    // }
}
