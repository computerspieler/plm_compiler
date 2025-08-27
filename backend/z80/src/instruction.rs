#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteRegister {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordRegister {
	AF,
	BC,
	DE,
	HL,
	AF_,
	BC_,
	DE_,
	HL_,
	IX,
	IY,
	SP,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndocumentedRegister {
	IXH,
	IXL,
	IYH,
	IYL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand<ToU8, Address, Constant, Offset> {
	Constant(Constant),
	Address(Address),
	Port(ToU8),
	ByteRegister(ByteRegister),
	WordRegister(WordRegister),
	PortRegister(ByteRegister),
	AddressRegister(WordRegister),
	AddressRegisterWithOffset(WordRegister, Offset),

	UndocumentedRegister(UndocumentedRegister),
	I,
	R,
	F,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction<ToU8, Address, Constant, Offset> {
	LD(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	PUSH(Operand<ToU8, Address, Constant, Offset>),
	POP(Operand<ToU8, Address, Constant, Offset>),
	EX(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	EXX,
	LDI,
	LDIR,
	LDD,
	LDDR,
	CPI,
	CPIR,
	CPD,
	CPDR,

	ADD(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	ADC(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	SUB(Operand<ToU8, Address, Constant, Offset>),
	SBC(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	AND(Operand<ToU8, Address, Constant, Offset>),
	OR(Operand<ToU8, Address, Constant, Offset>),
	XOR(Operand<ToU8, Address, Constant, Offset>),
	CP(Operand<ToU8, Address, Constant, Offset>),

	INC(Operand<ToU8, Address, Constant, Offset>),
	DEC(Operand<ToU8, Address, Constant, Offset>),

	DAA,
	CPL,
	NEG,
	CCF,
	SCF,
	NOP,
	HALT,
	DI,
	EI,

	IM(ToU8),

	RLCA,
	RLA,
	RRCA,
	RRA,
	RLC(Operand<ToU8, Address, Constant, Offset>),
	RL(Operand<ToU8, Address, Constant, Offset>),
	RRC(Operand<ToU8, Address, Constant, Offset>),
	RR(Operand<ToU8, Address, Constant, Offset>),
	SLA(Operand<ToU8, Address, Constant, Offset>),
	SLL(Operand<ToU8, Address, Constant, Offset>),
	SRA(Operand<ToU8, Address, Constant, Offset>),
	SRL(Operand<ToU8, Address, Constant, Offset>),
	RLD,
	RRD,

	BIT(ToU8, Operand<ToU8, Address, Constant, Offset>),
	SET(ToU8, Operand<ToU8, Address, Constant, Offset>),
	RES(ToU8, Operand<ToU8, Address, Constant, Offset>),

	JP(Option<Condition>, Operand<ToU8, Address, Constant, Offset>),
	JR(Option<Condition>, Operand<ToU8, Address, Constant, Offset>),

	DJNZ(Offset),

	CALL(Option<Condition>, Operand<ToU8, Address, Constant, Offset>),
	RET(Option<Condition>),
	RETI,
	RETN,
	RST(ToU8),

	IN(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	INI,
	INIR,
	IND,
	INDR,
	OUT(Operand<ToU8, Address, Constant, Offset>, Operand<ToU8, Address, Constant, Offset>),
	OUTI,
	OTIR,
	OUTD,
	OTDR,

	/* This is Assembler specific */
	Binary(Vec<ToU8>),
}
