use crate::ast::{Expression, Type};

trait Architecture {
    fn get_pointer_numerical_type() -> Type;

    fn get_expression_output_size<VariableType>(&self, e: Expression<VariableType>) -> usize;
}
