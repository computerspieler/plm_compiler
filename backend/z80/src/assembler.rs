use crate::instruction::*;
use std::{collections::VecDeque, ops::Add};

pub struct Assembler<InputType: Iterator<Item = Instruction>> {
	input: InputType,
	enable_undocumented_instructions: bool,
	queue: VecDeque<u8>
}

impl<InputType: Iterator<Item = Instruction>>
Assembler<InputType> {
	pub fn new(input: InputType, enable_undocumented_instructions: bool) -> Self {
		Self {
			input: input,
			enable_undocumented_instructions: enable_undocumented_instructions,
			queue: VecDeque::with_capacity(4)
		}
	}

	fn get_r_value(&self, r: ByteRegister) -> u8 {
		use ByteRegister::*;
		match r {
			A => 7,
			B => 0,
			C => 1,
			D => 2,
			E => 3,
			H => 4,
			L => 5
		}
	}

	fn get_ss_value(&self, ss: WordRegister) -> u8 {
		use WordRegister::*;
		match ss {
			BC => 0,
			DE => 1,
			HL => 2,
			SP => 3,

			_ => panic!("Unsupported case")
		}
	}

	fn get_qq_value(&self, ss: WordRegister) -> u8 {
		use WordRegister::*;
		match ss {
			BC => 0,
			DE => 1,
			HL => 2,
			AF => 3,

			_ => panic!("Unsupported case")
		}
	}

	fn get_pp_value(&self, ss: WordRegister) -> u8 {
		use WordRegister::*;
		match ss {
			BC => 0,
			DE => 1,
			IX => 2,
			SP => 3,

			_ => panic!("Unsupported case")
		}
	}

	fn get_rr_value(&self, ss: WordRegister) -> u8 {
		use WordRegister::*;
		match ss {
			BC => 0,
			DE => 1,
			IY => 2,
			SP => 3,

			_ => panic!("Unsupported case")
		}
	}

	fn get_dd_value(&self, ss: WordRegister) -> u8 {
		use WordRegister::*;
		match ss {
			BC => 0,
			DE => 1,
			HL => 2,
			SP => 3,

			_ => panic!("Unsupported case")
		}
	}

	fn get_cc_value(&self, cc: Condition) -> u8 {
		use Condition::*;
		match cc {
			NZ => 0,
			Z  => 1,
			NC => 2,
			C  => 3,
			PO => 4,
			PE => 5,
			P  => 6,
			M  => 7
		}
	}

	fn convert_instruction(&mut self, inst: Instruction) -> bool
	{
	use Instruction::*;
	use crate::instruction::WordRegister::*;
	use crate::instruction::ByteRegister::*;
	use Operand::*;
	macro_rules! b {
		[$e:expr] => {{
			self.queue.push_back(($e) as u8);
			return true;
		}};
		[$e:expr,$($next:expr),*] => {{
			self.queue.push_back(($e) as u8);
			b![$($next),*];
		}};
	}
	macro_rules! ub {
		[$($next:expr),*] => {{
			if !self.enable_undocumented_instructions {
				println!("Error, Undocumented instruction: {:?}", inst);
				return false;
			} else {
				b![$($next),*];
			}
		}};
	}
	match inst {
		LD(ByteRegister(r), ByteRegister(r_))
			=> b![0x40 | self.get_r_value(r) << 3 | self.get_r_value(r_)],
		LD(ByteRegister(r), Constant(n))
			=> b![0x06 | self.get_r_value(r) << 3, n & 0xFF],
		LD(ByteRegister(r), AddressRegister(HL))
			=> b![0x46 | self.get_r_value(r) << 3],

		LD(ByteRegister(r), AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0x46 | self.get_r_value(r) << 3, d as u8],
		LD(ByteRegister(r), AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0x46 | self.get_r_value(r) << 3, d as u8],
		LD(AddressRegister(HL), ByteRegister(r))
			=> b![0x70 | self.get_r_value(r)],
		LD(AddressRegisterWithOffset(IX, d), ByteRegister(r))
			=> b![0xDD, 0x70 | self.get_r_value(r), d as u8],
		LD(AddressRegisterWithOffset(IY, d), ByteRegister(r))
			=> b![0xFD, 0x70 | self.get_r_value(r), d as u8],
		LD(AddressRegister(HL), Constant(n))
			=> b![0x36, n as u8],
		LD(AddressRegisterWithOffset(IX, d), Constant(n))
			=> b![0xDD, 0x36, d as u8, n as u8],
		LD(AddressRegisterWithOffset(IY, d), Constant(n))
			=> b![0xFD, 0x36, d as u8, n as u8],

		LD(ByteRegister(A), AddressRegister(BC)) => b![0x0A],
		LD(ByteRegister(A), AddressRegister(DE)) => b![0x1A],
		LD(ByteRegister(A), Address(nn))    => b![0x3A, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(AddressRegister(BC), ByteRegister(A)) => b![0x02],
		LD(AddressRegister(DE), ByteRegister(A)) => b![0x12],
		LD(Address(nn),    ByteRegister(A)) => b![0x32, (nn >> 8) & 0xFF, nn & 0xFF],

		LD(ByteRegister(A), I) => b![0xED, 0x57],
		LD(ByteRegister(A), R) => b![0xED, 0x5F],
		LD(I, ByteRegister(A)) => b![0xED, 0x47],
		LD(R, ByteRegister(A)) => b![0xED, 0x4F],

		LD(WordRegister(IX), Constant(nn))
			=> b![0xDD, 0x21, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(WordRegister(IY), Constant(nn))
			=> b![0xFD, 0x21, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(WordRegister(dd), Constant(nn))
			=> b![0x01 | self.get_dd_value(dd) << 4, (nn >> 8) & 0xFF, nn & 0xFF],

		LD(WordRegister(IX), Address(nn))
			=> b![0xDD, 0x2A, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(WordRegister(IY), Address(nn))
			=> b![0xFD, 0x2A, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(WordRegister(HL), Address(nn))
			=> b![0x2A, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(WordRegister(dd), Address(nn))
			=> b![0xED, 0x4B | self.get_dd_value(dd) << 4, (nn >> 8) & 0xFF, nn & 0xFF],
		
		LD(Address(nn), WordRegister(IX))
			=> b![0xDD, 0x22, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(Address(nn), WordRegister(IY))
			=> b![0xFD, 0x22, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(Address(nn), WordRegister(HL))
			=> b![0x22, (nn >> 8) & 0xFF, nn & 0xFF],
		LD(Address(nn), WordRegister(dd))
			=> b![0xED, 0x43 | self.get_dd_value(dd) << 4, (nn >> 8) & 0xFF, nn & 0xFF],
		
		LD(WordRegister(SP), WordRegister(HL)) => b![0xF9],
		LD(WordRegister(SP), WordRegister(IX)) => b![0xDD, 0xF9],
		LD(WordRegister(SP), WordRegister(IY)) => b![0xFD, 0xF9],

		PUSH(WordRegister(IX)) => b![0xDD, 0xE5],
		PUSH(WordRegister(IY)) => b![0xFD, 0xE5],
		PUSH(WordRegister(qq)) => b![0xC5 | self.get_qq_value(qq) << 4],

		POP(WordRegister(IX)) => b![0xDD, 0xE1],
		POP(WordRegister(IY)) => b![0xFD, 0xE1],
		POP(WordRegister(qq)) => b![0xC1 | self.get_qq_value(qq) << 4],

		EX(WordRegister(DE),    WordRegister(HL))  => b![0xEB],
		EX(WordRegister(AF),    WordRegister(AF_)) => b![0x08],
		EX(AddressRegister(SP), WordRegister(HL))  => b![0xE3],
		EX(AddressRegister(SP), WordRegister(IX))  => b![0xDD, 0xE3],
		EX(AddressRegister(SP), WordRegister(IY))  => b![0xFD, 0xE3],

		EXX 				=> b![0xD9],
		LDI 				=> b![0xED, 0xA0],
		LDIR 				=> b![0xED, 0xB0],
		LDD 				=> b![0xED, 0xA8],
		LDDR 				=> b![0xED, 0xB8],
		CPI 				=> b![0xED, 0xA1],
		CPIR 				=> b![0xED, 0xB1],
		CPD 				=> b![0xED, 0xA9],
		CPDR 				=> b![0xED, 0xB9],
		
		ADD(ByteRegister(A), ByteRegister(r))
			=> b![0x80 | self.get_r_value(r)],
		ADD(ByteRegister(A), Constant(n))
			=> b![0xC6, n as u8],
		ADD(ByteRegister(A), AddressRegister(HL))
			=> b![0x86],
		ADD(ByteRegister(A), AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0x86, d as u8],
		ADD(ByteRegister(A), AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0x86, d as u8],
		
		ADC(ByteRegister(A), ByteRegister(r))
			=> b![0x84 | self.get_r_value(r)],
		ADC(ByteRegister(A), Constant(n))
			=> b![0xCE, n as u8],
		ADC(ByteRegister(A), AddressRegister(HL))
			=> b![0x8E],
		ADC(ByteRegister(A), AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0x8E, d as u8],
		ADC(ByteRegister(A), AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0x8E, d as u8],

		SBC(ByteRegister(A), ByteRegister(r))
			=> b![0x94 | self.get_r_value(r)],
		SBC(ByteRegister(A), Constant(n))
			=> b![0xDE, n as u8],
		SBC(ByteRegister(A), AddressRegister(HL))
			=> b![0x9E],
		SBC(ByteRegister(A), AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0x9E, d as u8],
		SBC(ByteRegister(A), AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0x9E, d as u8],

		ADD(WordRegister(HL), WordRegister(ss))
			=> b![0x09 | self.get_ss_value(ss) << 4],
		ADD(WordRegister(IX), WordRegister(pp))
			=> b![0xDD, 0x49 | self.get_pp_value(pp) << 4],
		ADD(WordRegister(IY), WordRegister(rr))
			=> b![0xFD, 0x09 | self.get_rr_value(rr) << 4],
		
		ADC(WordRegister(HL), WordRegister(ss))
			=> b![0xED, 0x4A | self.get_ss_value(ss) << 4],

		SBC(WordRegister(HL), WordRegister(ss))
			=> b![0xED, 0x42 | self.get_ss_value(ss) << 4],

		SUB(ByteRegister(r))
			=> b![0x90 | self.get_r_value(r)],
		SUB(Constant(n))
			=> b![0xD6, n as u8],
		SUB(AddressRegister(HL))
			=> b![0x96],
		SUB(AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0x96, d as u8],
		SUB(AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0x96, d as u8],

		AND(ByteRegister(r))
			=> b![0xA0 | self.get_r_value(r)],
		AND(Constant(n))
			=> b![0xE6, n as u8],
		AND(AddressRegister(HL))
			=> b![0xA6],
		AND(AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0xA6, d as u8],
		AND(AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0xA6, d as u8],

		XOR(ByteRegister(r))
			=> b![0xA8 | self.get_r_value(r)],
		XOR(Constant(n))
			=> b![0xEE, n as u8],
		XOR(AddressRegister(HL))
			=> b![0xAE],
		XOR(AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0xAE, d as u8],
		XOR(AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0xAE, d as u8],

		OR(ByteRegister(r))
			=> b![0xB0 | self.get_r_value(r)],
		OR(Constant(n))
			=> b![0xF6, n as u8],
		OR(AddressRegister(HL))
			=> b![0xB6],
		OR(AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0xB6, d as u8],
		OR(AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0xB6, d as u8],

		CP(ByteRegister(r))
			=> b![0xB8 | self.get_r_value(r)],
		CP(Constant(n))
			=> b![0xFE, n as u8],
		CP(AddressRegister(HL))
			=> b![0xBE],
		CP(AddressRegisterWithOffset(IX, d))
			=> b![0xDD, 0xBE, d as u8],
		CP(AddressRegisterWithOffset(IY, d))
			=> b![0xFD, 0xBE, d as u8],

		INC(ByteRegister(r))						=> b![0x04 | self.get_r_value(r) << 3],
		INC(AddressRegister(HL)) 					=> b![0x34],
		INC(AddressRegisterWithOffset(IX, d))		=> b![0xDD, 0x34, d as u8],
		INC(AddressRegisterWithOffset(IY, d))		=> b![0xFD, 0x34, d as u8],
		INC(WordRegister(IX))						=> b![0xDD, 0x23],
		INC(WordRegister(IY))						=> b![0xFD, 0x23],
		INC(WordRegister(ss)) 						=> b![0x03 | self.get_ss_value(ss) << 4],

		DEC(ByteRegister(r))					=> b![0x05 | self.get_r_value(r) << 3],
		DEC(AddressRegister(HL)) 				=> b![0x35],
		DEC(AddressRegisterWithOffset(IX, d))	=> b![0xDD, 0x35, d as u8],
		DEC(AddressRegisterWithOffset(IY, d))	=> b![0xFD, 0x35, d as u8],
		DEC(WordRegister(IX))					=> b![0xDD, 0x2B],
		DEC(WordRegister(IY))					=> b![0xFD, 0x2B],
		DEC(WordRegister(ss)) 					=> b![0x0B | self.get_ss_value(ss) << 4],

		DAA		=> b![0x27],
		CPL		=> b![0x2F],
		NEG		=> b![0xED, 0x44],
		CCF		=> b![0x3F],
		SCF		=> b![0x37],
		NOP		=> b![0x00],
		HALT	=> b![0x76],
		DI		=> b![0xF3],
		EI		=> b![0xFB],

		IM(0)	=> b![0xED, 0x46],
		IM(1)	=> b![0xED, 0x56],
		IM(2)	=> b![0xED, 0x5E],
		
		RLC(ByteRegister(r))		  => b![0xCB, 0x00 | self.get_r_value(r)],
		RLC(AddressRegister(HL))				  => b![0xCB, 0x06],
		RLC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x06],
		RLC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x06],

		RL(ByteRegister(r))		 => b![0xCB, 0x10 | self.get_r_value(r)],
		RL(AddressRegister(HL))				  	 => b![0xCB, 0x16],
		RL(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x16],
		RL(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x16],

		RRC(ByteRegister(r))		  => b![0xCB, 0x08 | self.get_r_value(r)],
		RRC(AddressRegister(HL))				  => b![0xCB, 0x0E],
		RRC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x0E],
		RRC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x0E],

		RR(ByteRegister(r))		 => b![0xCB, 0x18 | self.get_r_value(r)],
		RR(AddressRegister(HL))				 	 => b![0xCB, 0x1E],
		RR(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x1E],
		RR(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x1E],

		SLA(ByteRegister(r))		  => b![0xCB, 0x20 | self.get_r_value(r)],
		SLA(AddressRegister(HL))				  => b![0xCB, 0x26],
		SLA(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x26],
		SLA(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x26],

		SRA(ByteRegister(r))		  => b![0xCB, 0x28 | self.get_r_value(r)],
		SRA(AddressRegister(HL))				  => b![0xCB, 0x2E],
		SRA(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x2E],
		SRA(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x2E],

		SRL(ByteRegister(r))		  => b![0xCB, 0x38 | self.get_r_value(r)],
		SRL(AddressRegister(HL))				  => b![0xCB, 0x3E],
		SRL(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x3E],
		SRL(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x3E],

		RLCA 	=> b![0x07],
		RLA 	=> b![0x17],
		RRCA 	=> b![0x0F],
		RRA 	=> b![0x1F],
		RLD 	=> b![0xED, 0x6F],
		RRD 	=> b![0xED, 0x67],

		BIT(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0x40 | b << 3 | self.get_r_value(r)],
		RES(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0x80 | b << 3 | self.get_r_value(r)],
		SET(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0xC0 | b << 3 | self.get_r_value(r)],
		
		JP(None, Address(nn))
			=> b![0xC3, (nn >> 8) & 0xFF, nn & 0xFF],
		JP(None, AddressRegister(HL))
			=> b![0xE9],
		JP(None, AddressRegister(IX))
			=> b![0xDD, 0xE9],
		JP(None, AddressRegister(IY))
			=> b![0xFD, 0xE9],
		JP(Some(cc), Address(nn))
			=> b![0xC2 | self.get_cc_value(cc) << 3, (nn >> 8) & 0xFF, nn & 0xFF],
		
		JR(None, Address(nn)) => b![0x18, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::C), Address(nn))
			=> b![0x38, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::NC), Address(nn))
			=> b![0x30, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::Z), Address(nn))
			=> b![0x28, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::NZ), Address(nn))
			=> b![0x20, (nn >> 8) & 0xFF, nn & 0xFF],

		DJNZ(offset)
			=> b![0x10, offset],
		
		CALL(None, Address(nn))
			=> b![0xCD, (nn >> 8) & 0xFF, nn & 0xFF],
		CALL(Some(cc), Address(nn))
			=> b![0xC8 | self.get_cc_value(cc) << 3, (nn >> 8) & 0xFF, nn & 0xFF],

		RET(None) 					=> b![0xC9],
		RET(Some(cc))	=> b![0xC0 | (self.get_cc_value(cc) << 3)],
		RETI 						=> b![0xED, 0x4D],
		RETN 						=> b![0xED, 0x45],
		
		RST(n)	if n % 8 == 0 && n <= 0x38
			=> b![0xC7 | (n / 8) << 3],
		
		IN(ByteRegister(A), Port(n))
			=> b![0xDB, n],
		IN(ByteRegister(r), PortRegister(C))
			=> b![0xED, 0x40 | self.get_r_value(r) << 3],
	
		INI 				=> b![0xED, 0xA2],
		INIR 				=> b![0xED, 0xB2],
		IND 				=> b![0xED, 0xAA],
		INDR 				=> b![0xED, 0xBA],

		OUT(Port(n), ByteRegister(A))
			=> b![0xD3, n],
		OUT(PortRegister(C), ByteRegister(r))
			=> b![0xED, 0x41 | self.get_r_value(r) << 3],
		OUTI 				=> b![0xED, 0xA3],
		OTIR 				=> b![0xED, 0xB3],
		OUTD 				=> b![0xED, 0xAB],
		OTDR 				=> b![0xED, 0xBB],

		_ => {
			println!("Error, Invalid instruction: {:?}", inst);
			return false;
		}
	}
	}
}

impl<InputType: Iterator<Item = Instruction>>
Iterator for Assembler<InputType> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		loop {
			if self.queue.len() > 0 {
				return self.queue.pop_front();
			}
	
			match self.input.next() {
			None => { return None; }
			Some(inst) => {
				if !self.convert_instruction(inst) {
					return None;
				}
			}
			}
		}
	}
}
