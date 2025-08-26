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
pub enum Instruction {
	LD(Operand, Operand),
	PUSH(Operand),
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

	ADD(Operand, Operand),
	ADC(Operand, Operand),
	SUB(Operand),
	SBC(Operand, Operand),
	AND(Operand),
	OR(Operand),
	XOR(Operand),
	CP(Operand),

	INC(Operand),
	DEC(Operand),

	DAA,
	CPL,
	NEG,
	CCF,
	SCF,
	NOP,
	HALT,
	DI,
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

	BIT(u8, Operand),
	SET(u8, Operand),
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
