use assembler_common::InstructionLister;

#[derive(Debug, Clone)]
pub enum ByteRegister {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

#[derive(Debug, Clone)]
pub enum WordRegister {
	AF,
	BC,
	DE,
	HL,
	// Since we can't have prime, i'll use underscores
	AF_,
	BC_,
	DE_,
	HL_,
	IX,
	IY,
	SP,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, InstructionLister)]
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

	BIT(u8, Operand),
	#[help("Set the bit c of op0")]
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
