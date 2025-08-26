use z80::{instruction::*, parser};

macro_rules! test {
    ($test_name:ident, $code:expr, [$($expected_output: expr),+]) => {
        #[test]
        fn $test_name() {
            match parser::parse($code) {
            Err(e) => {
                eprintln!("Received error: {}", e);
                assert!(false);
            }
            Ok((leftovers, output)) => {
                println!("Worked, leftovers: {}", leftovers);
                let expected = [$($expected_output),+];
                let l = expected.len();
                assert_eq!(expected.len(), output.len());
                for i in 0 .. l {
                    assert_eq!(expected[i], output[i]);
                }
            }
            }
        }
    };
}

test!(test_single_instruction,
    "LDa,B",
    [
        Instruction::LD(
            Operand::ByteRegister(ByteRegister::A),
            Operand::ByteRegister(ByteRegister::B)
        )
    ]
);

test!(test_multi_instruction,
    "LDa,B\nOUT(c),d",
    [
        Instruction::LD(
            Operand::ByteRegister(ByteRegister::A),
            Operand::ByteRegister(ByteRegister::B)
        ),
        Instruction::OUT(
            Operand::PortRegister(ByteRegister::C),
            Operand::ByteRegister(ByteRegister::D)
        )
    ]
);

test!(test_comment,
    "LDa,B;gfsgdfsfds\nOUT(c),d",
    [
        Instruction::LD(
            Operand::ByteRegister(ByteRegister::A),
            Operand::ByteRegister(ByteRegister::B)
        ),
        Instruction::OUT(
            Operand::PortRegister(ByteRegister::C),
            Operand::ByteRegister(ByteRegister::D)
        )
    ]
);

test!(test_comment_with_spaces,
    "LDa,B  ;gfsgdfsfd   s  \nOUT(c),d",
    [
        Instruction::LD(
            Operand::ByteRegister(ByteRegister::A),
            Operand::ByteRegister(ByteRegister::B)
        ),
        Instruction::OUT(
            Operand::PortRegister(ByteRegister::C),
            Operand::ByteRegister(ByteRegister::D)
        )
    ]
);