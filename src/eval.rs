use crate::{env::Env, error::LispError, exp::LispExp, lexer::tokenize, parser::parse};

pub fn eval_str(code: &str, env: &mut Env) -> Result<LispExp, LispError> {
    let tokens = tokenize(code)?;
    let mut exps = parse(tokens)?;
    let last = exps.remove(exps.len() - 1);
    for exp in exps.iter() {
        eval(exp.clone(), env)?;
    }
    eval(last, env)
}

pub fn eval(exp: LispExp, env: &mut Env) -> Result<LispExp, LispError> {
    match exp {
        LispExp::Bool(_) => Ok(exp),
        LispExp::Number(_) => Ok(exp),
        LispExp::Procedure(_) => Ok(exp),
        LispExp::Lambda { params, body, env: captured_env } => {
            Ok(LispExp::Lambda { params, body, env: captured_env })
        }
        LispExp::Symbol(sym) => {
            env.lookup(&sym).map(Ok)
                .unwrap_or(Err(LispError::Eval(format!("undefined variable: {}", sym))))
        },
        LispExp::List(exps) => {
            // 第一个是 operator 剩下的是参数
            let op = eval(exps[0].clone(), env)?;
            let result = apply(op, exps.into_iter().skip(1).collect(), env)?;
            Ok(result)
        }
    }
}

// apply 提供新的 env
pub fn apply(procedure: LispExp, params: Vec<LispExp>, env: &mut Env) -> Result<LispExp, LispError> {
    // builtin procedure 不需要新的 env
    match &procedure {
        LispExp::Procedure(func) => {
            Ok(func(&params, env)?)
        },
        LispExp::Lambda { params: param_slots, body, env } => {
            let mut new_env = Env::extend(env.clone());
            match param_slots.as_ref() {
                LispExp::List(list) => {
                    for (index, exp) in list.iter().enumerate() {
                        let key = exp.expect_symbol()?;
                        let value = params.get(index);
                        if let Some(value) = value {
                            new_env.assign(&key, value.clone());
                        } else {
                            return Err(LispError::Eval(format!("apply params count not match: {:?}, {:?}", procedure, params)))
                        }
                    }
                },
                _ => return Err(LispError::Eval(format!("apply params count not match: {:?}, {:?}", procedure, params)))
            }
            Ok(eval(body.as_ref().clone(), &mut new_env)?)
        },
        _ => Err(LispError::Eval(format!("can not apply: {:?}", procedure)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{env::Env, eval::eval_str, exp::LispExp};

    #[test]
    fn test_simple() {
        assert_eq!(eval_str("(+ 2 2)", &mut Env::new()), Ok(LispExp::Number(4.0)));
        assert_eq!(eval_str("(+ 2 (- 5 1))", &mut Env::new()), Ok(LispExp::Number(6.0)));
        assert_eq!(eval_str("(* 2 (/ 5 1))", &mut Env::new()), Ok(LispExp::Number(10.0)));
        assert_eq!(eval_str("((lambda (a) (+ a 1)) 10)", &mut Env::new()), Ok(LispExp::Number(11.0)));
        assert_eq!(eval_str("
            (define a 1)
            (+ a 5)
        ", &mut Env::new()), Ok(LispExp::Number(6.0)));
    }
}