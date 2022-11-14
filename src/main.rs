use std::error::Error;
use std::fs;
use std::{cell::RefCell, env, rc::Rc};

use crate::scope::Scope;
use rustyline::error::ReadlineError;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

mod eval;
mod lexer;
mod object;
mod parser;
mod scope;

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scope = Rc::new(RefCell::new(Scope::new()));

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // We have a file path, read it and evaluate it.
        let program_src = fs::read_to_string(args[1].clone())?;
        println!("Program Source:\n{}", program_src);
        let res_eval = eval::eval(program_src, &mut scope);
        match res_eval {
            Err(s) => println!("Evaluation Error: {}", s),
            Ok(o) => println!("Evaluation Output: {}", o),
        }
    } else {
        // There is not file path in the arguments meaning we are enabling cli mode.
        let h = InputValidator {
            brackets: MatchingBracketValidator::new(),
        };
        let mut rl = Editor::new()?;
        rl.set_helper(Some(h));
        rl.bind_sequence(
            KeyEvent(KeyCode::Enter, Modifiers::SHIFT),
            EventHandler::Simple(Cmd::Newline),
        );
        rl.bind_sequence(
            KeyEvent(KeyCode::Tab, Modifiers::NONE),
            EventHandler::Simple(Cmd::Insert(1, "  ".to_string())),
        );

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let res_eval = eval::eval(line, &mut scope);
                    match res_eval {
                        Err(s) => println!("Evaluation error: {}", s),
                        Ok(o) => println!("{}", o),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Got Interrupt Signal Ctrl-C. Exiting...");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Got Interrupt Signal Ctrl-D. Exiting...");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    Ok(())
}
