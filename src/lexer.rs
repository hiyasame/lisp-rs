use crate::error::LispError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    // 有一些比较特殊的符号，不过 lisp 里面 code is data
    // 我觉得不需要在分词的时候就分得太清楚
    Symbol(String),
    LParen,
    RParen
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LispError> {
    let mut tokens = Vec::new();
    let mut chars: Vec<char> = input.chars().filter(|&c| c != '\n').collect();

    if chars.is_empty() {
        return Ok(tokens)
    }

    while !chars.is_empty() {
        let mut ch = chars.remove(0);
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            // '\'' => {
            //     tokens.push(Token::Quote)
            // },
            ' ' => {}
            // symbol
            _ => {
                let mut str = String::new();
                str.push(ch);
                while !chars.is_empty() && !chars[0].is_whitespace() {
                    let peek = chars[0];
                    if peek == '(' || peek == ')' {
                        break;
                    }
                    ch = chars.remove(0);
                    str.push(ch);
                }

                if !chars.is_empty() && chars[0].is_whitespace() {
                    chars.remove(0);
                }

                if str.is_empty() {
                    continue;
                }
                
                // 数字
                if let Ok(i) = str.parse::<f64>() {
                    tokens.push(Token::Number(i));
                } else {
                    tokens.push(Token::Symbol(str));
                }
            }
        }
    }

    Ok(tokens)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Symbol("+".into()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::RParen
        ])
    }

    #[test]
    
    fn test_complex() {
        let tokens = tokenize("(
                (define r 10)
                (define pi 3.14)
                (* pi (* r r))
        )").unwrap();
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::LParen,
            Token::Symbol("define".into()),
            Token::Symbol("r".into()),
            Token::Number(10.0),
            Token::RParen,
            Token::LParen,
            Token::Symbol("define".into()),
            Token::Symbol("pi".into()),
            #[allow(clippy::approx_constant)]
            Token::Number(3.14),
            Token::RParen,
            Token::LParen,
            Token::Symbol("*".into()),
            Token::Symbol("pi".into()),
            Token::LParen,
            Token::Symbol("*".into()),
            Token::Symbol("r".into()),
            Token::Symbol("r".into()),
            Token::RParen,
            Token::RParen,
            Token::RParen
        ])
    }
}