use crate::instruction::*;
use std::collections::VecDeque;

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
		LD(ByteRegister(r), ByteRegister(r_)) =>
			b![0x40 | self.get_r_value(r) << 3 | self.get_r_value(r_)],
		LD(ByteRegister(r), Constant(n)) =>
			b![0x06 | self.get_r_value(r) << 3, n & 0xFF],
		LD(ByteRegister(r), AddressRegister(HL)) =>
			b![0x46 | self.get_r_value(r) << 3],
		LD(ByteRegister(A), I) => b![0xED, 0x57],
		LD(ByteRegister(A), R) => b![0xED, 0x5F],
		LD(I, ByteRegister(A)) => b![0xED, 0x47],
		LD(R, ByteRegister(A)) => b![0xED, 0x4F],
			
/*
		LD(Operand, Operand),
		PUSH(Operand),
		POP(Operand),
		EX(Operand, Operand),
*/
		EXX 				=> b![0xD9],
		LDI 				=> b![0xED, 0xA0],
		LDIR 				=> b![0xED, 0xB0],
		LDD 				=> b![0xED, 0xA8],
		LDDR 				=> b![0xED, 0xB8],
		CPI 				=> b![0xED, 0xA1],
		CPIR 				=> b![0xED, 0xB1],
		CPD 				=> b![0xED, 0xA9],
		CPDR 				=> b![0xED, 0xB9],
		
		/*
		ADD(Operand, Operand),
		ADC(Operand, Operand),
		SUB(Operand),
		SBC(Operand, Operand),
		AND(Operand),
		OR(Operand),
		XOR(Operand),
		CP(Operand),
		*/

		INC(ByteRegister(r))			=> b![0x04 | self.get_r_value(r) << 3],
		INC(AddressRegister(HL)) 					=> b![0x34],
		INC(AddressRegisterWithOffset(IX, d))	=> b![0xDD, 0x34, d as u8],
		INC(AddressRegisterWithOffset(IY, d))	=> b![0xFD, 0x34, d as u8],
		INC(WordRegister(IX))						=> b![0xDD, 0x23],
		INC(WordRegister(IY))						=> b![0xFD, 0x23],
		INC(WordRegister(ss)) 		=> b![0x03 | self.get_ss_value(ss) << 4],

		DEC(ByteRegister(r))			=> b![0x05 | self.get_r_value(r) << 3],
		DEC(AddressRegister(HL)) 					=> b![0x35],
		DEC(AddressRegisterWithOffset(IX, d))	=> b![0xDD, 0x35, d as u8],
		DEC(AddressRegisterWithOffset(IY, d))	=> b![0xFD, 0x35, d as u8],
		DEC(WordRegister(IX))						=> b![0xDD, 0x2B],
		DEC(WordRegister(IY))						=> b![0xFD, 0x2B],
		DEC(WordRegister(ss)) 		=> b![0x0B | self.get_ss_value(ss) << 4],

		DAA 				=> b![0x27],
		CPL 				=> b![0x2F],
		NEG 				=> b![0xED, 0x44],
		CCF 				=> b![0x3F],
		SCF 				=> b![0x37],
		NOP 				=> b![0x00],
		HALT 				=> b![0x76],
		DI 					=> b![0xF3],
		EI 					=> b![0xFB],

		IM(0)				=> b![0xED, 0x46],
		IM(1)				=> b![0xED, 0x56],
		IM(2)				=> b![0xED, 0x5E],
		
		RLCA 				=> b![0x07],
		RLA 				=> b![0x17],
		RRCA 				=> b![0x0F],
		RRA 				=> b![0x1F],
		/*
		RLC(Operand),
		RL(Operand),
		RRC(Operand),
		RR(Operand),
		SLA(Operand),
		SRA(Operand),
		SRL(Operand),
		*/
		RLD 				=> b![0xED, 0x6F],
		RRD 				=> b![0xED, 0x67],

		BIT(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0x40 | b << 3 | self.get_r_value(r)],
		RES(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0x80 | b << 3 | self.get_r_value(r)],
		SET(b, ByteRegister(r)) if b < 8
			=> b![0xCB, 0xC0 | b << 3 | self.get_r_value(r)],
		
		
		JP(None, Address(nn)) => b![0xC3, (nn >> 8) & 0xFF, nn & 0xFF],
		JP(None, AddressRegister(HL)) => b![0xE9],
		JP(None, AddressRegister(IX)) => b![0xDD, 0xE9],
		JP(None, AddressRegister(IY)) => b![0xFD, 0xE9],
		JP(Some(cc), Address(nn)) =>
			b![0xC2 | self.get_cc_value(cc) << 3, (nn >> 8) & 0xFF, nn & 0xFF],
		
		JR(None, Address(nn)) => b![0x18, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::C), Address(nn))
			=> b![0x38, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::NC), Address(nn))
			=> b![0x30, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::Z), Address(nn))
			=> b![0x28, (nn >> 8) & 0xFF, nn & 0xFF],
		JR(Some(Condition::NZ), Address(nn))
			=> b![0x20, (nn >> 8) & 0xFF, nn & 0xFF],

		DJNZ(offset)	=> b![0x10, offset],
		
		CALL(None, Address(nn)) => b![0xCD, (nn >> 8) & 0xFF, nn & 0xFF],
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
