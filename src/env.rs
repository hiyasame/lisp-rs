use std::collections::HashMap;

use crate::{
    error::LispError,
    eval::eval,
    exp::LispExp,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    parent: Option<Box<Env>>,
    vars: HashMap<String, LispExp>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            parent: None,
            vars: Self::base_env(),
        }
    }

    pub fn extend(parent: Self) -> Env {
        Env {
            vars: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<LispExp> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent.clone()?
                .lookup(name),
        }
    }

    pub fn assign(&mut self, name: &str, val: LispExp) {
        self.vars.insert(name.to_string(), val);
    }

    fn base_env() -> HashMap<String, LispExp> {
        let mut env = HashMap::new();

        // builtin
        env.insert(
            "lambda".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                // (lambda (a b) (+ a b))
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!(
                        "invalid lambda expression: {:?}",
                        exps
                    )));
                }
                Ok(LispExp::Lambda {
                    params: Box::new(exps[0].clone()),
                    body: Box::new(exps[1].clone()),
                    env: Env::extend(env.clone()),
                })
            }),
        );

        env.insert(
            "define".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                // (define (a b) (+ a b))
                // (define a 10)
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!(
                        "invalid define expression: {:?}",
                        exps
                    )));
                }
                if let LispExp::List(fdef) = exps[0].to_owned() {
                    // is a function
                    // convert to lambda
                    env.assign(
                        fdef.first().unwrap().expect_symbol()?.as_str(),
                        LispExp::Lambda {
                            params: Box::new(LispExp::List(fdef.into_iter().skip(1).collect())),
                            body: Box::new(exps[1].to_owned()),
                            env: env.clone(),
                        },
                    );
                } else {
                    // is a variable
                    env.assign(exps[0].expect_symbol()?.as_str(), exps[1].clone());
                }
                Ok(LispExp::List(vec![]))
            }),
        );

        env.insert(
            "+".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!("invalid + expression: {:?}", exps)));
                }
                let lhs = eval(exps[0].clone(), env)?.expect_number()?;
                let rhs = eval(exps[1].clone(), env)?.expect_number()?;
                Ok(LispExp::Number(lhs + rhs))
            }),
        );

        env.insert(
            "-".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!("invalid - expression: {:?}", exps)));
                }
                let lhs = eval(exps[0].clone(), env)?.expect_number()?;
                let rhs = eval(exps[1].clone(), env)?.expect_number()?;
                Ok(LispExp::Number(lhs - rhs))
            }),
        );

        env.insert(
            "*".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!("invalid * expression: {:?}", exps)));
                }
                let lhs = eval(exps[0].clone(), env)?.expect_number()?;
                let rhs = eval(exps[1].clone(), env)?.expect_number()?;
                Ok(LispExp::Number(lhs * rhs))
            }),
        );

        env.insert(
            "/".into(),
            LispExp::Procedure(|exps, env| -> Result<LispExp, LispError> {
                if exps.len() < 2 {
                    return Err(LispError::Eval(format!("invalid / expression: {:?}", exps)));
                }
                let lhs = eval(exps[0].clone(), env)?.expect_number()?;
                let rhs = eval(exps[1].clone(), env)?.expect_number()?;
                Ok(LispExp::Number(lhs / rhs))
            }),
        );

        env
    }
}
