use z80::assembler::*;
use z80::instruction::Instruction::*;
use z80::instruction::WordRegister::*;
use z80::instruction::ByteRegister::*;
use z80::instruction::Operand::*;

macro_rules! test {
    ($test_name:ident, [$( $insts:expr ),+], [$( $values:literal ),+]) => {
        #[test]
        fn $test_name() {
            let i = vec![$($insts),+];
            let mut a = Assembler::new(i.into_iter(), false);
            let o: Vec<u8> = a.collect();
            let e: Vec<u8> = vec![$($values),+];
            assert!(o == e, "Invalid output for {:?}: got {:?}, expected {:?}", [$($insts),+], o, e);
        }
    }
}

test!(t0, [NOP], [0x00]);
test!(t1, [LD(ByteRegister(A), ByteRegister(A))], [0x7F]);
/* TODO: More tests */
