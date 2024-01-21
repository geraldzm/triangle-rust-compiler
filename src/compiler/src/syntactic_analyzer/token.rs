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

#[cfg(test)]
mod tests {}
