use crate::abstract_syntax_trees::{
    binary_expression::BinaryExpression, character_expression::CharacterExpression,
    character_literal::CharacterLiteral, command::Command, expression::Expression,
    identifier::Identifier, if_expression::IfExpression, integer_expression::IntegerExpression,
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

    fn accept(&mut self, expected_kind: TokenKind) -> Result<(), SyntaxError> {
        let token = self.scan();

        if token.kind == expected_kind {
            Ok(())
        } else {
            let message = format!("{} expected here", expected_kind.get_spelling());
            Err(self.syntactic_error(&token, &message))
        }
    }

    fn pos_start(&self) -> SourcePosition {
        match &self.current_token {
            Some(Token { position, .. }) => SourcePosition::start_from(position),
            None => SourcePosition::new(),
        }
    }

    fn pos_finish(&self, pos: &mut SourcePosition) {
        if let Some(Token { position, .. }) = &self.current_token {
            pos.finish = position.start;
        } else {
            todo!()
        }
    }

    fn is_current_token_kind(&self, the_kind: TokenKind) -> bool {
        match &self.current_token {
            Some(Token { kind, .. }) => the_kind == *kind,
            _ => false,
        }
    }

    fn syntactic_error(&mut self, token: &Token, message_template: &str) -> SyntaxError {
        self.error_reporter
            .report_error(message_template, &token.spelling, &token.position);

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
        Err(parser.syntactic_error(&token, error_ms))
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

fn parse_operator_literal(parser: &mut Parser) -> Result<Operator, SyntaxError> {
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

fn parse_expression(parser: &mut Parser) -> Result<Box<dyn Expression>, SyntaxError> {
    // Save start of expression
    let mut expression_pos = parser.pos_start();

    // match token with an expression
    match parser.scan() {
        Token {
            kind: TokenKind::IF,
            ..
        } => {
            let e1_ast = parse_expression(parser)?;
            parser.accept(TokenKind::THEN)?;

            let e2_ast = parse_expression(parser)?;
            parser.accept(TokenKind::ELSE)?;

            let e3_ast = parse_expression(parser)?;
            parser.pos_finish(&mut expression_pos);

            let exp = IfExpression::new(e1_ast, e2_ast, e3_ast, expression_pos);

            return Ok(Box::new(exp));
        }
        _ => {
            let _ = parse_secondary_expression(parser);
            todo!()
        }
    }
}

fn parse_secondary_expression(parser: &mut Parser) -> Result<Box<dyn Expression>, SyntaxError> {
    // Save start of command
    let secondary_expression_pos = parser.pos_start();

    let mut expression = parse_primary_expression(parser)?;

    while parser.is_current_token_kind(TokenKind::OPERATOR) {
        let op_ast = parse_operator_literal(parser)?;
        let e2_ast = parse_primary_expression(parser)?;

        expression = Box::new(BinaryExpression::new(
            expression,
            op_ast,
            e2_ast,
            secondary_expression_pos.clone(),
        ));
    }

    Ok(expression)
}

fn parse_primary_expression(parser: &mut Parser) -> Result<Box<dyn Expression>, SyntaxError> {
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
            let exp = IntegerExpression::new(int_lit_ast, primary_expression_pos);
            Ok(Box::new(exp))
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
            let exp = CharacterExpression::new(char_lit_ast, primary_expression_pos);
            Ok(Box::new(exp))
        }
        token => Err(parser.syntactic_error(&token, "\"%\" cannot start an expression")),
    }
}
