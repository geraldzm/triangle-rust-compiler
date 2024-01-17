use crate::abstract_syntax_trees::{integer_literal::IntegerLiteral, terminal::Terminal};

use super::{
    errors::{ErrorReporter, SyntaxError},
    scanner::Scanner,
    source_position::SourcePosition,
    token::{Token, TokenKind},
};

pub struct Parser {
    lexical_analyser: Scanner,
    error_reporter: ErrorReporter,
    current_token: Option<Token>,
    previous_position: SourcePosition,
}

impl Parser {
    pub fn new(lexical_analyser: Scanner, error_reporter: ErrorReporter) -> Self {
        Parser {
            lexical_analyser,
            error_reporter,
            current_token: None,
            previous_position: SourcePosition::new(),
        }
    }

    /// Returns the String (spelling) of current token (if any) and scans a new token
    /// into current_token
    ///  
    fn accept_it(&mut self) -> Option<String> {
        match self.current_token.replace(self.lexical_analyser.scan()) {
            Some(token) => {
                self.previous_position = token.position;
                Some(token.spelling)
            }
            _ => None,
        }
    }

    /// start records the position of the start of a phrase.
    /// This is defined to be the position of the first
    /// character of the first token of the phrase.
    fn start(&self) -> SourcePosition {
        match &self.current_token {
            Some(token) => SourcePosition::new_with(token.position().start, 0),
            _ => SourcePosition::new(),
        }
    }

    /// finish records the position of the end of a phrase.
    /// This is defined to be the position of the last
    /// character of the last token of the phrase.
    fn finish(&self) -> SourcePosition {
        match &self.current_token {
            Some(token) => SourcePosition::new_with(0, token.position().finish),
            _ => SourcePosition::new(),
        }
    }

    fn syntactic_error(&mut self, message_template: &str, token_quoted: &str) -> SyntaxError {
        let mut pos = &SourcePosition::new();

        if let Some(token) = &self.current_token {
            pos = token.position();
        }

        self.error_reporter
            .report_error(message_template, token_quoted, pos);

        SyntaxError
    }

    // ----------------------------------------------------------------------------
    //
    // PROGRAMS
    //
    // ----------------------------------------------------------------------------
    pub fn parse_program(&mut self) {
        // Program programAST = null;

        // read first token
        self.accept_it();

        // parse command
        self.parse_command();

        // try {
        //   Command cAST = parseCommand();
        //   programAST = new Program(cAST, previousTokenPosition);
        //   if (currentToken.kind != Token.EOT) {
        //     syntacticError("\"%\" not expected after end of program", currentToken.spelling);
        //   }
        // }
        // catch (SyntaxError s) { return null; }
        // return programAST;
    }

    // ----------------------------------------------------------------------------
    //
    // LITERALS
    //
    // ----------------------------------------------------------------------------

    // parseIntegerLiteral parses an integer-literal, and constructs
    // a leaf AST to represent it.
    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, SyntaxError> {
        let err = Err(self.syntactic_error("integer literal expected here", ""));

        match &self.current_token {
            Some(token) if matches!(token.kind(), TokenKind::INTLITERAL) => {
                if let Some(spelling) = self.accept_it() {
                    return Ok(IntegerLiteral::new(
                        spelling,
                        self.previous_position.clone(),
                    ));
                }

                err
            }
            _ => err,
        }
    }

    // ----------------------------------------------------------------------------
    //
    // COMMANDS
    //
    // ----------------------------------------------------------------------------
    fn parse_command(&mut self) -> Result<(), SyntaxError> {
        //  Command commandAST = null; // in case there's a syntactic error

        // -----
        // SourcePosition commandPos = new SourcePosition();
        // start(commandPos);
        let mut command_pos = self.start();
        // -----

        // commandAST = parseSingleCommand();
        // while (currentToken.kind == Token.SEMICOLON) {
        //   acceptIt();
        //   Command c2AST = parseSingleCommand();
        //   finish(commandPos);
        //   commandAST = new SequentialCommand(commandAST, c2AST, commandPos);
        // }
        // return commandAST;

        Err(self.syntactic_error("sdkfj", ""))
    }
}
