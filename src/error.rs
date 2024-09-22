use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LispError {
    #[error("Lexer error: `{0}`")]
    Lexer(String),
    #[error("Parser error: `{0}`")]
    Parser(String),
    #[error("Eval error: `{0}`")]
    Eval(String)
}