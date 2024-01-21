use crate::abstract_syntax_trees::command::Command;

use super::{
    errors::{ErrorReporter, SyntaxError},
    scanner::Scanner,
    source_position::SourcePosition,
    token::{Token, TokenKind},
};

struct Parser {
    pub lexical_analyser: Scanner,
    pub error_reporter: ErrorReporter,
}

impl Parser {
    fn scan(&mut self) -> Token {
        self.scan()
    }
}

fn syntactic_error(
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
    let mut previous_position = SourcePosition::new();

    // parse command
    parse_command(&mut parser);
}

// ----------------------------------------------------------------------------
//
// COMMANDS
//
// ----------------------------------------------------------------------------

fn parse_command(parser: &mut Parser) -> Result<(), SyntaxError> {
    // -----
    let mut command_pos = SourcePosition::new();
    // -----

    parse_single_command(&command_pos, parser);
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

fn parse_single_command(
    command_start: &SourcePosition,
    parser: &mut Parser,
) -> Result<Command, SyntaxError> {
    // Save start of command
    let mut single_command_pos = SourcePosition::start_from(&command_start);

    // match token with a command
    match current_token {
        Some(Token {
            kind: TokenKind::WHILE,
            position,
            spelling,
        }) => {
            // accept_it(, lexical_analyser)
        }
        _ => todo!(),
    }

    todo!()
}
