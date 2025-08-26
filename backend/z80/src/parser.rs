use nom::{branch::alt, bytes::complete::{tag_no_case, take_until}, character::{complete::{char, multispace0}, digit1}, combinator::{map, opt}, multi::fold_many0, sequence::{delimited, terminated}, IResult, Parser};

use crate::instruction::{ByteRegister, Condition, Instruction, Operand, UndocumentedRegister, WordRegister};

fn parse_condition(text: &str) -> IResult<&str, Condition> {
    alt((
        map(tag_no_case("Z"),   |_| Condition::Z),
        map(tag_no_case("NZ"),  |_| Condition::NZ),
        map(tag_no_case("C"),   |_| Condition::C),
        map(tag_no_case("NC"),  |_| Condition::NC),
        map(tag_no_case("PO"),  |_| Condition::PO),
        map(tag_no_case("PE"),  |_| Condition::PE),
        map(tag_no_case("P"),   |_| Condition::P),
        map(tag_no_case("M"),   |_| Condition::M),
    )).parse(text)
}

fn parse_byte_register(text: &str) -> IResult<&str, ByteRegister> {
    alt((
        map(tag_no_case("A"), |_| ByteRegister::A),
        map(tag_no_case("B"), |_| ByteRegister::B),
        map(tag_no_case("C"), |_| ByteRegister::C),
        map(tag_no_case("D"), |_| ByteRegister::D),
        map(tag_no_case("E"), |_| ByteRegister::E),
        map(tag_no_case("H"), |_| ByteRegister::H),
        map(tag_no_case("L"), |_| ByteRegister::L),
    )).parse(text)
}

fn parse_word_register(text: &str) -> IResult<&str, WordRegister> {
    alt((
        map(tag_no_case("AF"), |_| WordRegister::AF),
        map(tag_no_case("BC"), |_| WordRegister::BC),
        map(tag_no_case("DE"), |_| WordRegister::DE),
        map(tag_no_case("HL"), |_| WordRegister::HL),
        map(tag_no_case("AF'"), |_| WordRegister::AF_),
        map(tag_no_case("BC'"), |_| WordRegister::BC_),
        map(tag_no_case("DE'"), |_| WordRegister::DE_),
        map(tag_no_case("HL'"), |_| WordRegister::HL_),
        map(tag_no_case("IX"), |_| WordRegister::IX),
        map(tag_no_case("IY"), |_| WordRegister::IY),
        map(tag_no_case("SP"), |_| WordRegister::SP),
    )).parse(text)
}

fn parse_undocumented_register(text: &str) -> IResult<&str, UndocumentedRegister> {
    alt((
        map(tag_no_case("IXH"), |_| UndocumentedRegister::IXH),
        map(tag_no_case("IXL"), |_| UndocumentedRegister::IXL),
        map(tag_no_case("IYH"), |_| UndocumentedRegister::IYH),
        map(tag_no_case("IYL"), |_| UndocumentedRegister::IYL),
    )).parse(text)
}

fn parse_operand(text: &str) -> IResult<&str, Operand> {
    alt((
        // Constant(i32),
        // Address(u16),
        // Port(u8),
        // AddressRegisterWithOffset(WordRegister, i8),
        map(
            delimited(char('('), parse_byte_register, char(')')),
            |x| Operand::PortRegister(x)
        ),
        map(
            delimited(char('('), parse_word_register, char(')')),
            |x| Operand::AddressRegister(x)
        ),
        map(parse_undocumented_register, |x| Operand::UndocumentedRegister(x)),
        map(parse_byte_register, |x| Operand::ByteRegister(x)),
        map(parse_word_register, |x| Operand::WordRegister(x)),
        map(tag_no_case("i"), |_| Operand::I),
        map(tag_no_case("r"), |_| Operand::R),
        map(tag_no_case("f"), |_| Operand::F),
    )).parse(text)
}

fn parse_instruction(text: &str) -> IResult<&str, Instruction> {
    alt((
        alt((
            map(
                (tag_no_case("LD"), parse_operand, char(','), parse_operand),
                | x | Instruction::LD(x.1, x.3)
            ),
            map(
                (tag_no_case("PUSH"), parse_operand),
                | x | Instruction::PUSH(x.1)
            ),
            map(
                (tag_no_case("POP"), parse_operand),
                | x | Instruction::POP(x.1)
            ),
            map(
                (tag_no_case("EX"), parse_operand, char(','), parse_operand),
                | x | Instruction::EX(x.1, x.3)
            ),
            map(
                tag_no_case("EXX"),
                | _ | Instruction::EXX
            ),
            map(
                tag_no_case("LDI"),
                | _ | Instruction::LDI
            ),
            map(
                tag_no_case("LDIR"),
                | _ | Instruction::LDIR
            ),
            map(
                tag_no_case("LDD"),
                | _ | Instruction::LDD
            ),
            map(
                tag_no_case("LDDR"),
                | _ | Instruction::LDDR
            ),
            map(
                tag_no_case("CPI"),
                | _ | Instruction::CPI
            ),
            map(
                tag_no_case("CPIR"),
                | _ | Instruction::CPIR
            ),
            map(
                tag_no_case("CPD"),
                | _ | Instruction::CPD
            ),
            map(
                tag_no_case("CPDR"),
                | _ | Instruction::CPDR
            ),
            map(
                (tag_no_case("ADD"), parse_operand, char(','), parse_operand),
                | x | Instruction::ADD(x.1, x.3)
            ),
            map(
                (tag_no_case("ADC"), parse_operand, char(','), parse_operand),
                | x | Instruction::ADC(x.1, x.3)
            ),
            map(
                (tag_no_case("SUB"), parse_operand),
                | x | Instruction::SUB(x.1)
            ),
            map(
                (tag_no_case("SBC"), parse_operand, char(','), parse_operand),
                | x | Instruction::SBC(x.1, x.3)
            ),
            map(
                (tag_no_case("AND"), parse_operand),
                | x | Instruction::AND(x.1)
            ),
            map(
                (tag_no_case("OR"), parse_operand),
                | x | Instruction::OR(x.1)
            ),
            map(
                (tag_no_case("XOR"), parse_operand),
                | x | Instruction::XOR(x.1)
            ),
            map(
                (tag_no_case("CP"), parse_operand),
                | x | Instruction::CP(x.1)
            ),
        )),
        alt((
            map(
                (tag_no_case("INC"), parse_operand),
                | x | Instruction::INC(x.1)
            ),
            map(
                (tag_no_case("DEC"), parse_operand),
                | x | Instruction::DEC(x.1)
            ),
            map(
                tag_no_case("DAA"),
                | _ | Instruction::DAA
            ),
            map(
                tag_no_case("CPL"),
                | _ | Instruction::CPL
            ),
            map(
                tag_no_case("NEG"),
                | _ | Instruction::NEG
            ),
            map(
                tag_no_case("CCF"),
                | _ | Instruction::CCF
            ),
            map(
                tag_no_case("SCF"),
                | _ | Instruction::SCF
            ),
            map(
                tag_no_case("NOP"),
                | _ | Instruction::NOP
            ),
            map(
                tag_no_case("HALT"),
                | _ | Instruction::HALT
            ),
            map(
                tag_no_case("DI"),
                | _ | Instruction::DI
            ),
            map(
                tag_no_case("EI"),
                | _ | Instruction::EI
            ),
            map(
                (tag_no_case("IM"), digit1()),
                | x: (&str, &str) | Instruction::IM(x.1.parse().unwrap())
            ),
            map(
                tag_no_case("RLCA"),
                | _ | Instruction::RLCA
            ),
            map(
                tag_no_case("RLA"),
                | _ | Instruction::RLA
            ),
            map(
                tag_no_case("RRCA"),
                | _ | Instruction::RRCA
            ),
            map(
                tag_no_case("RRA"),
                | _ | Instruction::RRA
            ),
            map(
                (tag_no_case("RLC"), parse_operand),
                | x | Instruction::RLC(x.1)
            ),
            map(
                (tag_no_case("RL"), parse_operand),
                | x | Instruction::RL(x.1)
            ),
            map(
                (tag_no_case("RRC"), parse_operand),
                | x | Instruction::RRC(x.1)
            ),
            map(
                (tag_no_case("RR"), parse_operand),
                | x | Instruction::RR(x.1)
            ),
            map(
                (tag_no_case("SLA"), parse_operand),
                | x | Instruction::SLA(x.1)
            ),
        )),
        alt((
            map(
                (tag_no_case("SLL"), parse_operand),
                | x | Instruction::SLL(x.1)
            ),
            map(
                (tag_no_case("SRA"), parse_operand),
                | x | Instruction::SRA(x.1)
            ),
            map(
                (tag_no_case("SRL"), parse_operand),
                | x | Instruction::SRL(x.1)
            ),
            map(
                tag_no_case("RLD"),
                | _ | Instruction::RLD
            ),
            map(
                tag_no_case("RRD"),
                | _ | Instruction::RRD
            ),
            map(
                (tag_no_case("BIT"), digit1(), char(','), parse_operand),
                | x | Instruction::BIT(x.1.parse().unwrap(), x.3)
            ),
            map(
                (tag_no_case("SET"), digit1(), char(','), parse_operand),
                | x | Instruction::SET(x.1.parse().unwrap(), x.3)
            ),
            map(
                (tag_no_case("RES"), digit1(), char(','), parse_operand),
                | x | Instruction::RES(x.1.parse().unwrap(), x.3)
            ),
            map(
                (tag_no_case("JP"), opt(terminated(parse_condition, char(','))), parse_operand),
                | x | Instruction::JP(x.1, x.2)
            ),
            map(
                (tag_no_case("JR"), opt(terminated(parse_condition, char(','))), parse_operand),
                | x | Instruction::JR(x.1, x.2)
            ),
            /*
            map(
                tag_no_case("DJNZ"), //(i8),
                | x | Instruction::DJNZ
            ),
            */
            map(
                (tag_no_case("CALL"), opt(terminated(parse_condition, char(','))), parse_operand),
                | x | Instruction::CALL(x.1, x.2)
            ),
            map(
                (tag_no_case("RET"), opt(parse_condition)),
                | x | Instruction::RET(x.1)
            ),
            map(
                tag_no_case("RETI"),
                | _ | Instruction::RETI
            ),
            map(
                tag_no_case("RETN"),
                | _ | Instruction::RETN
            ),
            map(
                (tag_no_case("RST"), digit1()),
                | x: (&str, &str) | Instruction::RST(x.1.parse::<u8>().unwrap())
            ),
            map(
                (tag_no_case("IN"), parse_operand, char(','), parse_operand),
                | x | Instruction::IN(x.1, x.3)
            ),
            map(
                tag_no_case("INI"),
                | _ | Instruction::INI
            ),
            map(
                tag_no_case("INIR"),
                | _ | Instruction::INIR
            ),
            map(
                tag_no_case("IND"),
                | _ | Instruction::IND
            ),
        )),
        alt((
            map(
                tag_no_case("INDR"),
                | _ | Instruction::INDR
            ),
            map(
                (tag_no_case("OUT"), parse_operand, char(','), parse_operand),
                | x | Instruction::OUT(x.1, x.3)
            ),
            map(
                tag_no_case("OUTI"),
                | _ | Instruction::OUTI
            ),
            map(
                tag_no_case("OTIR"),
                | _ | Instruction::OTIR
            ),
            map(
                tag_no_case("OUTD"),
                | _ | Instruction::OUTD
            ),
            map(
                tag_no_case("OTDR"),
                | _ | Instruction::OTDR
            )
        ))
    )).parse(text)
}

fn parse_line(text: &str) -> IResult<&str, Instruction> {
    terminated(
        parse_instruction,
        (multispace0, opt((char(';'), take_until("\n"))), opt(char('\n')))
    ).parse(text)
}

pub fn parse(text: &str) -> IResult<&str, Vec<Instruction>> {
    fold_many0(
        parse_line,
        Vec::<Instruction>::new,
        |mut acc, item| {
            acc.push(item);
            acc
        }
    ).parse(text)
}
