use crate::abstract_syntax_trees::command::Command;

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
}

impl Parser {
    pub fn new(mut lexical_analyser: Scanner, error_reporter: ErrorReporter) -> Self {
        let current_token = lexical_analyser.scan();
        Self {
            lexical_analyser,
            error_reporter,
            current_token: Some(current_token),
        }
    }

    fn scan(&mut self) -> Token {
        match self.current_token.replace(self.lexical_analyser.scan()) {
            Some(t) => t,
            None => Token::error_token(),
        }
    }

    fn pos<F>(&self, f: F) -> SourcePosition
    where
        F: Fn(&SourcePosition) -> SourcePosition,
    {
        match &self.current_token {
            Some(Token { position, .. }) => f(position),
            None => SourcePosition::new(),
        }
    }

    fn pos_start(&self) -> SourcePosition {
        self.pos(SourcePosition::start_from)
    }

    fn _pos_finish(&self) -> SourcePosition {
        self.pos(SourcePosition::finish_from)
    }
}

fn _syntactic_error(
    current_token: &Option<Token>,
    error_reporter: &mut ErrorReporter,
    message_template: &str,
    token_quoted: &str,
) -> SyntaxError {
    let mut pos = &SourcePosition::new();

    if let Some(token) = &current_token {
        pos = token.position();
    }

    error_reporter.report_error(message_template, token_quoted, pos);

    SyntaxError
}

// ----------------------------------------------------------------------------
//
// LITERALS
//
// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------
//
// PROGRAMS
//
// ----------------------------------------------------------------------------
pub fn parse_program(mut parser: Parser) {
    let mut _previous_position = SourcePosition::new();

    // parse command
    let _ = parse_command(&mut parser);
}

// ----------------------------------------------------------------------------
//
// COMMANDS
//
// ----------------------------------------------------------------------------

fn parse_command(parser: &mut Parser) -> Result<(), SyntaxError> {
    // -----
    let mut _command_pos = parser.pos_start();
    // -----

    let _ = parse_single_command(parser);
    // commandAST = parseSingleCommand();
    // while (currentToken.kind == Token.SEMICOLON) {
    //   acceptIt();
    //   Command c2AST = parseSingleCommand();
    //   finish(commandPos);
    //   commandAST = new SequentialCommand(commandAST, c2AST, commandPos);
    // }
    // return commandAST;

    todo!()
}

fn parse_single_command(parser: &mut Parser) -> Result<Command, SyntaxError> {
    // Save start of command
    let mut _single_command_pos = parser.pos_start();

    // match token with a command
    match parser.scan() {
        Token {
            kind: TokenKind::WHILE,
            position: _,
            spelling: _,
        } => {
            let _ = parse_expression(parser);
        }
        _ => todo!(),
    }

    todo!()
}

///////////////////////////////////////////////////////////////////////////////
//
// EXPRESSIONS
//
///////////////////////////////////////////////////////////////////////////////

fn parse_expression(parser: &mut Parser) -> Result<(), SyntaxError> {
    // Save start of command
    let mut _expression_pos = parser.pos_start();

    // match token with a command
    match parser.scan() {
        _ => {
            let _ = parse_secondary_expression(parser);
        } // _ => todo!(),
    }

    todo!()
}

fn parse_secondary_expression(parser: &mut Parser) -> Result<(), SyntaxError> {
    // Save start of command
    let mut _secondary_expression_pos = parser.pos_start();

    // match token with a command
    // match parser.scan() {
    //     _ => {
    //         (parser);
    //     } // _ => todo!(),
    // }

    todo!()
}
