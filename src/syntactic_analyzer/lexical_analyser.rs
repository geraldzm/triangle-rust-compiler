use super::source_file::SourceFile;

fn is_letter(c: &char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_digit(c: &char) -> bool {
    c.is_ascii_alphanumeric()
}

fn is_operator(c: &char) -> bool {
    matches!(
        *c,
        '+' | '-' | '*' | '/' | '=' | '<' | '>' | '\\' | '&' | '@' | '%' | '^' | '?'
    )
}

pub struct Scanner {
    source_file: SourceFile,
    current_char: Option<char>,
    current_spelling: String,
    currently_scanning_token: bool,
}

impl Scanner {
    /// Appends the current character to the current token, and gets
    /// the next character from the source program.
    ///
    /// # Panics
    ///
    /// Panics if the current_char value equals [`None`].
    ///
    fn take_it(&mut self) {
        if self.currently_scanning_token {
            self.current_spelling.push(self.current_char.unwrap());
        }

        self.current_char = self.source_file.next();
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_operator_test() {
        let operator = '>';
        assert!(is_operator(&operator));

        let operator = '?';
        assert!(is_operator(&operator));

        let operator = '\\';
        assert!(is_operator(&operator));

        let operator = 'z';
        assert!(!is_operator(&operator));
    }
}
