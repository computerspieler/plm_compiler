use crate::ast::{Expression, Type};

pub trait Architecture {
	fn get_pointer_numerical_type(&self) -> Type;
	fn get_expression_output_size<VariableType>(&self, e: Expression<VariableType>) -> usize;
	fn has_floating_point(&self) -> bool;
}
