use super::{
    source_file::{self, SourceFile},
    source_position::SourcePosition,
    token::{self, Token},
};

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_operator(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '=' | '<' | '>' | '\\' | '&' | '@' | '%' | '^' | '?'
    )
}

pub struct Scanner {
    source_file: SourceFile,
    current_char: Option<char>,
}

impl Scanner {
    pub fn new(source_file: SourceFile) -> Scanner {
        let mut scanner = Scanner {
            source_file,
            current_char: None,
        };

        // read the first character
        scanner.take_it();

        // return the scanner
        scanner
    }

    /// Appends the current character to the current token, and gets
    /// the next character from the source program.
    ///
    fn take_it_into(&mut self, current_spelling: &mut String) {
        if let Some(c) = self.current_char {
            current_spelling.push(c);
        }

        self.take_it();
    }

    fn take_it(&mut self) {
        self.current_char = self.source_file.next();
    }

    fn is_the_end(&self) -> bool {
        match self.current_char {
            Some(c) => {
                matches!(c, source_file::EOL | source_file::EOT)
            }
            None => true,
        }
    }

    fn scan_separator(&mut self) {
        loop {
            // TODO: remove unwrap
            let current_char = self.current_char.unwrap();

            match current_char {
                '!' => {
                    // line commend, consume all the line
                    self.take_it();
                    while !self.is_the_end() {
                        self.take_it();
                    }

                    if let Some(c) = self.current_char {
                        if c == source_file::EOL {
                            self.take_it();
                        }
                    }
                }
                ' ' | '\n' | '\r' | '\t' => {
                    self.take_it();
                }
                _ => break,
            }
        }
    }

    fn consumer<F>(&mut self, current_spelling: &mut String, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        // take the char that was already read
        self.take_it_into(current_spelling);

        // keep consuming until predicate is false or we run out of chars
        while let Some(c) = self.current_char {
            if predicate(c) {
                self.take_it_into(current_spelling);
            }
        }
    }

    fn match_char(&mut self, current_spelling: &mut String) -> token::TokenKind {
        if let Some(c) = self.current_char {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    // consume all the word
                    self.consumer(current_spelling, |c| is_letter(c) || is_digit(c));

                    // return type
                    return token::TokenKind::IDENTIFIER;
                }
                '0'..='9' => {
                    // consume all the number
                    self.consumer(current_spelling, is_digit);

                    // return type
                    return token::TokenKind::INTLITERAL;
                }

                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '\\' | '&' | '@' | '%' | '^' | '?' => {
                    // consume all the operator
                    self.consumer(current_spelling, is_operator);

                    // return type
                    return token::TokenKind::OPERATOR;
                }

                '\'' => {
                    // consume quoted character
                    self.take_it_into(current_spelling); // take the first '
                    self.take_it_into(current_spelling); // take the quoted character

                    if let Some(c) = self.current_char {
                        if c == '\'' {
                            self.take_it_into(current_spelling); // take the ending '
                            return token::TokenKind::CHARLITERAL;
                        }
                    }

                    return token::TokenKind::ERROR;
                }

                '.' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::DOT;
                }

                ':' => {
                    self.take_it_into(current_spelling);
                    if let Some(c) = self.current_char {
                        if c == '=' {
                            self.take_it_into(current_spelling);
                            return token::TokenKind::BECOMES;
                        }
                    }

                    return token::TokenKind::COLON;
                }

                ';' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::SEMICOLON;
                }

                ',' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::COMMA;
                }

                '~' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::IS;
                }

                '(' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::LPAREN;
                }

                ')' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::RPAREN;
                }

                '[' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::LBRACKET;
                }

                ']' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::RBRACKET;
                }

                '{' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::LCURLY;
                }

                '}' => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::RCURLY;
                }

                source_file::EOT => {
                    self.take_it_into(current_spelling);
                    return token::TokenKind::EOT;
                }

                _ => token::TokenKind::ERROR,
            }
        } else {
            token::TokenKind::ERROR
        }
    }

    fn scan_token(&mut self) -> Token {
        let mut spelling = String::new();

        // save the current line
        let start_line = self.source_file.get_current_line();

        // match current character with a token type
        let kind = self.match_char(&mut spelling);

        // create a source position
        let finish_line = self.source_file.get_current_line();
        let pos = SourcePosition::new_with(start_line, finish_line);

        // return Token result
        Token::new(kind, spelling, pos)
    }

    pub fn scan(&mut self) -> Token {
        // ignore commend lines & separators (spaces, tabs, etc..)
        self.scan_separator();

        // scan a token
        self.scan_token()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_operator_test() {
        let operator = '>';
        assert!(is_operator(operator));

        let operator = '?';
        assert!(is_operator(operator));

        let operator = '\\';
        assert!(is_operator(operator));

        let operator = 'z';
        assert!(!is_operator(operator));
    }
}
