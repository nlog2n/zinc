//!
//! The generator expression place operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::Instruction;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::operand::constant::Constant;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant as SemanticConstant;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::Place as SemanticPlace;

#[derive(Debug, Clone)]
pub struct Place {
    pub location: Location,
    pub identifier: String,
    pub element_size: usize,
    pub total_size: usize,
    pub elements: Vec<SemanticPlaceElement>,
}

impl Place {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        if !self.elements.is_empty() {
            Constant::new_integer(BigInt::zero(), false, crate::BITLENGTH_FIELD)
                .write_all_to_bytecode(bytecode.clone());
        }
        for element in self.elements.into_iter() {
            match element {
                SemanticPlaceElement::IndexConstant { constant, access } => {
                    Constant::try_from_semantic(&SemanticConstant::Integer(constant))
                        .expect(crate::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS)
                        .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.location),
                    );
                    Constant::new_integer(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.location),
                    );
                }
                SemanticPlaceElement::IndexExpression { expression, access } => {
                    expression.write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.location),
                    );
                    Constant::new_integer(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.location),
                    );
                }
                SemanticPlaceElement::IndexRange { start, access, .. } => {
                    Constant::new_integer(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.location),
                    );
                }
                SemanticPlaceElement::IndexRangeInclusive { start, access, .. } => {
                    Constant::new_integer(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.location),
                    );
                }
                SemanticPlaceElement::Field { access } => {
                    Constant::new_integer(
                        BigInt::from(access.offset),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.location),
                    );
                }
            }
        }
    }
}

impl From<SemanticPlace> for Place {
    fn from(place: SemanticPlace) -> Self {
        Self {
            location: place.location,
            identifier: place.identifier,
            element_size: place.r#type.size(),
            total_size: place.total_size,
            elements: place.elements,
        }
    }
}
