use crate::{error::LispError, lexer::Token, exp::LispExp};

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<LispExp>, LispError> {
    let mut exps_stack: Vec<Vec<LispExp>> = vec![
        Vec::new()
    ];

    while !tokens.is_empty() {
        let token = tokens.remove(0);
        match token {
            Token::Number(f) => {
                exps_stack.last_mut().unwrap().push(LispExp::Number(f));
            }
            Token::Symbol(s) => {
                let exp = match s.as_str() {
                    "true" => LispExp::Bool(true),
                    "false" => LispExp::Bool(false),
                    _ => LispExp::Symbol(s)
                };
                exps_stack.last_mut().unwrap().push(exp);
            }
            Token::LParen => {
                exps_stack.push(Vec::new());
            }
            Token::RParen => {
                let list = exps_stack.pop().unwrap();
                if let Some(exps) = exps_stack.last_mut() {
                    exps.push(LispExp::List(list));
                } else {
                    return Err(LispError::Parser("redundant right paren".into()))
                }
            }
        }
    }

    if exps_stack.len() != 1 {
        return Err(LispError::Parser("unclosed left paren".into()))
    }

    Ok(exps_stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn test_simple() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        let exp = parse(tokens).unwrap();
        assert_eq!(exp, vec![
            LispExp::List(
                vec![
                    LispExp::Symbol("+".into()),
                    LispExp::Number(1.0),
                    LispExp::Number(2.0)
                ]
            )
        ]);
    }

    // 不想写测例（
    #[test]
    fn test_lambda() {
        let tokens = tokenize("(lambda (a) (+ a 1))").unwrap();
        let exp = parse(tokens).unwrap();
    }
}