use crate::object::Op;
use crate::{env::Scope, object::Object};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn eval(parsed_list: Object, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    eval_obj(&parsed_list, scope)
}

// fn eval_lambda_call(params: Vec<String>, body: Vec<Object>, scope: &mut Rc<RefCell<Scope>>) {
//     let mut new_scope = Rc::new(RefCell::new(Scope::extend(scope.clone())));
//     for p in params {
        
//     }
// }

fn eval_name(s: &String, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    let obj = scope.borrow_mut().get(s);
    if obj.is_none() {
        return Err("There is no defined name in this environment".to_string());
    }

    let ret_obj = obj.unwrap().clone();
    // match ret_obj {
    //     Object::Lambda(_, _) => {
    //         println!("HERE");
    //     }
    //     _ => (),
    // }
    Ok(ret_obj)
}

fn eval_list(l: &Vec<Object>, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    let head = &l[0];
    match head {
        Object::Condition => {
            if l.len() != 4 {
                return Err("Invalid number of arguments for if\n".to_string());
            }

            let cond_obj = eval_obj(&l[1], scope)?;
            let cond = match cond_obj {
                Object::Bool(b) => b,
                _ => return Err("Condition must be Bool\n".to_string()),
            };

            if cond == true {
                return eval_obj(&l[2], scope);
            } else {
                return eval_obj(&l[3], scope);
            }
        }

        Object::Keyword(s) => match s.as_str() {
            "def" => {
                if l.len() != 3 {
                    return Err("Invalid number of arguments for define\n".to_string());
                }

                let name = &l[1].clone();

                match name {
                    Object::Name(s) => {
                        let o = eval_obj(&l[2], scope)?;
                        scope.borrow_mut().set(&s, o);
                        return Ok(Object::Void);
                    }
                    _ => Err("Second argument must be a Name\n".to_string()),
                }
            }

            "lambda" => {
                let params = match &l[1] {
                    Object::List(list) => {
                        let mut params = Vec::new();
                        for o in list {
                            match o {
                                Object::Name(s) => params.push(s.clone()),
                                _ => {
                                    return Err(
                                        "Lambda parameter list type must be names!\n".to_string()
                                    )
                                }
                            }
                        }
                        params
                    }
                    _ => return Err("Lambda parameters is not a list!\n".to_string()),
                };

                let body = match &l[2] {
                    Object::List(list) => list.clone(),
                    _ => return Err("Lambda parameters is not a list!\n".to_string()),
                };

                Ok(Object::Lambda(params, body))
            }

            _ => {
                return Err("Invalid keyword!".to_string());
            }
        },

        Object::Operator(o) => {
            let operands = Object::List(l[1..].to_vec());
            let evaluated_operands = eval_obj(&operands, scope)?;
            match evaluated_operands {
                Object::List(operands) => {
                    let first_operand = &operands[0];
                    if !operands.iter().all(|_o| matches!(first_operand, _o)) {
                        return Err("Operands are not the same type!".to_string());
                    } else {
                        match o {
                            Op::Add => match first_operand {
                                Object::Integer(n) => {
                                    let mut sum: i64 = *n;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Integer(n) => sum += n,
                                            _ => return Err("How? + Int".to_string()),
                                        }
                                    }
                                    return Ok(Object::Integer(sum));
                                }
                                Object::Float(f) => {
                                    let mut sum: f64 = *f;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Float(f) => sum += f,
                                            _ => return Err("How? + Float".to_string()),
                                        }
                                    }
                                    return Ok(Object::Float(sum));
                                }
                                _ => return Err("Not implemented".to_string()),
                            },

                            Op::Sub => match first_operand {
                                Object::Integer(n) => {
                                    let mut diff: i64 = *n;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Integer(n) => diff -= n,
                                            _ => return Err("How? - Int".to_string()),
                                        }
                                    }
                                    return Ok(Object::Integer(diff));
                                }
                                Object::Float(f) => {
                                    let mut diff: f64 = *f;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Float(f) => diff -= f,
                                            _ => return Err("How? - Float".to_string()),
                                        }
                                    }
                                    return Ok(Object::Float(diff));
                                }
                                _ => return Err("Not implemented".to_string()),
                            },

                            Op::Div => match first_operand {
                                Object::Integer(n) => {
                                    let mut diff: i64 = *n;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Integer(n) => diff /= n,
                                            _ => return Err("How? / Int".to_string()),
                                        }
                                    }
                                    return Ok(Object::Integer(diff));
                                }
                                Object::Float(f) => {
                                    let mut diff: f64 = *f;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Float(f) => diff /= f,
                                            _ => return Err("How? / Float".to_string()),
                                        }
                                    }
                                    return Ok(Object::Float(diff));
                                }
                                _ => return Err("Not implemented".to_string()),
                            },

                            Op::Mul => match first_operand {
                                Object::Integer(n) => {
                                    let mut mult: i64 = *n;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Integer(n) => mult *= n,
                                            _ => return Err("How? * Int".to_string()),
                                        }
                                    }
                                    return Ok(Object::Integer(mult));
                                }
                                Object::Float(f) => {
                                    let mut mult: f64 = *f;
                                    for i in 1..operands.len() {
                                        match operands[i] {
                                            Object::Float(f) => mult *= f,
                                            _ => return Err("How? / Float".to_string()),
                                        }
                                    }
                                    return Ok(Object::Float(mult));
                                }
                                _ => return Err("Not implemented".to_string()),
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

                            _ => return Err("Not implemented operator!".to_string()),
                        }
                    }
                }
                _ => return Err("Wrong evaluated operands".to_string()),
            }
        }

        _ => {
            let mut new_list = Vec::new();
            for obj in l {
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

fn eval_obj(obj: &Object, scope: &mut Rc<RefCell<Scope>>) -> Result<Object, String> {
    match obj {
        Object::Void => Ok(Object::Void),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(f) => Ok(Object::Float(*f)),
        // Object::Operator(o) => eval_operator(o, scope),
        // Object::Condition => eval_condition(scope),
        // Object::Keyword(s) => eval_keyword(s, scope),
        Object::Name(s) => eval_name(s, scope),
        Object::List(l) => eval_list(l, scope),
        _ => todo!(),
    }
}
