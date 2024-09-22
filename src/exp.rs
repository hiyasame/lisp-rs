use crate::{env::Env, error::LispError};

// parser 得到的结果
// 同时也是 eval 得到的结果
// lisp 中 code is data
#[derive(Clone, Debug, PartialEq)]
pub enum LispExp {
    Number(f64),
    Bool(bool),
    Symbol(String),
    List(Vec<LispExp>),
    Lambda {
        params: Box<LispExp>,
        body: Box<LispExp>,
        env: Env
    },
    // 内建 procedure, 用户 define 的 procedure 都是 lambda
    Procedure(fn(&Vec<LispExp>, &mut Env) -> Result<LispExp, LispError>)
}

impl LispExp {
    pub fn as_list(self) -> Self {
        match self {
            Self::Bool(bool) => Self::List(vec![Self::Bool(bool)]),
            Self::Number(num) => Self::List(vec![Self::Number(num)]),
            Self::Symbol(sym) => Self::List(vec![Self::Symbol(sym)]),
            _ => self
        }
    }

    pub fn expect_symbol(&self) -> Result<String, LispError> {
        match self {
            Self::Symbol(sym) => Ok(sym.into()),
            _ => Err(LispError::Eval(format!("expect a symbol but find: {:?}", self)))
        }
    }

    pub fn expect_number(&self) -> Result<f64, LispError> {
        match self {
            Self::Number(num) => Ok(*num),
            _ => Err(LispError::Eval(format!("expect a symbol but find: {:?}", self)))
        }
    }

    pub fn expect_list(&self) -> Result<&Vec<LispExp>, LispError> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(LispError::Eval(format!("expect a symbol but find: {:?}", self)))
        }
    }
}