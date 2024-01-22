use std::fmt::Display;

use super::source_position::SourcePosition;

pub struct Token {
    pub kind: TokenKind,
    pub position: SourcePosition,
    pub spelling: String,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String, position: SourcePosition) -> Token {
        Token {
            kind,
            spelling,
            position,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn spelling(&self) -> &String {
        &self.spelling
    }

    pub fn position(&self) -> &SourcePosition {
        &self.position
    }

    pub fn error_token() -> Self {
        Self {
            kind: TokenKind::ERROR,
            spelling: String::new(),
            position: SourcePosition::new(),
        }
    }
}

#[derive(PartialEq)]
pub enum TokenKind {
    // literals, identifiers, operators...
    INTLITERAL,
    CHARLITERAL,
    IDENTIFIER,
    OPERATOR,

    // reserved words ...
    ARRAY,
    BEGIN,
    CONST,
    DO,
    ELSE,
    END,
    FUNC,
    IF,
    IN,
    LET,
    OF,
    PROC,
    RECORD,
    THEN,
    TYPE,
    VAR,
    WHILE,

    // punctuation...
    DOT,
    COLON,
    SEMICOLON,
    COMMA,
    BECOMES,
    IS,

    // brackets...
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LCURLY,
    RCURLY,

    // special tokens...
    EOT,
    ERROR,
}

impl TokenKind {
    pub fn get_spelling(&self) -> &'static str {
        match self {
            TokenKind::INTLITERAL => "<int>",
            TokenKind::CHARLITERAL => "<char>",
            TokenKind::IDENTIFIER => "<identifier>",
            TokenKind::OPERATOR => "<operator>",
            TokenKind::ARRAY => "array",
            TokenKind::BEGIN => "begin",
            TokenKind::CONST => "const",
            TokenKind::DO => "do",
            TokenKind::ELSE => "else",
            TokenKind::END => "end",
            TokenKind::FUNC => "func",
            TokenKind::IF => "if",
            TokenKind::IN => "in",
            TokenKind::LET => "let",
            TokenKind::OF => "of",
            TokenKind::PROC => "proc",
            TokenKind::RECORD => "record",
            TokenKind::THEN => "then",
            TokenKind::TYPE => "type",
            TokenKind::VAR => "var",
            TokenKind::WHILE => "while",
            TokenKind::DOT => ".",
            TokenKind::COLON => ":",
            TokenKind::SEMICOLON => ";",
            TokenKind::COMMA => ",",
            TokenKind::BECOMES => ":=",
            TokenKind::IS => "~",
            TokenKind::LPAREN => "(",
            TokenKind::RPAREN => ")",
            TokenKind::LBRACKET => "[",
            TokenKind::RBRACKET => "]",
            TokenKind::LCURLY => "{",
            TokenKind::RCURLY => "}",
            TokenKind::EOT => "",
            TokenKind::ERROR => "<error>",
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_spelling())
    }
}

#[cfg(test)]
mod tests {}
