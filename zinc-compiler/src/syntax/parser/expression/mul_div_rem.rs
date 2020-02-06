//!
//! The multiplication/division/remainder operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::CastingOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    CastingFirstOperand,
    CastingOperator,
    CastingSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::CastingFirstOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::CastingFirstOperand => {
                    let (expression, next) =
                        CastingOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location_if_unset(expression.location);
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::CastingOperator;
                }
                State::CastingOperator => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::As),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Casting));
                            self.state = State::CastingSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::CastingSecondOperand => {
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder
                        .push_operand(r#type.location, ExpressionOperand::Type(r#type));
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::CastingOperator;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let input = r#"42 as field"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 1),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::Type(Type::new(
                            Location::new(1, 7),
                            TypeVariant::new_field(),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 4),
                        ExpressionObject::Operator(ExpressionOperator::Casting),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 12))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
