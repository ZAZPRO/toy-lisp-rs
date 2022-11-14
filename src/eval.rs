use crate::object::Op;
use crate::{lexer, parser};
use crate::{object::Object, scope::Scope};
use std::cell::RefCell;
use std::rc::Rc;

pub fn eval(string: String, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    let mut lexer_tokens = lexer::lexing(&string);
    lexer_tokens.reverse();
    let parsed_objects = match parser::parse(&mut lexer_tokens) {
        Err(s) => return Err(s.to_string()),
        Ok(o) => o,
    };
    eval_obj(&parsed_objects, scope)
}

fn eval_obj(obj: &Object, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match obj {
        Object::List(l) => eval_list(l, scope),
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(f) => Ok(Object::Float(*f)),
        Object::Name(s) => eval_name(s, scope),
        _ => Err("Unexpected eval type!".to_string()),
    }
}

fn eval_list(list: &Vec<Object>, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    if list.len() < 1 {
        return Err("Invalid List".to_string());
    }
    let head = &list[0];
    match head {
        Object::Condition => {
            if list.len() != 4 {
                return Err("Invalid number of arguments for If conditional check!".to_string());
            }

            let cond_obj = eval_obj(&list[1], scope)?;
            let cond = match cond_obj {
                Object::Bool(b) => b,
                _ => return Err("Condition must be a Bool type!".to_string()),
            };

            if cond == true {
                return eval_obj(&list[2], scope);
            } else {
                return eval_obj(&list[3], scope);
            }
        }

        Object::Keyword(s) => match s.as_str() {
            "def" => {
                if list.len() != 3 {
                    return Err("Invalid number of arguments for define. Must be 3!".to_string());
                }

                let name = &list[1].clone();

                match name {
                    Object::Name(s) => {
                        let o = eval_obj(&list[2], scope)?;
                        scope.borrow_mut().set(&s, o);
                        return Ok(Object::Void);
                    }
                    _ => Err("Second argument must be a Name type!".to_string()),
                }
            }

            "lambda" => {
                let params = match &list[1] {
                    Object::List(list) => {
                        let mut params = Vec::new();
                        for o in list {
                            match o {
                                Object::Name(s) => params.push(s.clone()),
                                _ => {
                                    return Err(
                                        "Lambda parameter list type must be Name type!".to_string()
                                    )
                                }
                            }
                        }
                        params
                    }
                    _ => return Err("Lambda parameters is not a list!".to_string()),
                };

                let body = match &list[2] {
                    Object::List(list) => list.clone(),
                    _ => return Err("Lambda parameters is not a list!".to_string()),
                };

                Ok(Object::Lambda(params, body))
            }

            _ => {
                return Err("Invalid keyword!".to_string());
            }
        },

        Object::Operator(_) => {
            return eval_operator(&head, list, scope);
        }

        Object::Name(s) => {
            return eval_lambda_call(s, scope, list);
        }

        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, scope)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_lambda_call(
    s: &String,
    scope: &mut Rc<RefCell<Scope>>,
    list: &Vec<Object>,
) -> Result<Object, String> {
    let lamdba = scope.borrow_mut().get(s);
    if lamdba.is_some() {
        match lamdba.unwrap() {
            Object::Lambda(params, body) => {
                let mut new_scope = Rc::new(RefCell::new(Scope::extend(scope.clone())));
                for (i, param) in params.iter().enumerate() {
                    let obj = eval_obj(&list[1 + i], scope)?;
                    new_scope.borrow_mut().set(param, obj);
                }
                return eval_obj(&Object::List(body), &mut new_scope);
            }
            _ => return Err(format!("Unbound symbol: {} !", s)),
        }
    } else {
        return Err(format!("Unbound symbol: {} !", s));
    }
}

fn eval_name(s: &String, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    let obj = scope.borrow_mut().get(s);
    if obj.is_none() {
        return Err("There is no defined name in this environment!".to_string());
    }

    Ok(obj.unwrap().clone())
}

fn eval_operator(
    object: &Object,
    list: &Vec<Object>,
    scope: &mut Rc<RefCell<Scope>>,
) -> Result<Object, String> {
    match object {
        Object::Operator(operator) => {
            let other_operands = list[1..].to_vec();

            let mut operands: Vec<Object> = Vec::new();
            for obj in other_operands {
                let operand = eval_obj(&obj, scope)?;
                operands.push(operand.clone());
            }

            let first_operand = &operands[0];
            if !operands.iter().all(|_obj| matches!(first_operand, _obj)) {
                return Err("Operands are not the same type!".to_string());
            } else {
                match operator {
                    Op::Add => match first_operand {
                        Object::Integer(n) => {
                            let mut sum: i64 = *n;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Integer(n) => sum += n,
                                    _ => return Err("+ Operator Int Error!".to_string()),
                                }
                            }
                            return Ok(Object::Integer(sum));
                        }
                        Object::Float(f) => {
                            let mut sum: f64 = *f;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Float(f) => sum += f,
                                    _ => return Err("+ Operator Float Error!".to_string()),
                                }
                            }
                            return Ok(Object::Float(sum));
                        }
                        _ => {
                            return Err(
                                "+ Operator not implemented for this object type!".to_string()
                            )
                        }
                    },

                    Op::Sub => match first_operand {
                        Object::Integer(n) => {
                            let mut diff: i64 = *n;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Integer(n) => diff -= n,
                                    _ => return Err("- Operator Int Error!".to_string()),
                                }
                            }
                            return Ok(Object::Integer(diff));
                        }
                        Object::Float(f) => {
                            let mut diff: f64 = *f;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Float(f) => diff -= f,
                                    _ => return Err("- Operator Float Error!".to_string()),
                                }
                            }
                            return Ok(Object::Float(diff));
                        }
                        _ => {
                            return Err(
                                "- Operator not implemented for this object type!".to_string()
                            )
                        }
                    },

                    Op::Mul => match first_operand {
                        Object::Integer(n) => {
                            let mut mult: i64 = *n;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Integer(n) => mult *= n,
                                    _ => return Err("* Operator Int Error!".to_string()),
                                }
                            }
                            return Ok(Object::Integer(mult));
                        }
                        Object::Float(f) => {
                            let mut mult: f64 = *f;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Float(f) => mult *= f,
                                    _ => return Err("* Operator Float Error!".to_string()),
                                }
                            }
                            return Ok(Object::Float(mult));
                        }
                        _ => {
                            return Err(
                                "* Operator not implemented for this object type!".to_string()
                            )
                        }
                    },

                    Op::Div => match first_operand {
                        Object::Integer(n) => {
                            let mut diff: i64 = *n;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Integer(n) => diff /= n,
                                    _ => return Err("/ Operator Int Error!".to_string()),
                                }
                            }
                            return Ok(Object::Integer(diff));
                        }
                        Object::Float(f) => {
                            let mut diff: f64 = *f;
                            for i in 1..operands.len() {
                                match operands[i] {
                                    Object::Float(f) => diff /= f,
                                    _ => return Err("/ Operator Float Error!".to_string()),
                                }
                            }
                            return Ok(Object::Float(diff));
                        }
                        _ => {
                            return Err(
                                "/ Operator not implemented for this object type!".to_string()
                            )
                        }
                    },

                    Op::Eq => {
                        let mut res = false;
                        for i in 1..operands.len() {
                            if operands[i] != *first_operand {
                                return Ok(Object::Bool(false));
                            } else {
                                res = true;
                            }
                        }
                        return Ok(Object::Bool(res));
                    }

                    Op::NotEq => {
                        let mut res = false;
                        for i in 1..operands.len() {
                            if operands[i] == *first_operand {
                                res |= false;
                            } else {
                                res |= true;
                            }
                        }
                        return Ok(Object::Bool(res));
                    }

                    Op::Greater => {
                        let mut res = false;
                        for i in 1..operands.len() {
                            if *first_operand > operands[i] {
                                res = true;
                            } else {
                                return Ok(Object::Bool(false));
                            }
                        }
                        return Ok(Object::Bool(res));
                    }

                    Op::Smaller => {
                        let mut res = false;
                        for i in 1..operands.len() {
                            if *first_operand < operands[i] {
                                res = true;
                            } else {
                                return Ok(Object::Bool(false));
                            }
                        }
                        return Ok(Object::Bool(res));
                    }
                }
            }
        }
        _ => Err("Not an operator!".to_string()),
    }
}
