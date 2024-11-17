#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
	Add,
	AddWithCarry,
	Substract,
	SubstractWithCarry,

	Multiply,
	Division,
	Modulo,

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
	ExtractAddress
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
	ArrayIndex(String, Box<Expression>),
	Variable(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableInitialValue {
	Value(i32),
	Array(Vec<i32>),
	ReadOnlyArray(Vec<i32>),
	ValueOfPointer(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	BinaryOp(BinaryOperation, Box<Expression>, Box<Expression>),
	UnaryOp(UnaryOperation, Box<Expression>),
	Identifier(String),
	String(String),
	/* The issue is that both share similar syntaxes.
	 * Example: arr(5), f(6).
	 *  The first one is the fifth element of the array,
	 *  and the seconde one is a call to a function.
	 * So for now, we'll act like they're the same.
	 */
	FunctionCallOrArrayElement(String, Box<Expression>),
	FunctionCall(String, Vec<Expression>),
	Constant(i32),
	VariableAssignment(Variable, Box<Expression>),
	AddressOfConstant(VariableInitialValue)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type  {
	Byte(usize),
	Address(usize),
	Data,	// It's like byte(n), but n is unknown
	Macro,	// This one is just used by the compiler
	Void	// Used for precedure who doesn't return anything
} 

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
	IfElse(Expression, Box<Statement>, Box<Statement>),
	Block(Vec<Statement>),
	While(Expression, Box<Statement>),
	IterativeLoop(String, Expression, Expression, Box<Statement>),
	DoCase(Expression, Vec<Statement>),
	GoToValue(i32),
	GoToIdentifier(String),
	DisableInterrupt,
	EnableInterrupt,
	Halt,
	VariableDeclaration(Vec<String>, Vec<Type>, Vec<Option<VariableInitialValue>>),
	EndOfStatement(Option<String>),
	FunctionCall(String, Vec<Expression>),
	Return(Option<Expression>),
	Expression(Expression),
	Label(String, Box<Statement>),
	Procedure(Vec<String>, Type, Box<Statement>),
	ProgramBasis(i32),
	EndOfFile,
	NoOperation
}
