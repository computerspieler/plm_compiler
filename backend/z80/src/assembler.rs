use crate::instruction::*;
use std::collections::VecDeque;


fn get_r_value(r: ByteRegister) -> u8 {
	use ByteRegister::*;
	match r {
		| A => 7,
		| B => 0,
		| C => 1,
		| D => 2,
		| E => 3,
		| H => 4,
		| L => 5,
	}
}

fn get_ss_value(ss: WordRegister) -> u8 {
	use WordRegister::*;
	match ss {
		| BC => 0,
		| DE => 1,
		| HL => 2,
		| SP => 3,

		| _ => panic!("Unsupported case"),
	}
}

fn get_qq_value(ss: WordRegister) -> u8 {
	use WordRegister::*;
	match ss {
		| BC => 0,
		| DE => 1,
		| HL => 2,
		| AF => 3,

		| _ => panic!("Unsupported case"),
	}
}

fn get_pp_value(ss: WordRegister) -> u8 {
	use WordRegister::*;
	match ss {
		| BC => 0,
		| DE => 1,
		| IX => 2,
		| SP => 3,

		| _ => panic!("Unsupported case"),
	}
}

fn get_rr_value(ss: WordRegister) -> u8 {
	use WordRegister::*;
	match ss {
		| BC => 0,
		| DE => 1,
		| IY => 2,
		| SP => 3,

		| _ => panic!("Unsupported case"),
	}
}

fn get_dd_value(ss: WordRegister) -> u8 {
	use WordRegister::*;
	match ss {
		| BC => 0,
		| DE => 1,
		| HL => 2,
		| SP => 3,

		| _ => panic!("Unsupported case"),
	}
}

fn get_cc_value(cc: Condition) -> u8 {
	use Condition::*;
	match cc {
		| NZ => 0,
		| Z => 1,
		| NC => 2,
		| C => 3,
		| PO => 4,
		| PE => 5,
		| P => 6,
		| M => 7,
	}
}

fn get_both_part_word_pair(pair: WordRegister) -> Option<(Operand<u8, u16, i32, i8>, Operand<u8, u16, i32, i8>)> {
	match pair {
	WordRegister::AF => None,
	WordRegister::BC => Some((
		Operand::ByteRegister(ByteRegister::C),
		Operand::ByteRegister(ByteRegister::B)
	)),
	WordRegister::DE => Some((
		Operand::ByteRegister(ByteRegister::E),
		Operand::ByteRegister(ByteRegister::D)
	)),
	WordRegister::HL => Some((
		Operand::ByteRegister(ByteRegister::L),
		Operand::ByteRegister(ByteRegister::H)
	)),
	WordRegister::IX => Some((
		Operand::UndocumentedRegister(UndocumentedRegister::IXL),
		Operand::UndocumentedRegister(UndocumentedRegister::IXH)
	)),
	WordRegister::IY => Some((
		Operand::UndocumentedRegister(UndocumentedRegister::IYL),
		Operand::UndocumentedRegister(UndocumentedRegister::IYH)
	)),

	WordRegister::SP => None,
	WordRegister::AF_ => None,
	WordRegister::BC_ => None,
	WordRegister::DE_ => None,
	WordRegister::HL_ => None
	}
}


pub struct Assembler<InputType: Iterator<Item = Instruction<u8, u16, i32, i8>>> {
	input: InputType,
	enable_macro_instructions: bool,
	enable_undocumented_instructions: bool,
	has_error_occured: bool,
	queue: VecDeque<u8>,
}

impl<InputType: Iterator<Item = Instruction<u8, u16, i32, i8>>> Assembler<InputType> {
	pub fn new(input: InputType,
		enable_macro_instructions: bool,
		enable_undocumented_instructions: bool
	) -> Self {
		Self {
			input: input,
			enable_macro_instructions: enable_macro_instructions,
			enable_undocumented_instructions: enable_undocumented_instructions,
			has_error_occured: false,
			queue: VecDeque::with_capacity(4),
		}
	}

	pub fn has_error_occured(&self) -> bool {
		return self.has_error_occured;
	}

	fn convert_instruction(&mut self, inst: Instruction<u8, u16, i32, i8>) -> bool {
		use crate::instruction::ByteRegister::*;
		use crate::instruction::WordRegister::*;
		use Instruction::*;
		use Operand::*;

		macro_rules! parse {
			($($inst: expr),+) => {{
				$(
					if !self.convert_instruction($inst) {
						return false
					}
				)*
				return true;
			}};
		}

		if !self.enable_macro_instructions {
			return self.convert_real_instruction(inst);
		}

		match inst {
			| LD(arg, AddressRegister(IX)) => parse!(
				LD(arg, AddressRegisterWithOffset(IX, 0))
			),
			| LD(arg, AddressRegister(IY)) => parse!(
				LD(arg, AddressRegisterWithOffset(IY, 0))
			),
			| LD(AddressRegister(IX), arg) => parse!(
				LD(AddressRegisterWithOffset(IX, 0), arg)
			),
			| LD(AddressRegister(IY), arg) => parse!(
				LD(AddressRegisterWithOffset(IY, 0), arg)
			),

			// Load instruction between memory and word registers
			| LD(AddressRegisterWithOffset(r, offset), WordRegister(r2)) if offset<127 => {
				let parts_r2 = get_both_part_word_pair(r2);
				if parts_r2.is_none() {
					return false;
				}
				let (lower_r2, higher_r2) = parts_r2.unwrap();
				parse!(
					LD(AddressRegisterWithOffset(r.clone(), offset), lower_r2),
					LD(AddressRegisterWithOffset(r, offset+1), higher_r2)
				)
			},
			| LD(WordRegister(r), AddressRegisterWithOffset(r2, offset)) if offset<127 => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					LD(lower_r, AddressRegisterWithOffset(r2.clone(), offset)),
					LD(higher_r, AddressRegisterWithOffset(r2, offset+1))
				)
			},
			// We need to handle this special case to avoid an integer overflow
			| LD(AddressRegisterWithOffset(r, offset), WordRegister(r2)) if offset>=127 => {
				let parts_r2 = get_both_part_word_pair(r2);
				if parts_r2.is_none() {
					return false;
				}
				let (lower_r2, higher_r2) = parts_r2.unwrap();
				parse!(
					LD(AddressRegisterWithOffset(r.clone(), offset), lower_r2),
					INC(WordRegister(r.clone())),
					LD(AddressRegisterWithOffset(r.clone(), offset), higher_r2),
					DEC(WordRegister(r))
				)
			},
			| LD(WordRegister(r), AddressRegisterWithOffset(r2, offset)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					LD(lower_r, AddressRegisterWithOffset(r2.clone(), offset)),
					INC(WordRegister(r2.clone())),
					LD(higher_r, AddressRegisterWithOffset(r2.clone(), offset)),
					DEC(WordRegister(r2))
				)
			},

			| LD(AddressRegister(r), WordRegister(r2)) => {
				let parts_r2 = get_both_part_word_pair(r2);
				if parts_r2.is_none() {
					return false;
				}
				let (lower_r2, higher_r2) = parts_r2.unwrap();
				parse!(
					LD(AddressRegister(r.clone()), lower_r2),
					INC(WordRegister(r.clone())),
					LD(AddressRegister(r.clone()), higher_r2),
					DEC(WordRegister(r))
				)
			},
			| LD(WordRegister(r), AddressRegister(r2)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					LD(lower_r, AddressRegister(r2.clone())),
					INC(WordRegister(r2.clone())),
					LD(higher_r, AddressRegister(r2.clone())),
					DEC(WordRegister(r2))
				)
			},

			// If r is SP, then there is a real instruction for that
			| LD(WordRegister(r), WordRegister(r2)) if r != SP => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				let parts_r2 = get_both_part_word_pair(r2);
				if parts_r2.is_none() {
					return false;
				}
				let (lower_r2, higher_r2) = parts_r2.unwrap();
				parse!(
					LD(higher_r, higher_r2),
					LD(lower_r, lower_r2)
				)
			},

			| ADD(ByteRegister(A), AddressRegister(IX)) => parse!(
				ADD(ByteRegister(A), AddressRegisterWithOffset(IX, 0))
			),
			| ADD(ByteRegister(A), AddressRegister(IY)) => parse!(
				ADD(ByteRegister(A), AddressRegisterWithOffset(IY, 0))
			),
			| ADC(ByteRegister(A), AddressRegister(IX)) => parse!(
				ADC(ByteRegister(A), AddressRegisterWithOffset(IX, 0))
			),
			| ADC(ByteRegister(A), AddressRegister(IY)) => parse!(
				ADC(ByteRegister(A), AddressRegisterWithOffset(IY, 0))
			),
			| SBC(ByteRegister(A), AddressRegister(IX)) => parse!(
				SBC(ByteRegister(A), AddressRegisterWithOffset(IX, 0))
			),
			| SBC(ByteRegister(A), AddressRegister(IY)) => parse!(
				SBC(ByteRegister(A), AddressRegisterWithOffset(IY, 0))
			),
			| SUB(AddressRegister(IX)) => parse!(SUB(AddressRegisterWithOffset(IX, 0))),
			| SUB(AddressRegister(IY)) => parse!(SUB(AddressRegisterWithOffset(IY, 0))),

			| AND(AddressRegister(IX)) => parse!(AND(AddressRegisterWithOffset(IX, 0))),
			| AND(AddressRegister(IY)) => parse!(AND(AddressRegisterWithOffset(IY, 0))),
			| XOR(AddressRegister(IX)) => parse!(XOR(AddressRegisterWithOffset(IX, 0))),
			| XOR(AddressRegister(IY)) => parse!(XOR(AddressRegisterWithOffset(IY, 0))),
			| OR(AddressRegister(IX)) => parse!(OR(AddressRegisterWithOffset(IX, 0))),
			| OR(AddressRegister(IY)) => parse!(OR(AddressRegisterWithOffset(IY, 0))),
			| CP(AddressRegister(IX)) => parse!(CP(AddressRegisterWithOffset(IX, 0))),
			| CP(AddressRegister(IY)) => parse!(CP(AddressRegisterWithOffset(IY, 0))),
			| INC(AddressRegister(IX)) => parse!(INC(AddressRegisterWithOffset(IX, 0))),
			| INC(AddressRegister(IY)) => parse!(INC(AddressRegisterWithOffset(IY, 0))),
			| DEC(AddressRegister(IX)) => parse!(DEC(AddressRegisterWithOffset(IX, 0))),
			| DEC(AddressRegister(IY)) => parse!(DEC(AddressRegisterWithOffset(IY, 0))),
			| RLC(AddressRegister(IX)) => parse!(RLC(AddressRegisterWithOffset(IX, 0))),
			| RLC(AddressRegister(IY)) => parse!(RLC(AddressRegisterWithOffset(IY, 0))),
			| RL(AddressRegister(IX)) => parse!(RL(AddressRegisterWithOffset(IX, 0))),
			| RL(AddressRegister(IY)) => parse!(RL(AddressRegisterWithOffset(IY, 0))),
			| SRA(AddressRegister(IX)) => parse!(SRA(AddressRegisterWithOffset(IX, 0))),
			| SRA(AddressRegister(IY)) => parse!(SRA(AddressRegisterWithOffset(IY, 0))),
			| RRC(AddressRegister(IX)) => parse!(RRC(AddressRegisterWithOffset(IX, 0))),
			| RRC(AddressRegister(IY)) => parse!(RRC(AddressRegisterWithOffset(IY, 0))),
			| RR(AddressRegister(IX)) => parse!(RR(AddressRegisterWithOffset(IX, 0))),
			| RR(AddressRegister(IY)) => parse!(RR(AddressRegisterWithOffset(IY, 0))),
			| SLA(AddressRegister(IX)) => parse!(SLA(AddressRegisterWithOffset(IX, 0))),
			| SLA(AddressRegister(IY)) => parse!(SLA(AddressRegisterWithOffset(IY, 0))),
			| SLL(AddressRegister(IX)) => parse!(SLL(AddressRegisterWithOffset(IX, 0))),
			| SLL(AddressRegister(IY)) => parse!(SLL(AddressRegisterWithOffset(IY, 0))),
			| SRL(AddressRegister(IX)) => parse!(SRL(AddressRegisterWithOffset(IX, 0))),
			| SRL(AddressRegister(IY)) => parse!(SRL(AddressRegisterWithOffset(IY, 0))),

			| BIT(b, AddressRegister(IX)) => parse!(BIT(b, AddressRegisterWithOffset(IX, 0))),
			| BIT(b, AddressRegister(IY)) => parse!(BIT(b, AddressRegisterWithOffset(IY, 0))),
			| RES(b, AddressRegister(IX)) => parse!(RES(b, AddressRegisterWithOffset(IX, 0))),
			| RES(b, AddressRegister(IY)) => parse!(RES(b, AddressRegisterWithOffset(IY, 0))),
			| SET(b, AddressRegister(IX)) => parse!(SET(b, AddressRegisterWithOffset(IX, 0))),
			| SET(b, AddressRegister(IY)) => parse!(SET(b, AddressRegisterWithOffset(IY, 0))),

			| RL(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					RL(lower_r),
					RL(higher_r)
				)
			},
			// SLA shifts the content of a register one bit to the left
			// which is equivalent to adding a register to itself
			| SLA(WordRegister(HL)) => parse!(ADD(WordRegister(HL), WordRegister(HL))),
			| SLA(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					SLA(lower_r),
					RL(higher_r)
				)
			},
			| SLL(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					SLL(lower_r),
					RL(higher_r)
				)
			},

			| RR(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					RR(higher_r),
					RR(lower_r)
				)
			},
			| SRA(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					SRA(higher_r),
					RR(lower_r)
				)
			},
			| SRL(WordRegister(r)) => {
				let parts_r = get_both_part_word_pair(r);
				if parts_r.is_none() {
					return false;
				}
				let (lower_r, higher_r) = parts_r.unwrap();

				parse!(
					SRL(higher_r),
					RR(lower_r)
				)
			},

			| _ => self.convert_real_instruction(inst),
		}
	}

	fn convert_undocumented_instruction(&mut self, inst: Instruction<u8, u16, i32, i8>) -> bool {
		use crate::instruction::ByteRegister;
		use crate::instruction::UndocumentedRegister::*;
		use crate::instruction::WordRegister::*;
		use Instruction::*;
		use Operand::*;

		macro_rules! b {
			[$($e:expr),*] => {{
				$(self.queue.push_back(($e) as u8);)*
				return true;
			}};
		}

		macro_rules! ixx_instruction {
			($r: ident| $($before:expr),* ;$eh:literal,$el:literal; $($next:expr),*) => {
				match $r {
				IXH => b![0xDD, $($before,)* $eh $(,$next)*],
				IXL => b![0xDD, $($before,)* $el $(,$next)*],
				IYH => b![0xFD, $($before,)* $eh $(,$next)*],
				IYL => b![0xFD, $($before,)* $el $(,$next)*],
				}
			}
		}

		match inst {
			| SLL(ByteRegister(r)) => b![0xCB, 0x30 | get_r_value(r)],
			| SLL(AddressRegister(HL)) => b![0xCB, 0x36],
			| SLL(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d as u8, 0x36],
			| SLL(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d as u8, 0x36],
			| OUT(PortRegister(ByteRegister::C), Constant(0)) => b![0xED, 0x71],
			| IN(F, PortRegister(ByteRegister::C)) => b![0xED, 0x70],

			/* IXx/IYx instructions */
			| ADD(ByteRegister(ByteRegister::A), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x84,0x85;)
			}
			| ADC(ByteRegister(ByteRegister::A), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x8C,0x8D;)
			}
			| INC(UndocumentedRegister(r)) => ixx_instruction!(r| ;0x24,0x2C;),
			| SUB(UndocumentedRegister(r)) => ixx_instruction!(r| ;0x94,0x95;),
			| SBC(ByteRegister(ByteRegister::A), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x9C,0x9D;)
			}
			| DEC(UndocumentedRegister(r)) => ixx_instruction!(r| ;0x25,0x2D;),

			| AND(UndocumentedRegister(r)) => ixx_instruction!(r| ;0xA4,0xA5;),
			| OR(UndocumentedRegister(r)) => ixx_instruction!(r| ;0xB4,0xB5;),
			| XOR(UndocumentedRegister(r)) => ixx_instruction!(r| ;0xAC,0xAD;),
			| CP(UndocumentedRegister(r)) => ixx_instruction!(r| ;0xBC,0xBD;),

			| LD(UndocumentedRegister(r), Constant(n)) if n >= -128 && n < 256 => {
				ixx_instruction!(r| ;0x26,0x2E; n)
			}
			| LD(ByteRegister(ByteRegister::A), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x7C,0x7D; )
			}
			| LD(ByteRegister(ByteRegister::B), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x44,0x45; )
			}
			| LD(ByteRegister(ByteRegister::C), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x4C,0x4D; )
			}
			| LD(ByteRegister(ByteRegister::D), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x54,0x55; )
			}
			| LD(ByteRegister(ByteRegister::E), UndocumentedRegister(r)) => {
				ixx_instruction!(r| ;0x5C,0x5D; )
			}

			| LD(UndocumentedRegister(r), ByteRegister(ByteRegister::A)) => {
				ixx_instruction!(r| ;0x67,0x6F; )
			}
			| LD(UndocumentedRegister(r), ByteRegister(ByteRegister::B)) => {
				ixx_instruction!(r| ;0x60,0x68; )
			}
			| LD(UndocumentedRegister(r), ByteRegister(ByteRegister::C)) => {
				ixx_instruction!(r| ;0x61,0x69; )
			}
			| LD(UndocumentedRegister(r), ByteRegister(ByteRegister::D)) => {
				ixx_instruction!(r| ;0x62,0x6A; )
			}
			| LD(UndocumentedRegister(r), ByteRegister(ByteRegister::E)) => {
				ixx_instruction!(r| ;0x63,0x6B; )
			}

			| LD(UndocumentedRegister(IXH), UndocumentedRegister(IXH)) => b![0xDD, 0x64],
			| LD(UndocumentedRegister(IXH), UndocumentedRegister(IXL)) => b![0xDD, 0x65],
			| LD(UndocumentedRegister(IXL), UndocumentedRegister(IXH)) => b![0xDD, 0x6C],
			| LD(UndocumentedRegister(IXL), UndocumentedRegister(IXL)) => b![0xDD, 0x6D],

			| LD(UndocumentedRegister(IYH), UndocumentedRegister(IYH)) => b![0xFD, 0x64],
			| LD(UndocumentedRegister(IYH), UndocumentedRegister(IYL)) => b![0xFD, 0x65],
			| LD(UndocumentedRegister(IYL), UndocumentedRegister(IYH)) => b![0xFD, 0x6C],
			| LD(UndocumentedRegister(IYL), UndocumentedRegister(IYL)) => b![0xFD, 0x6D],

			| _ => {
				println!("Error, Invalid instruction: {:?}", inst);
				return false;
			}
		}
	}

	fn convert_real_instruction(&mut self, inst: Instruction<u8, u16, i32, i8>) -> bool {
		use crate::instruction::ByteRegister::*;
		use crate::instruction::WordRegister::*;
		use Instruction::*;
		use Operand::*;

		macro_rules! b {
			[$($e:expr),*] => {{
				$(self.queue.push_back(($e) as u8);)*
				return true;
			}};
		}

		match inst {
			| Binary(data) => {
				self.queue.extend(data);
				return true;
			}

			| LD(ByteRegister(r), ByteRegister(r_)) => {
				b![0x40 | get_r_value(r) << 3 | get_r_value(r_)]
			}
			| LD(ByteRegister(r), Constant(n)) if n >= -128 && n < 256 => {
				b![0x06 | get_r_value(r) << 3, n & 0xFF]
			}
			| LD(ByteRegister(r), AddressRegister(HL)) => b![0x46 | get_r_value(r) << 3],

			| LD(ByteRegister(r), AddressRegisterWithOffset(IX, d)) => {
				b![0xDD, 0x46 | get_r_value(r) << 3, d as u8]
			}
			| LD(ByteRegister(r), AddressRegisterWithOffset(IY, d)) => {
				b![0xFD, 0x46 | get_r_value(r) << 3, d as u8]
			}
			| LD(AddressRegister(HL), ByteRegister(r)) => b![0x70 | get_r_value(r)],
			| LD(AddressRegisterWithOffset(IX, d), ByteRegister(r)) => {
				b![0xDD, 0x70 | get_r_value(r), d as u8]
			}
			| LD(AddressRegisterWithOffset(IY, d), ByteRegister(r)) => {
				b![0xFD, 0x70 | get_r_value(r), d as u8]
			}
			| LD(AddressRegister(HL), Constant(n)) if n >= -128 && n < 256 => b![0x36, n as u8],
			| LD(AddressRegisterWithOffset(IX, d), Constant(n)) if n >= -128 && n < 256 => {
				b![0xDD, 0x36, d as u8, n as u8]
			}
			| LD(AddressRegisterWithOffset(IY, d), Constant(n)) if n >= -128 && n < 256 => {
				b![0xFD, 0x36, d as u8, n as u8]
			}

			| LD(ByteRegister(A), AddressRegister(BC)) => b![0x0A],
			| LD(ByteRegister(A), AddressRegister(DE)) => b![0x1A],
			| LD(ByteRegister(A), Address(nn)) => b![0x3A, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(AddressRegister(BC), ByteRegister(A)) => b![0x02],
			| LD(AddressRegister(DE), ByteRegister(A)) => b![0x12],
			| LD(Address(nn), ByteRegister(A)) => b![0x32, nn & 0xFF, (nn >> 8) & 0xFF],

			| LD(ByteRegister(A), I) => b![0xED, 0x57],
			| LD(ByteRegister(A), R) => b![0xED, 0x5F],
			| LD(I, ByteRegister(A)) => b![0xED, 0x47],
			| LD(R, ByteRegister(A)) => b![0xED, 0x4F],

			| LD(WordRegister(IX), Constant(nn)) if nn >= 0 && nn < 0x10000 => {
				b![0xDD, 0x21, nn & 0xFF, (nn >> 8) & 0xFF]
			}
			| LD(WordRegister(IY), Constant(nn)) if nn >= 0 && nn < 0x10000 => {
				b![0xFD, 0x21, nn & 0xFF, (nn >> 8) & 0xFF]
			}
			| LD(WordRegister(dd), Constant(nn)) if nn >= 0 && nn < 0x10000 => b![
				0x01 | get_dd_value(dd) << 4,
				nn & 0xFF,
				(nn >> 8) & 0xFF
			],

			| LD(WordRegister(IX), Address(nn)) => b![0xDD, 0x2A, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(WordRegister(IY), Address(nn)) => b![0xFD, 0x2A, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(WordRegister(HL), Address(nn)) => b![0x2A, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(WordRegister(dd), Address(nn)) => b![
				0xED,
				0x4B | get_dd_value(dd) << 4,
				nn & 0xFF,
				(nn >> 8) & 0xFF
			],

			| LD(Address(nn), WordRegister(IX)) => b![0xDD, 0x22, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(Address(nn), WordRegister(IY)) => b![0xFD, 0x22, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(Address(nn), WordRegister(HL)) => b![0x22, nn & 0xFF, (nn >> 8) & 0xFF],
			| LD(Address(nn), WordRegister(dd)) => b![
				0xED,
				0x43 | get_dd_value(dd) << 4,
				nn & 0xFF,
				(nn >> 8) & 0xFF
			],

			| LD(WordRegister(SP), WordRegister(HL)) => b![0xF9],
			| LD(WordRegister(SP), WordRegister(IX)) => b![0xDD, 0xF9],
			| LD(WordRegister(SP), WordRegister(IY)) => b![0xFD, 0xF9],

			| PUSH(WordRegister(IX)) => b![0xDD, 0xE5],
			| PUSH(WordRegister(IY)) => b![0xFD, 0xE5],
			| PUSH(WordRegister(qq)) => b![0xC5 | get_qq_value(qq) << 4],

			| POP(WordRegister(IX)) => b![0xDD, 0xE1],
			| POP(WordRegister(IY)) => b![0xFD, 0xE1],
			| POP(WordRegister(qq)) => b![0xC1 | get_qq_value(qq) << 4],

			| EX(WordRegister(DE), WordRegister(HL)) => b![0xEB],
			| EX(WordRegister(AF), WordRegister(AF_)) => b![0x08],
			| EX(AddressRegister(SP), WordRegister(HL)) => b![0xE3],
			| EX(AddressRegister(SP), WordRegister(IX)) => b![0xDD, 0xE3],
			| EX(AddressRegister(SP), WordRegister(IY)) => b![0xFD, 0xE3],

			| EXX => b![0xD9],
			| LDI => b![0xED, 0xA0],
			| LDIR => b![0xED, 0xB0],
			| LDD => b![0xED, 0xA8],
			| LDDR => b![0xED, 0xB8],
			| CPI => b![0xED, 0xA1],
			| CPIR => b![0xED, 0xB1],
			| CPD => b![0xED, 0xA9],
			| CPDR => b![0xED, 0xB9],

			| ADD(ByteRegister(A), ByteRegister(r)) => b![0x80 | get_r_value(r)],
			| ADD(ByteRegister(A), Constant(n)) if n >= -128 && n < 256 => b![0xC6, n],
			| ADD(ByteRegister(A), AddressRegister(HL)) => b![0x86],
			| ADD(ByteRegister(A), AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x86, d],
			| ADD(ByteRegister(A), AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x86, d],

			| ADC(ByteRegister(A), ByteRegister(r)) => b![0x88 | get_r_value(r)],
			| ADC(ByteRegister(A), Constant(n)) if n >= -128 && n < 256 => b![0xCE, n],
			| ADC(ByteRegister(A), AddressRegister(HL)) => b![0x8E],
			| ADC(ByteRegister(A), AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x8E, d],
			| ADC(ByteRegister(A), AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x8E, d],

			| SBC(ByteRegister(A), ByteRegister(r)) => b![0x98 | get_r_value(r)],
			| SBC(ByteRegister(A), Constant(n)) if n >= -128 && n < 256 => b![0xDE, n],
			| SBC(ByteRegister(A), AddressRegister(HL)) => b![0x9E],

			| SBC(ByteRegister(A), AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x9E, d],
			| SBC(ByteRegister(A), AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x9E, d],

			| ADD(WordRegister(HL), WordRegister(ss)) => b![0x09 | get_ss_value(ss) << 4],
			| ADD(WordRegister(IX), WordRegister(pp)) => {
				b![0xDD, 0x09 | get_pp_value(pp) << 4]
			}
			| ADD(WordRegister(IY), WordRegister(rr)) => {
				b![0xFD, 0x09 | get_rr_value(rr) << 4]
			}

			| ADC(WordRegister(HL), WordRegister(ss)) => {
				b![0xED, 0x4A | get_ss_value(ss) << 4]
			}

			| SBC(WordRegister(HL), WordRegister(ss)) => {
				b![0xED, 0x42 | get_ss_value(ss) << 4]
			}

			| SUB(ByteRegister(r)) => b![0x90 | get_r_value(r)],
			| SUB(Constant(n)) if n >= -128 && n < 256 => b![0xD6, n],
			| SUB(AddressRegister(HL)) => b![0x96],
			| SUB(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x96, d],
			| SUB(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x96, d],

			| AND(ByteRegister(r)) => b![0xA0 | get_r_value(r)],
			| AND(Constant(n)) if n >= -128 && n < 256 => b![0xE6, n],
			| AND(AddressRegister(HL)) => b![0xA6],
			| AND(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xA6, d],
			| AND(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xA6, d],

			| XOR(ByteRegister(r)) => b![0xA8 | get_r_value(r)],
			| XOR(Constant(n)) if n >= -128 && n < 256 => b![0xEE, n],
			| XOR(AddressRegister(HL)) => b![0xAE],
			| XOR(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xAE, d],
			| XOR(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xAE, d],

			| OR(ByteRegister(r)) => b![0xB0 | get_r_value(r)],
			| OR(Constant(n)) if n >= -128 && n < 256 => b![0xF6, n],
			| OR(AddressRegister(HL)) => b![0xB6],
			| OR(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xB6, d],
			| OR(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xB6, d],

			| CP(ByteRegister(r)) => b![0xB8 | get_r_value(r)],
			| CP(Constant(n)) if n >= -128 && n < 256 => b![0xFE, n],
			| CP(AddressRegister(HL)) => b![0xBE],
			| CP(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xBE, d],
			| CP(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xBE, d],

			| INC(ByteRegister(r)) => b![0x04 | get_r_value(r) << 3],
			| INC(AddressRegister(HL)) => b![0x34],
			| INC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x34, d],
			| INC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x34, d],
			| INC(WordRegister(IX)) => b![0xDD, 0x23],
			| INC(WordRegister(IY)) => b![0xFD, 0x23],
			| INC(WordRegister(ss)) => b![0x03 | get_ss_value(ss) << 4],

			| DEC(ByteRegister(r)) => b![0x05 | get_r_value(r) << 3],
			| DEC(AddressRegister(HL)) => b![0x35],
			| DEC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0x35, d],
			| DEC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0x35, d],
			| DEC(WordRegister(IX)) => b![0xDD, 0x2B],
			| DEC(WordRegister(IY)) => b![0xFD, 0x2B],
			| DEC(WordRegister(ss)) => b![0x0B | get_ss_value(ss) << 4],

			| DAA => b![0x27],
			| CPL => b![0x2F],
			| NEG => b![0xED, 0x44],
			| CCF => b![0x3F],
			| SCF => b![0x37],
			| NOP => b![0x00],
			| HALT => b![0x76],
			| DI => b![0xF3],
			| EI => b![0xFB],

			| IM(0) => b![0xED, 0x46],
			| IM(1) => b![0xED, 0x56],
			| IM(2) => b![0xED, 0x5E],

			| RLC(ByteRegister(r)) => b![0xCB, 0x00 | get_r_value(r)],
			| RLC(AddressRegister(HL)) => b![0xCB, 0x06],
			| RLC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x06],
			| RLC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x06],

			| RL(ByteRegister(r)) => b![0xCB, 0x10 | get_r_value(r)],
			| RL(AddressRegister(HL)) => b![0xCB, 0x16],
			| RL(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x16],
			| RL(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x16],

			| RRC(ByteRegister(r)) => b![0xCB, 0x08 | get_r_value(r)],
			| RRC(AddressRegister(HL)) => b![0xCB, 0x0E],
			| RRC(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x0E],
			| RRC(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x0E],

			| RR(ByteRegister(r)) => b![0xCB, 0x18 | get_r_value(r)],
			| RR(AddressRegister(HL)) => b![0xCB, 0x1E],
			| RR(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x1E],
			| RR(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x1E],

			| SLA(ByteRegister(r)) => b![0xCB, 0x20 | get_r_value(r)],
			| SLA(AddressRegister(HL)) => b![0xCB, 0x26],
			| SLA(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x26],
			| SLA(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x26],

			| SRA(ByteRegister(r)) => b![0xCB, 0x28 | get_r_value(r)],
			| SRA(AddressRegister(HL)) => b![0xCB, 0x2E],
			| SRA(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x2E],
			| SRA(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x2E],

			| SRL(ByteRegister(r)) => b![0xCB, 0x38 | get_r_value(r)],
			| SRL(AddressRegister(HL)) => b![0xCB, 0x3E],
			| SRL(AddressRegisterWithOffset(IX, d)) => b![0xDD, 0xCB, d, 0x3E],
			| SRL(AddressRegisterWithOffset(IY, d)) => b![0xFD, 0xCB, d, 0x3E],

			| RLCA => b![0x07],
			| RLA => b![0x17],
			| RRCA => b![0x0F],
			| RRA => b![0x1F],
			| RLD => b![0xED, 0x6F],
			| RRD => b![0xED, 0x67],

			| BIT(b, ByteRegister(r)) if b < 8 => b![0xCB, 0x40 | b << 3 | get_r_value(r)],
			| BIT(b, AddressRegister(HL)) if b < 8 => b![0xCB, 0x46 | b << 3],
			| BIT(b, AddressRegisterWithOffset(IX, d)) if b < 8 => b![0xDD, 0xCB, d, 0x46 | b << 3],
			| BIT(b, AddressRegisterWithOffset(IY, d)) if b < 8 => b![0xFD, 0xCB, d, 0x46 | b << 3],

			| RES(b, ByteRegister(r)) if b < 8 => b![0xCB, 0x80 | b << 3 | get_r_value(r)],
			| RES(b, AddressRegister(HL)) if b < 8 => b![0xCB, 0x86 | b << 3],
			| RES(b, AddressRegisterWithOffset(IX, d)) if b < 8 => b![0xDD, 0xCB, d, 0x86 | b << 3],
			| RES(b, AddressRegisterWithOffset(IY, d)) if b < 8 => b![0xFD, 0xCB, d, 0x86 | b << 3],

			| SET(b, ByteRegister(r)) if b < 8 => b![0xCB, 0xC0 | b << 3 | get_r_value(r)],
			| SET(b, AddressRegister(HL)) if b < 8 => b![0xCB, 0xC6 | b << 3],
			| SET(b, AddressRegisterWithOffset(IX, d)) if b < 8 => b![0xDD, 0xCB, d, 0xC6 | b << 3],
			| SET(b, AddressRegisterWithOffset(IY, d)) if b < 8 => b![0xFD, 0xCB, d, 0xC6 | b << 3],

			| JP(None, Constant(nn)) if nn >= 0 && nn < 0x10000 => {
				b![0xC3, nn & 0xFF, (nn >> 8) & 0xFF]
			}
			| JP(None, WordRegister(HL)) => b![0xE9],
			| JP(None, WordRegister(IX)) => b![0xDD, 0xE9],
			| JP(None, WordRegister(IY)) => b![0xFD, 0xE9],
			| JP(Some(cc), Constant(nn)) if nn >= 0 && nn < 0x10000 => b![
				0xC2 | get_cc_value(cc) << 3,
				nn & 0xFF,
				(nn >> 8) & 0xFF
			],

			| JR(None, Constant(d)) if d >= -126 && d <= 129 => b![0x18, d & 0xFF],
			| JR(Some(Condition::C), Constant(d)) if d >= -126 && d <= 129 => b![0x38, d & 0xFF],
			| JR(Some(Condition::NC), Constant(d)) if d >= -126 && d <= 129 => b![0x30, d & 0xFF],
			| JR(Some(Condition::Z), Constant(d)) if d >= -126 && d <= 129 => b![0x28, d & 0xFF],
			| JR(Some(Condition::NZ), Constant(d)) if d >= -126 && d <= 129 => b![0x20, d & 0xFF],

			| DJNZ(offset) => b![0x10, offset],

			| CALL(None, Constant(nn)) if nn >= 0 && nn < 0x10000 => {
				b![0xCD, nn & 0xFF, (nn >> 8) & 0xFF]
			}
			| CALL(Some(cc), Constant(nn)) if nn >= 0 && nn < 0x10000 => b![
				0xC4 | get_cc_value(cc) << 3,
				nn & 0xFF,
				(nn >> 8) & 0xFF
			],

			| RET(None) => b![0xC9],
			| RET(Some(cc)) => b![0xC0 | (get_cc_value(cc) << 3)],
			| RETI => b![0xED, 0x4D],
			| RETN => b![0xED, 0x45],

			| RST(n) if n % 8 == 0 && n <= 0x38 => b![0xC7 | (n / 8) << 3],

			| IN(ByteRegister(A), Port(n)) => b![0xDB, n],
			| IN(ByteRegister(r), PortRegister(C)) => b![0xED, 0x40 | get_r_value(r) << 3],

			| INI => b![0xED, 0xA2],
			| INIR => b![0xED, 0xB2],
			| IND => b![0xED, 0xAA],
			| INDR => b![0xED, 0xBA],

			| OUT(Port(n), ByteRegister(A)) => b![0xD3, n],
			| OUT(PortRegister(C), ByteRegister(r)) => b![0xED, 0x41 | get_r_value(r) << 3],
			| OUTI => b![0xED, 0xA3],
			| OTIR => b![0xED, 0xB3],
			| OUTD => b![0xED, 0xAB],
			| OTDR => b![0xED, 0xBB],

			| _ if self.enable_undocumented_instructions => {
				self.convert_undocumented_instruction(inst)
			}
			| _ => {
				println!("Error, Invalid instruction: {:?}", inst);
				return false;
			}
		}
	}

	pub fn clear_and_collect_into(&mut self, collection: &mut Vec<u8>) {
		collection.clear();
		collection.extend(self);
	}
}

impl<InputType: Iterator<Item = Instruction<u8, u16, i32, i8>>> Iterator for Assembler<InputType> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		loop {
			if self.queue.len() > 0 {
				return self.queue.pop_front();
			}

			match self.input.next() {
				| None => {
					return None;
				}
				| Some(inst) => {
					if !self.convert_instruction(inst) {
						self.has_error_occured = true;
						return None;
					}
				}
			}
		}
	}
}
