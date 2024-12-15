use crate::ast::*;

pub trait TypeCheckable<Environment> {
    fn get_type(&self, env: &Environment) -> Option<Type>;
}

/* TODO: Errors */
impl<Environment, VariableType: TypeCheckable<Environment>> TypeCheckable<Environment> 
for Expression<VariableType>
{
    fn get_type(&self, env: &Environment) -> Option<Type> {
        match self {
        Expression::BinaryOp(op, lhs, rhs) => {
            let (lhs_type, rhs_type) =
                (lhs.get_type(env), rhs.get_type(env));
            if lhs_type.is_none() || rhs_type.is_none() {
                return None;
            }

            let (lhs_type, rhs_type) =
                (lhs_type.unwrap(), rhs_type.unwrap());
            
            match op {
            BinaryOperation::Add |
            BinaryOperation::AddWithCarry |
            BinaryOperation::Substract |
            BinaryOperation::SubstractWithCarry |
            
            BinaryOperation::Multiply |
            BinaryOperation::Division |
            BinaryOperation::Modulo |

            BinaryOperation::And |
            BinaryOperation::Or |
            BinaryOperation::Xor => {
                if lhs_type.is_value() && rhs_type.is_value() {
                    Some(lhs_type.max(rhs_type))
                } else {
                    None
                }
            }

            BinaryOperation::Equal |
            BinaryOperation::NotEqual |
            BinaryOperation::Greater |
            BinaryOperation::GreaterOrEqual |
            BinaryOperation::Less |
            BinaryOperation::LessOrEqual => {
                if lhs_type == rhs_type && lhs_type.is_value() {
                    Some(Type::U8)
                } else {
                    None
                }
            }

            BinaryOperation::ShiftLeft |
            BinaryOperation::ShiftLeftWithCarry |
            BinaryOperation::ShiftRight |
            BinaryOperation::ShiftRightWithCarry |
            BinaryOperation::RotateLeft |
            BinaryOperation::RotateLeftWithCarry |
            BinaryOperation::RotateRight |
            BinaryOperation::RotateRightWithCarry => {
                if lhs_type.is_value() && rhs_type.is_numerical() {
                    Some(lhs_type)
                } else {
                    None
                }
            }
            }
        }
        Expression::UnaryOp(op, elt) => {
            match (*elt).get_type(env) {
            None => { None }
            Some(t) => {
                match op {
                UnaryOperation::Invert |
                UnaryOperation::Not => {
                    if t.is_numerical() {
                        Some(t)
                    } else {
                        None
                    }
                }
                UnaryOperation::Reference |
                UnaryOperation::Dereference => {
                    if t.is_address() {
                        Some(t)
                    } else {
                        None
                    }
                }
                }
            }
            }
        }
        Expression::Constant(Constant::Value(_, t)) => {
            Some(t.clone())
        }
        Expression::Constant(Constant::ReadOnlyArray(_, t)) |
        Expression::Constant(Constant::Array(_, t)) => {
            Some(Type::Pointer(Box::new(t.clone())))
        }
        Expression::Variable(var) => {
            var.get_type(env)
        }
        Expression::FunctionCall(_name, _args) => {
            None
        }
        }
    }
}
