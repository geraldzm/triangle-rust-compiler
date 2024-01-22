use crate::abstract_syntax_trees::{
    character_expression::CharacterExpression, character_literal::CharacterLiteral,
    command::Command, identifier::Identifier, integer_expression::IntegerExpression,
    integer_literal::IntegerLiteral, operator::Operator,
};

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

    fn pos_start(&self) -> SourcePosition {
        match &self.current_token {
            Some(Token { position, .. }) => SourcePosition::start_from(position),
            None => SourcePosition::new(),
        }
    }

    fn syntactic_error(
        &mut self,
        token: &Token,
        message_template: &str,
        token_quoted: &str,
    ) -> SyntaxError {
        self.error_reporter
            .report_error(message_template, token_quoted, &token.position);

        SyntaxError
    }
}

// ----------------------------------------------------------------------------
//
// LITERALS
//
// ----------------------------------------------------------------------------

fn parse_literal<F, G>(
    parser: &mut Parser,
    kind: TokenKind,
    error_ms: &'static str,
    new: F,
) -> Result<G, SyntaxError>
where
    F: Fn(String, SourcePosition) -> G,
{
    let token = parser.scan();

    if token.kind == kind {
        Ok(new(token.spelling, token.position))
    } else {
        Err(parser.syntactic_error(&token, error_ms, ""))
    }
}

fn _parse_integer_literal(parser: &mut Parser) -> Result<IntegerLiteral, SyntaxError> {
    // match token with Int Literal
    parse_literal(
        parser,
        TokenKind::INTLITERAL,
        "integer literal expected here",
        IntegerLiteral::new,
    )
}

fn _parse_character_literal(parser: &mut Parser) -> Result<CharacterLiteral, SyntaxError> {
    // match token with Char Literal
    parse_literal(
        parser,
        TokenKind::CHARLITERAL,
        "character literal expected here",
        CharacterLiteral::new,
    )
}

fn _parse_identifier_literal(parser: &mut Parser) -> Result<Identifier, SyntaxError> {
    // match token with identifier Literal
    parse_literal(
        parser,
        TokenKind::IDENTIFIER,
        "identifier literal expected here",
        Identifier::new,
    )
}

fn _parse_operator_literal(parser: &mut Parser) -> Result<Operator, SyntaxError> {
    // match token with operator Literal
    parse_literal(
        parser,
        TokenKind::OPERATOR,
        "operator literal expected here",
        Operator::new,
    )
}
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

// ----------------------------------------------------------------------------
//
// EXPRESSIONS
//
// ----------------------------------------------------------------------------

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

fn _parse_primary_expression(parser: &mut Parser) -> Result<(), SyntaxError> {
    // Save start of command
    let mut primary_expression_pos = parser.pos_start();

    // match token with an primary expression
    match parser.scan() {
        Token {
            kind: TokenKind::INTLITERAL,
            spelling,
            position,
        } => {
            // save end of the expression
            primary_expression_pos.finish = position.finish;

            // create AST struct
            let int_lit_ast = IntegerLiteral::new(spelling, position);

            // create Expression
            IntegerExpression::new(int_lit_ast, primary_expression_pos);
        }
        Token {
            kind: TokenKind::CHARLITERAL,
            spelling,
            position,
        } => {
            // save end of the expression
            primary_expression_pos.finish = position.finish;

            // create AST struct
            let char_lit_ast = CharacterLiteral::new(spelling, position);

            // create Expression
            CharacterExpression::new(char_lit_ast, primary_expression_pos);
        }
        token => {
            return Err(parser.syntactic_error(
                &token,
                "\"%\" cannot start an expression",
                &token.spelling,
            ));
        }
    }

    todo!()
}
