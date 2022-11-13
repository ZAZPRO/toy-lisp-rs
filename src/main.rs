use std::{cell::RefCell, rc::Rc};

use crate::env::Scope;

mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() {
    let srcs = vec![
        // format!("(+ {} -{} 5 10)", i64::MAX, i64::MAX),
        // "(- 1000000000.0 1000000000.0 -20.0 5.0)".to_string(),
        // "(if (< 5 10 10) (#t) (#f))".to_string(),
        "(
            (def r 10.0)
            (def pi 3.1415926535897931)
            (* pi (* r r))
          )"
        .to_string(),
        "(
            (def sqr (lambda (r) (* r r))) 
            (sqr 10)
        )"
        .to_string(),
    ];
    for src in srcs {
        let mut lexer_tokens = lexer::lexing(src.as_str());
        lexer_tokens.reverse();
        let parsed_objects = parser::parse(&mut lexer_tokens).unwrap();
        println!("{}", parsed_objects);
        let mut scope = Rc::new(RefCell::new(Scope::new()));
        let eval_object = eval::eval(parsed_objects, &mut scope).unwrap();
        println!("{}", eval_object);
    }
}
