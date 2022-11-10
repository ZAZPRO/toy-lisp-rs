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

    #[token("[#t]|[#f]", |lex| if lex.slice() == "#t" {true} else {false})]
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

    #[token("def")]
    Define,

    #[token("lambda")]
    Lambda,

    #[token("if")]
    If,

    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice())]
    Symbol(&'a str),
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

        assert_eq!(lex.next(), Some(LexerToken::Define));
        assert_eq!(lex.slice(), "def");

        assert_eq!(lex.next(), Some(LexerToken::Symbol("main")));
        assert_eq!(lex.slice(), "main");

        assert_eq!(lex.next(), Some(LexerToken::ParenthesesOpen));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(LexerToken::Lambda));
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
}
