use parser_macros::{IdentifierParser, InstructionLister, InstructionParser};

#[derive(Debug, Clone, IdentifierParser)]
pub enum ByteRegister {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

#[derive(Debug, Clone, IdentifierParser)]
pub enum WordRegister {
	AF,
	BC,
	DE,
	HL,
	#[rename("AF'")] AF_,
	#[rename("BC'")] BC_,
	#[rename("DE'")] DE_,
	#[rename("HL'")] HL_,
	IX,
	IY,
	SP,
}

#[derive(Debug, Clone, IdentifierParser)]
pub enum UndocumentedRegister {
	IXH,
	IXL,
	IYH,
	IYL,
}

#[derive(Debug, Clone)]
pub enum Operand {
	Constant(i32),
	Address(u16),
	Port(u8),
	ByteRegister(ByteRegister),
	WordRegister(WordRegister),
	PortRegister(ByteRegister),
	AddressRegister(WordRegister),
	AddressRegisterWithOffset(WordRegister, i8),

	UndocumentedRegister(UndocumentedRegister),
	I,
	R,
	F,
}

#[derive(Debug, Clone, IdentifierParser)]
pub enum Condition {
	Z,
	NZ,
	C,
	NC,
	PO,
	PE,
	P,
	M,
}

#[derive(Debug, Clone, InstructionLister, InstructionParser)]
pub enum Instruction {
	#[help("Transfer data from op1 to op0")]
	LD(Operand, Operand),
	#[help("Push op0 onto the stack")]
	PUSH(Operand),
	#[help("Pop op0 from the stack")]
	POP(Operand),
	EX(Operand, Operand),
	EXX,
	LDI,
	LDIR,
	LDD,
	LDDR,
	CPI,
	CPIR,
	CPD,
	CPDR,

	#[help("Add data from op1 to op0")]
	ADD(Operand, Operand),
	#[help("Add data with the carry from op1 to op0")]
	ADC(Operand, Operand),
	#[help("Substract data from op1 to op0")]
	SUB(Operand),
	#[help("Substract data with the carry from op1 to op0")]
	SBC(Operand, Operand),
	#[help("Apply the AND gate between register A and op0")]
	AND(Operand),
	#[help("Apply the OR gate between register A and op0")]
	OR(Operand),
	#[help("Apply the XOR gate between register A and op0")]
	XOR(Operand),
	#[help("Compare register A and op0")]
	CP(Operand),

	#[help("Increment op0")]
	INC(Operand),
	#[help("Decrement op0")]
	DEC(Operand),

	DAA,
	CPL,
	NEG,
	#[help("Clear the carry flag")]
	CCF,
	#[help("Set the carry flag")]
	SCF,
	#[help("Do nothing")]
	NOP,
	#[help("Halt the processor")]
	HALT,
	#[help("Disable interrupts")]
	DI,
	#[help("Enable interrupts")]
	EI,

	IM(u8),

	RLCA,
	RLA,
	RRCA,
	RRA,
	RLC(Operand),
	RL(Operand),
	RRC(Operand),
	RR(Operand),
	SLA(Operand),
	SLL(Operand),
	SRA(Operand),
	SRL(Operand),
	RLD,
	RRD,

	#[help("Test the bit c of op0")]
	BIT(u8, Operand),
	#[help("Set the bit c of op0")]
	SET(u8, Operand),
	#[help("Clear the bit c of op0")]
	RES(u8, Operand),

	JP(Option<Condition>, Operand),
	JR(Option<Condition>, Operand),

	DJNZ(i8),

	CALL(Option<Condition>, Operand),
	RET(Option<Condition>),
	RETI,
	RETN,
	RST(u8),

	IN(Operand, Operand),
	INI,
	INIR,
	IND,
	INDR,
	OUT(Operand, Operand),
	OUTI,
	OTIR,
	OUTD,
	OTDR,

	/* This is Assembler specific */
	Binary(Vec<u8>),
}
