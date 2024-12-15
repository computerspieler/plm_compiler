use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Type {
    Void,
    
	U8,
    U16,
    I8,
    I16,

	Number,
    Pointer(Box<Type>),
    Reference(Box<Type>)
}

impl Type {
	pub fn is_numerical(&self) -> bool {
		match self {
		Type::U8 |
		Type::U16 |
		Type::I8 |
		Type::I16 |
		Type::Number => { true }
		_ => { false }	
		}
	}

	pub fn is_address(&self) -> bool {
		match self {
		Type::Pointer(_) |
		Type::Reference(_) => { true }
		_ => { false }	
		}
	}

	pub fn is_value(&self) -> bool {
		return self.is_numerical() || self.is_address();
	}
}

impl Ord for Type {
	fn cmp(&self, other: &Self) -> Ordering {
	match (self, other) {
	(Type::Void, Type::Void) => { Ordering::Equal }
	(Type::Void, _) => { Ordering::Less }
	(_, Type::Void) => { Ordering::Greater }

	(Type::I8, Type::I8) |
	(Type::U8, Type::I8) |
	(Type::I8, Type::U8) |
	(Type::U8, Type::U8) => { Ordering::Equal }
	
	(Type::I16, Type::I16) |
	(Type::U16, Type::I16) |
	(Type::I16, Type::U16) |
	(Type::U16, Type::U16) => { Ordering::Equal }

	(Type::I16, Type::I8) |
	(Type::U16, Type::I8) |
	(Type::I16, Type::U8) |
	(Type::U16, Type::U8) => { Ordering::Greater }

	(Type::I8, Type::I16) |
	(Type::U8, Type::I16) |
	(Type::I8, Type::U16) |
	(Type::U8, Type::U16) => { Ordering::Less }

	(Type::Pointer(_), Type::Pointer(_)) => { Ordering::Equal }
	(Type::Pointer(_), _) => { Ordering::Greater }
	(_, Type::Pointer(_)) => { Ordering::Less }

	(t1, Type::Reference(t2)) => { Type::cmp(t1, t2) }
	(Type::Reference(t1), t2) => { Type::cmp(t1, t2) }

	(Type::Number, t) => {
		if t.is_numerical() {
			Ordering::Equal
		} else {
			Ordering::Less
		}
	}
	(t, Type::Number) => {
		if t.is_numerical() {
			Ordering::Equal
		} else {
			Ordering::Greater
		}
	}

	}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
	Add,
	AddWithCarry,
	Substract,
	SubstractWithCarry,

	Multiply,
	Division,
	Modulo,

	ShiftLeft,
	ShiftLeftWithCarry,
	ShiftRight,
	ShiftRightWithCarry,
	
	RotateLeft,
	RotateLeftWithCarry,
	RotateRight,
	RotateRightWithCarry,

	And,
	Or,
	Xor,

	Greater,
	Less,
	GreaterOrEqual,
	LessOrEqual,
	Equal,
	NotEqual
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
	Not,
	Invert,
	Dereference,
	Reference
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
	Value(i32, Type),
	Array(Vec<i32>, Type),
	ReadOnlyArray(Vec<i32>, Type)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<VariableType> {
	BinaryOp(BinaryOperation,
		Box<Expression<VariableType>>,
		Box<Expression<VariableType>>
	),
	UnaryOp(UnaryOperation,
		Box<Expression<VariableType>>
	),
	FunctionCall(String,
		Vec<Expression<VariableType>>
	),
	Variable(VariableType),
	Constant(Constant)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable<VariableType> {
    m_name: VariableType,
    m_type: Type
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<VariableType> {
	IfElse(Expression<VariableType>,
		Vec<Statement<VariableType>>,
		Vec<Statement<VariableType>>
	),
	Block(Vec<Statement<VariableType>>),
	Loop(Vec<Statement<VariableType>>),
	Switch(Expression<VariableType>,
		Vec<Option<Statement<VariableType>>>
	),
	Phi(VariableType, Vec<VariableType>),
	
	Return(Option<Expression<VariableType>>),
	Expression(Expression<VariableType>),
	FunctionDefinition(String, Type,
		Vec<Variable<VariableType>>,
		Vec<Statement<VariableType>>
	),
	Assignment(VariableType, Expression<VariableType>),

	Label(String),
	Jump(Expression<VariableType>),
	DisableInterrupt,
	EnableInterrupt,
	Halt,
	NoOperation
}

impl<VariableType> Statement<VariableType> {

}