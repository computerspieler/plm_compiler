#![allow(non_snake_case)] 

use z80::assembler::*;
use std::iter;

macro_rules! test {
    ($test_name:ident, [$inst:expr], [$( $values:expr ),+] $(, $var: ident: $vt: ty)*) => {
        #[test]
        fn $test_name() {
			$(
				let $var: $vt = <$vt>::MAX-1;
			)*
            let a = Assembler::new(iter::once($inst), false);
            let o: Vec<u8> = a.collect();
            let e: Vec<u8> = vec![$($values as u8),+];
			assert!(o != [], "Invalid output for {:?}: no output", $inst);
            assert!(o == e, "Invalid output for {:?}: got {:?}, expected {:?}", $inst, o, e);
        }
    };
}

macro_rules! test_ub {
    ($test_name:ident, [$inst:expr], [$( $values:expr ),+] $(, $var: ident: $vt: ty)*) => {
        #[test]
        fn $test_name() {
			$(
				let $var: $vt = <$vt>::MAX-1;
			)*

            let e: Vec<u8> = vec![$($values as u8),+];
			
            let a = Assembler::new(iter::once($inst), false);
            let o: Vec<u8> = a.collect();
            assert!(o != e, "An undefined instruction has been translated correctly");

			let a = Assembler::new(iter::once($inst), true);
			let o: Vec<u8> = a.collect();
			assert!(o == e, "Invalid output for {:?}: got {:?}, expected {:?}", $inst, o, e);
        }
    }
}

/*
 * This list has been created by Paulo Custidio
 * Source: https://metacpan.org/pod/Asm::Z80::Table
 */

use z80::instruction::*;
use z80::instruction::Instruction::*;
use z80::instruction::WordRegister::*;
use z80::instruction::ByteRegister::*;
use z80::instruction::UndocumentedRegister::*;
use z80::instruction::Operand::*;

test!(adc_a__hl__, [ADC(ByteRegister(A),AddressRegister(HL))], [0x8E]);
test!(adc_a__ix__, [ADC(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x8E, 0x00]);
test!(adc_a__ix_DIS__, [ADC(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x8E, DIS], DIS: i8);
test!(adc_a__iy__, [ADC(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x8E, 0x00]);
test!(adc_a__iy_DIS__, [ADC(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x8E, DIS], DIS: i8);
test!(adc_a_N_, [ADC(ByteRegister(A),Constant(n as i32))], [0xCE, n], n: u8);
test!(adc_a_a_, [ADC(ByteRegister(A),ByteRegister(A))], [0x8F]);
test!(adc_a_b_, [ADC(ByteRegister(A),ByteRegister(B))], [0x88]);
test!(adc_a_c_, [ADC(ByteRegister(A),ByteRegister(C))], [0x89]);
test!(adc_a_d_, [ADC(ByteRegister(A),ByteRegister(D))], [0x8A]);
test!(adc_a_e_, [ADC(ByteRegister(A),ByteRegister(E))], [0x8B]);
test!(adc_a_h_, [ADC(ByteRegister(A),ByteRegister(H))], [0x8C]);
test_ub!(adc_a_ixh_, [ADC(ByteRegister(A),UndocumentedRegister(IXH))], [0xDD, 0x8C]);
test_ub!(adc_a_ixl_, [ADC(ByteRegister(A),UndocumentedRegister(IXL))], [0xDD, 0x8D]);
test_ub!(adc_a_iyh_, [ADC(ByteRegister(A),UndocumentedRegister(IYH))], [0xFD, 0x8C]);
test_ub!(adc_a_iyl_, [ADC(ByteRegister(A),UndocumentedRegister(IYL))], [0xFD, 0x8D]);
test!(adc_a_l_, [ADC(ByteRegister(A),ByteRegister(L))], [0x8D]);
test!(adc_hl_bc_, [ADC(WordRegister(HL),WordRegister(BC))], [0xED, 0x4A]);
test!(adc_hl_de_, [ADC(WordRegister(HL),WordRegister(DE))], [0xED, 0x5A]);
test!(adc_hl_hl_, [ADC(WordRegister(HL),WordRegister(HL))], [0xED, 0x6A]);
test!(adc_hl_sp_, [ADC(WordRegister(HL),WordRegister(SP))], [0xED, 0x7A]);
test!(add_a__hl__, [ADD(ByteRegister(A),AddressRegister(HL))], [0x86]);
test!(add_a__ix__, [ADD(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x86, 0x00]);
test!(add_a__ix_DIS__, [ADD(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x86, DIS], DIS: i8);
test!(add_a__iy__, [ADD(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x86, 0x00]);
test!(add_a__iy_DIS__, [ADD(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x86, DIS], DIS: i8);
test!(add_a_N_, [ADD(ByteRegister(A),Constant(n as i32))], [0xC6, n], n: u8);
test!(add_a_a_, [ADD(ByteRegister(A),ByteRegister(A))], [0x87]);
test!(add_a_b_, [ADD(ByteRegister(A),ByteRegister(B))], [0x80]);
test!(add_a_c_, [ADD(ByteRegister(A),ByteRegister(C))], [0x81]);
test!(add_a_d_, [ADD(ByteRegister(A),ByteRegister(D))], [0x82]);
test!(add_a_e_, [ADD(ByteRegister(A),ByteRegister(E))], [0x83]);
test!(add_a_h_, [ADD(ByteRegister(A),ByteRegister(H))], [0x84]);
test_ub!(add_a_ixh_, [ADD(ByteRegister(A),UndocumentedRegister(IXH))], [0xDD, 0x84]);
test_ub!(add_a_ixl_, [ADD(ByteRegister(A),UndocumentedRegister(IXL))], [0xDD, 0x85]);
test_ub!(add_a_iyh_, [ADD(ByteRegister(A),UndocumentedRegister(IYH))], [0xFD, 0x84]);
test_ub!(add_a_iyl_, [ADD(ByteRegister(A),UndocumentedRegister(IYL))], [0xFD, 0x85]);
test!(add_a_l_, [ADD(ByteRegister(A),ByteRegister(L))], [0x85]);
test!(add_hl_bc_, [ADD(WordRegister(HL),WordRegister(BC))], [0x09]);
test!(add_hl_de_, [ADD(WordRegister(HL),WordRegister(DE))], [0x19]);
test!(add_hl_hl_, [ADD(WordRegister(HL),WordRegister(HL))], [0x29]);
test!(add_hl_sp_, [ADD(WordRegister(HL),WordRegister(SP))], [0x39]);
test!(add_ix_bc_, [ADD(WordRegister(IX),WordRegister(BC))], [0xDD, 0x09]);
test!(add_ix_de_, [ADD(WordRegister(IX),WordRegister(DE))], [0xDD, 0x19]);
test!(add_ix_ix_, [ADD(WordRegister(IX),WordRegister(IX))], [0xDD, 0x29]);
test!(add_ix_sp_, [ADD(WordRegister(IX),WordRegister(SP))], [0xDD, 0x39]);
test!(add_iy_bc_, [ADD(WordRegister(IY),WordRegister(BC))], [0xFD, 0x09]);
test!(add_iy_de_, [ADD(WordRegister(IY),WordRegister(DE))], [0xFD, 0x19]);
test!(add_iy_iy_, [ADD(WordRegister(IY),WordRegister(IY))], [0xFD, 0x29]);
test!(add_iy_sp_, [ADD(WordRegister(IY),WordRegister(SP))], [0xFD, 0x39]);
test!(and__hl__, [AND(AddressRegister(HL))], [0xA6]);
test!(and__ix__, [AND(AddressRegister(IX))], [0xDD, 0xA6, 0x00]);
test!(and__ix_DIS__, [AND(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xA6, DIS], DIS: i8);
test!(and__iy__, [AND(AddressRegister(IY))], [0xFD, 0xA6, 0x00]);
test!(and__iy_DIS__, [AND(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xA6, DIS], DIS: i8);
test!(and_N_, [AND(Constant(n as i32))], [0xE6, n], n: u8);
test!(and_a_, [AND(ByteRegister(A))], [0xA7]);
test!(and_b_, [AND(ByteRegister(B))], [0xA0]);
test!(and_c_, [AND(ByteRegister(C))], [0xA1]);
test!(and_d_, [AND(ByteRegister(D))], [0xA2]);
test!(and_e_, [AND(ByteRegister(E))], [0xA3]);
test!(and_h_, [AND(ByteRegister(H))], [0xA4]);
test_ub!(and_ixh_, [AND(UndocumentedRegister(IXH))], [0xDD, 0xA4]);
test_ub!(and_ixl_, [AND(UndocumentedRegister(IXL))], [0xDD, 0xA5]);
test_ub!(and_iyh_, [AND(UndocumentedRegister(IYH))], [0xFD, 0xA4]);
test_ub!(and_iyl_, [AND(UndocumentedRegister(IYL))], [0xFD, 0xA5]);
test!(and_l_, [AND(ByteRegister(L))], [0xA5]);
test!(bit_0__hl__, [BIT(0,AddressRegister(HL))], [0xCB, 0x46]);
test!(bit_0__ix__, [BIT(0,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x46]);
test!(bit_0__ix_DIS__, [BIT(0,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x46], DIS: i8);
test!(bit_0__iy__, [BIT(0,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x46]);
test!(bit_0__iy_DIS__, [BIT(0,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x46], DIS: i8);
test!(bit_0_a_, [BIT(0,ByteRegister(A))], [0xCB, 0x47]);
test!(bit_0_b_, [BIT(0,ByteRegister(B))], [0xCB, 0x40]);
test!(bit_0_c_, [BIT(0,ByteRegister(C))], [0xCB, 0x41]);
test!(bit_0_d_, [BIT(0,ByteRegister(D))], [0xCB, 0x42]);
test!(bit_0_e_, [BIT(0,ByteRegister(E))], [0xCB, 0x43]);
test!(bit_0_h_, [BIT(0,ByteRegister(H))], [0xCB, 0x44]);
test!(bit_0_l_, [BIT(0,ByteRegister(L))], [0xCB, 0x45]);
test!(bit_1__hl__, [BIT(1,AddressRegister(HL))], [0xCB, 0x4E]);
test!(bit_1__ix__, [BIT(1,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x4E]);
test!(bit_1__ix_DIS__, [BIT(1,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x4E], DIS: i8);
test!(bit_1__iy__, [BIT(1,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x4E]);
test!(bit_1__iy_DIS__, [BIT(1,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x4E], DIS: i8);
test!(bit_1_a_, [BIT(1,ByteRegister(A))], [0xCB, 0x4F]);
test!(bit_1_b_, [BIT(1,ByteRegister(B))], [0xCB, 0x48]);
test!(bit_1_c_, [BIT(1,ByteRegister(C))], [0xCB, 0x49]);
test!(bit_1_d_, [BIT(1,ByteRegister(D))], [0xCB, 0x4A]);
test!(bit_1_e_, [BIT(1,ByteRegister(E))], [0xCB, 0x4B]);
test!(bit_1_h_, [BIT(1,ByteRegister(H))], [0xCB, 0x4C]);
test!(bit_1_l_, [BIT(1,ByteRegister(L))], [0xCB, 0x4D]);
test!(bit_2__hl__, [BIT(2,AddressRegister(HL))], [0xCB, 0x56]);
test!(bit_2__ix__, [BIT(2,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x56]);
test!(bit_2__ix_DIS__, [BIT(2,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x56], DIS: i8);
test!(bit_2__iy__, [BIT(2,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x56]);
test!(bit_2__iy_DIS__, [BIT(2,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x56], DIS: i8);
test!(bit_2_a_, [BIT(2,ByteRegister(A))], [0xCB, 0x57]);
test!(bit_2_b_, [BIT(2,ByteRegister(B))], [0xCB, 0x50]);
test!(bit_2_c_, [BIT(2,ByteRegister(C))], [0xCB, 0x51]);
test!(bit_2_d_, [BIT(2,ByteRegister(D))], [0xCB, 0x52]);
test!(bit_2_e_, [BIT(2,ByteRegister(E))], [0xCB, 0x53]);
test!(bit_2_h_, [BIT(2,ByteRegister(H))], [0xCB, 0x54]);
test!(bit_2_l_, [BIT(2,ByteRegister(L))], [0xCB, 0x55]);
test!(bit_3__hl__, [BIT(3,AddressRegister(HL))], [0xCB, 0x5E]);
test!(bit_3__ix__, [BIT(3,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x5E]);
test!(bit_3__ix_DIS__, [BIT(3,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x5E], DIS: i8);
test!(bit_3__iy__, [BIT(3,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x5E]);
test!(bit_3__iy_DIS__, [BIT(3,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x5E], DIS: i8);
test!(bit_3_a_, [BIT(3,ByteRegister(A))], [0xCB, 0x5F]);
test!(bit_3_b_, [BIT(3,ByteRegister(B))], [0xCB, 0x58]);
test!(bit_3_c_, [BIT(3,ByteRegister(C))], [0xCB, 0x59]);
test!(bit_3_d_, [BIT(3,ByteRegister(D))], [0xCB, 0x5A]);
test!(bit_3_e_, [BIT(3,ByteRegister(E))], [0xCB, 0x5B]);
test!(bit_3_h_, [BIT(3,ByteRegister(H))], [0xCB, 0x5C]);
test!(bit_3_l_, [BIT(3,ByteRegister(L))], [0xCB, 0x5D]);
test!(bit_4__hl__, [BIT(4,AddressRegister(HL))], [0xCB, 0x66]);
test!(bit_4__ix__, [BIT(4,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x66]);
test!(bit_4__ix_DIS__, [BIT(4,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x66], DIS: i8);
test!(bit_4__iy__, [BIT(4,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x66]);
test!(bit_4__iy_DIS__, [BIT(4,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x66], DIS: i8);
test!(bit_4_a_, [BIT(4,ByteRegister(A))], [0xCB, 0x67]);
test!(bit_4_b_, [BIT(4,ByteRegister(B))], [0xCB, 0x60]);
test!(bit_4_c_, [BIT(4,ByteRegister(C))], [0xCB, 0x61]);
test!(bit_4_d_, [BIT(4,ByteRegister(D))], [0xCB, 0x62]);
test!(bit_4_e_, [BIT(4,ByteRegister(E))], [0xCB, 0x63]);
test!(bit_4_h_, [BIT(4,ByteRegister(H))], [0xCB, 0x64]);
test!(bit_4_l_, [BIT(4,ByteRegister(L))], [0xCB, 0x65]);
test!(bit_5__hl__, [BIT(5,AddressRegister(HL))], [0xCB, 0x6E]);
test!(bit_5__ix__, [BIT(5,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x6E]);
test!(bit_5__ix_DIS__, [BIT(5,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x6E], DIS: i8);
test!(bit_5__iy__, [BIT(5,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x6E]);
test!(bit_5__iy_DIS__, [BIT(5,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x6E], DIS: i8);
test!(bit_5_a_, [BIT(5,ByteRegister(A))], [0xCB, 0x6F]);
test!(bit_5_b_, [BIT(5,ByteRegister(B))], [0xCB, 0x68]);
test!(bit_5_c_, [BIT(5,ByteRegister(C))], [0xCB, 0x69]);
test!(bit_5_d_, [BIT(5,ByteRegister(D))], [0xCB, 0x6A]);
test!(bit_5_e_, [BIT(5,ByteRegister(E))], [0xCB, 0x6B]);
test!(bit_5_h_, [BIT(5,ByteRegister(H))], [0xCB, 0x6C]);
test!(bit_5_l_, [BIT(5,ByteRegister(L))], [0xCB, 0x6D]);
test!(bit_6__hl__, [BIT(6,AddressRegister(HL))], [0xCB, 0x76]);
test!(bit_6__ix__, [BIT(6,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x76]);
test!(bit_6__ix_DIS__, [BIT(6,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x76], DIS: i8);
test!(bit_6__iy__, [BIT(6,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x76]);
test!(bit_6__iy_DIS__, [BIT(6,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x76], DIS: i8);
test!(bit_6_a_, [BIT(6,ByteRegister(A))], [0xCB, 0x77]);
test!(bit_6_b_, [BIT(6,ByteRegister(B))], [0xCB, 0x70]);
test!(bit_6_c_, [BIT(6,ByteRegister(C))], [0xCB, 0x71]);
test!(bit_6_d_, [BIT(6,ByteRegister(D))], [0xCB, 0x72]);
test!(bit_6_e_, [BIT(6,ByteRegister(E))], [0xCB, 0x73]);
test!(bit_6_h_, [BIT(6,ByteRegister(H))], [0xCB, 0x74]);
test!(bit_6_l_, [BIT(6,ByteRegister(L))], [0xCB, 0x75]);
test!(bit_7__hl__, [BIT(7,AddressRegister(HL))], [0xCB, 0x7E]);
test!(bit_7__ix__, [BIT(7,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x7E]);
test!(bit_7__ix_DIS__, [BIT(7,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x7E], DIS: i8);
test!(bit_7__iy__, [BIT(7,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x7E]);
test!(bit_7__iy_DIS__, [BIT(7,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x7E], DIS: i8);
test!(bit_7_a_, [BIT(7,ByteRegister(A))], [0xCB, 0x7F]);
test!(bit_7_b_, [BIT(7,ByteRegister(B))], [0xCB, 0x78]);
test!(bit_7_c_, [BIT(7,ByteRegister(C))], [0xCB, 0x79]);
test!(bit_7_d_, [BIT(7,ByteRegister(D))], [0xCB, 0x7A]);
test!(bit_7_e_, [BIT(7,ByteRegister(E))], [0xCB, 0x7B]);
test!(bit_7_h_, [BIT(7,ByteRegister(H))], [0xCB, 0x7C]);
test!(bit_7_l_, [BIT(7,ByteRegister(L))], [0xCB, 0x7D]);
test!(call_NN_, [CALL(None, Constant(NN as i32))], [0xCD, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_c_NN_, [CALL(Some(Condition::C),Constant(NN as i32))], [0xDC, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_m_NN_, [CALL(Some(Condition::M),Constant(NN as i32))], [0xFC, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_nc_NN_, [CALL(Some(Condition::NC),Constant(NN as i32))], [0xD4, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_nz_NN_, [CALL(Some(Condition::NZ),Constant(NN as i32))], [0xC4, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_p_NN_, [CALL(Some(Condition::P),Constant(NN as i32))], [0xF4, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_pe_NN_, [CALL(Some(Condition::PE),Constant(NN as i32))], [0xEC, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_po_NN_, [CALL(Some(Condition::PO),Constant(NN as i32))], [0xE4, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(call_z_NN_, [CALL(Some(Condition::Z),Constant(NN as i32))], [0xCC, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ccf, [CCF], [0x3F]);
test!(cp__hl__, [CP(AddressRegister(HL))], [0xBE]);
test!(cp__ix__, [CP(AddressRegister(IX))], [0xDD, 0xBE, 0x00]);
test!(cp__ix_DIS__, [CP(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xBE, DIS], DIS: i8);
test!(cp__iy__, [CP(AddressRegister(IY))], [0xFD, 0xBE, 0x00]);
test!(cp__iy_DIS__, [CP(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xBE, DIS], DIS: i8);
test!(cp_N_, [CP(Constant(n as i32))], [0xFE, n], n: u8);
test!(cp_a_, [CP(ByteRegister(A))], [0xBF]);
test!(cp_b_, [CP(ByteRegister(B))], [0xB8]);
test!(cp_c_, [CP(ByteRegister(C))], [0xB9]);
test!(cp_d_, [CP(ByteRegister(D))], [0xBA]);
test!(cp_e_, [CP(ByteRegister(E))], [0xBB]);
test!(cp_h_, [CP(ByteRegister(H))], [0xBC]);
test_ub!(cp_ixh_, [CP(UndocumentedRegister(IXH))], [0xDD, 0xBC]);
test_ub!(cp_ixl_, [CP(UndocumentedRegister(IXL))], [0xDD, 0xBD]);
test_ub!(cp_iyh_, [CP(UndocumentedRegister(IYH))], [0xFD, 0xBC]);
test_ub!(cp_iyl_, [CP(UndocumentedRegister(IYL))], [0xFD, 0xBD]);
test!(cp_l_, [CP(ByteRegister(L))], [0xBD]);
test!(cpd, [CPD], [0xED, 0xA9]);
test!(cpdr, [CPDR], [0xED, 0xB9]);
test!(cpi, [CPI], [0xED, 0xA1]);
test!(cpir, [CPIR], [0xED, 0xB1]);
test!(cpl, [CPL], [0x2F]);
test!(daa, [DAA], [0x27]);
test!(dec__hl__, [DEC(AddressRegister(HL))], [0x35]);
test!(dec__ix__, [DEC(AddressRegister(IX))], [0xDD, 0x35, 0x00]);
test!(dec__ix_DIS__, [DEC(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x35, DIS], DIS: i8);
test!(dec__iy__, [DEC(AddressRegister(IY))], [0xFD, 0x35, 0x00]);
test!(dec__iy_DIS__, [DEC(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x35, DIS], DIS: i8);
test!(dec_a_, [DEC(ByteRegister(A))], [0x3D]);
test!(dec_b_, [DEC(ByteRegister(B))], [0x05]);
test!(dec_bc_, [DEC(WordRegister(BC))], [0x0B]);
test!(dec_c_, [DEC(ByteRegister(C))], [0x0D]);
test!(dec_d_, [DEC(ByteRegister(D))], [0x15]);
test!(dec_de_, [DEC(WordRegister(DE))], [0x1B]);
test!(dec_e_, [DEC(ByteRegister(E))], [0x1D]);
test!(dec_h_, [DEC(ByteRegister(H))], [0x25]);
test!(dec_hl_, [DEC(WordRegister(HL))], [0x2B]);
test!(dec_ix_, [DEC(WordRegister(IX))], [0xDD, 0x2B]);
test_ub!(dec_ixh_, [DEC(UndocumentedRegister(IXH))], [0xDD, 0x25]);
test_ub!(dec_ixl_, [DEC(UndocumentedRegister(IXL))], [0xDD, 0x2D]);
test!(dec_iy_, [DEC(WordRegister(IY))], [0xFD, 0x2B]);
test_ub!(dec_iyh_, [DEC(UndocumentedRegister(IYH))], [0xFD, 0x25]);
test_ub!(dec_iyl_, [DEC(UndocumentedRegister(IYL))], [0xFD, 0x2D]);
test!(dec_l_, [DEC(ByteRegister(L))], [0x2D]);
test!(dec_sp_, [DEC(WordRegister(SP))], [0x3B]);
test!(di, [DI], [0xF3]);
test!(ei, [EI], [0xFB]);
test!(ex__sp__hl_, [EX(AddressRegister(SP),WordRegister(HL))], [0xE3]);
test!(ex__sp__ix_, [EX(AddressRegister(SP),WordRegister(IX))], [0xDD, 0xE3]);
test!(ex__sp__iy_, [EX(AddressRegister(SP),WordRegister(IY))], [0xFD, 0xE3]);
test!(ex_af_af__, [EX(WordRegister(AF),WordRegister(AF_))], [0x08]);
test!(ex_de_hl_, [EX(WordRegister(DE),WordRegister(HL))], [0xEB]);
test!(exx, [EXX], [0xD9]);
test!(halt, [HALT], [0x76]);
test!(im_0_, [IM(0)], [0xED, 0x46]);
test!(im_1_, [IM(1)], [0xED, 0x56]);
test!(im_2_, [IM(2)], [0xED, 0x5E]);
test!(in_a__c__, [IN(ByteRegister(A),PortRegister(C))], [0xED, 0x78]);
test!(in_b__c__, [IN(ByteRegister(B),PortRegister(C))], [0xED, 0x40]);
test!(in_c__c__, [IN(ByteRegister(C),PortRegister(C))], [0xED, 0x48]);
test!(in_d__c__, [IN(ByteRegister(D),PortRegister(C))], [0xED, 0x50]);
test!(in_e__c__, [IN(ByteRegister(E),PortRegister(C))], [0xED, 0x58]);
test!(in_h__c__, [IN(ByteRegister(H),PortRegister(C))], [0xED, 0x60]);
test!(in_l__c__, [IN(ByteRegister(L),PortRegister(C))], [0xED, 0x68]);
test!(inc__hl__, [INC(AddressRegister(HL))], [0x34]);
test!(inc__ix__, [INC(AddressRegister(IX))], [0xDD, 0x34, 0x00]);
test!(inc__iy__, [INC(AddressRegister(IY))], [0xFD, 0x34, 0x00]);
test!(inc_a_, [INC(ByteRegister(A))], [0x3C]);
test!(inc_b_, [INC(ByteRegister(B))], [0x04]);
test!(inc_bc_, [INC(WordRegister(BC))], [0x03]);
test!(inc_c_, [INC(ByteRegister(C))], [0x0C]);
test!(inc_d_, [INC(ByteRegister(D))], [0x14]);
test!(inc_de_, [INC(WordRegister(DE))], [0x13]);
test!(inc_e_, [INC(ByteRegister(E))], [0x1C]);
test!(inc_h_, [INC(ByteRegister(H))], [0x24]);
test!(inc_hl_, [INC(WordRegister(HL))], [0x23]);
test!(inc_ix_, [INC(WordRegister(IX))], [0xDD, 0x23]);
test!(inc_iy_, [INC(WordRegister(IY))], [0xFD, 0x23]);
test!(inc_l_, [INC(ByteRegister(L))], [0x2C]);
test!(inc_sp_, [INC(WordRegister(SP))], [0x33]);
test!(ind, [IND], [0xED, 0xAA]);
test!(indr, [INDR], [0xED, 0xBA]);
test!(ini, [INI], [0xED, 0xA2]);
test!(inir, [INIR], [0xED, 0xB2]);
test!(jp__hl__, [JP(None, WordRegister(HL))], [0xE9]);
test!(jp__ix__, [JP(None, WordRegister(IX))], [0xDD, 0xE9]);
test!(jp__iy__, [JP(None, WordRegister(IY))], [0xFD, 0xE9]);
test!(ld__bc__a_, [LD(AddressRegister(BC),ByteRegister(A))], [0x02]);
test!(ld__de__a_, [LD(AddressRegister(DE),ByteRegister(A))], [0x12]);
test!(ld__hl__a_, [LD(AddressRegister(HL),ByteRegister(A))], [0x77]);
test!(ld__hl__b_, [LD(AddressRegister(HL),ByteRegister(B))], [0x70]);
test!(ld__hl__bc_, [LD(AddressRegister(HL),WordRegister(BC))], [0x71, 0x23, 0x70, 0x2B]);
test!(ld__hl__c_, [LD(AddressRegister(HL),ByteRegister(C))], [0x71]);
test!(ld__hl__d_, [LD(AddressRegister(HL),ByteRegister(D))], [0x72]);
test!(ld__hl__de_, [LD(AddressRegister(HL),WordRegister(DE))], [0x73, 0x23, 0x72, 0x2B]);
test!(ld__hl__e_, [LD(AddressRegister(HL),ByteRegister(E))], [0x73]);
test!(ld__hl__h_, [LD(AddressRegister(HL),ByteRegister(H))], [0x74]);
test!(ld__hl__l_, [LD(AddressRegister(HL),ByteRegister(L))], [0x75]);
test!(ld__ix__a_, [LD(AddressRegister(IX),ByteRegister(A))], [0xDD, 0x77, 0x00]);
test!(ld__ix__b_, [LD(AddressRegister(IX),ByteRegister(B))], [0xDD, 0x70, 0x00]);
test!(ld__ix__bc_, [LD(AddressRegister(IX),WordRegister(BC))], [0xDD, 0x71, 0x00, 0xDD, 0x70, 0x01]);
test!(ld__ix__c_, [LD(AddressRegister(IX),ByteRegister(C))], [0xDD, 0x71, 0x00]);
test!(ld__ix__d_, [LD(AddressRegister(IX),ByteRegister(D))], [0xDD, 0x72, 0x00]);
test!(ld__ix__de_, [LD(AddressRegister(IX),WordRegister(DE))], [0xDD, 0x73, 0x00, 0xDD, 0x72, 0x01]);
test!(ld__ix__e_, [LD(AddressRegister(IX),ByteRegister(E))], [0xDD, 0x73, 0x00]);
test!(ld__ix__h_, [LD(AddressRegister(IX),ByteRegister(H))], [0xDD, 0x74, 0x00]);
test!(ld__ix__hl_, [LD(AddressRegister(IX),WordRegister(HL))], [0xDD, 0x75, 0x00, 0xDD, 0x74, 0x01]);
test!(ld__ix__l_, [LD(AddressRegister(IX),ByteRegister(L))], [0xDD, 0x75, 0x00]);
test!(ld__iy__a_, [LD(AddressRegister(IY),ByteRegister(A))], [0xFD, 0x77, 0x00]);
test!(ld__iy__b_, [LD(AddressRegister(IY),ByteRegister(B))], [0xFD, 0x70, 0x00]);
test!(ld__iy__bc_, [LD(AddressRegister(IY),WordRegister(BC))], [0xFD, 0x71, 0x00, 0xFD, 0x70, 0x01]);
test!(ld__iy__c_, [LD(AddressRegister(IY),ByteRegister(C))], [0xFD, 0x71, 0x00]);
test!(ld__iy__d_, [LD(AddressRegister(IY),ByteRegister(D))], [0xFD, 0x72, 0x00]);
test!(ld__iy__de_, [LD(AddressRegister(IY),WordRegister(DE))], [0xFD, 0x73, 0x00, 0xFD, 0x72, 0x01]);
test!(ld__iy__e_, [LD(AddressRegister(IY),ByteRegister(E))], [0xFD, 0x73, 0x00]);
test!(ld__iy__h_, [LD(AddressRegister(IY),ByteRegister(H))], [0xFD, 0x74, 0x00]);
test!(ld__iy__hl_, [LD(AddressRegister(IY),WordRegister(HL))], [0xFD, 0x75, 0x00, 0xFD, 0x74, 0x01]);
test!(ld__iy__l_, [LD(AddressRegister(IY),ByteRegister(L))], [0xFD, 0x75, 0x00]);
test!(ld_a__de__, [LD(ByteRegister(A),AddressRegister(DE))], [0x1A]);
test!(ld_a__hl__, [LD(ByteRegister(A),AddressRegister(HL))], [0x7E]);
test!(ld_a__ix__, [LD(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x7E, 0x00]);
test!(ld_a__iy__, [LD(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x7E, 0x00]);
test!(ld_a_a_, [LD(ByteRegister(A),ByteRegister(A))], [0x7F]);
test!(ld_a_b_, [LD(ByteRegister(A),ByteRegister(B))], [0x78]);
test!(ld_a_c_, [LD(ByteRegister(A),ByteRegister(C))], [0x79]);
test!(ld_a_d_, [LD(ByteRegister(A),ByteRegister(D))], [0x7A]);
test!(ld_a_e_, [LD(ByteRegister(A),ByteRegister(E))], [0x7B]);
test!(ld_a_h_, [LD(ByteRegister(A),ByteRegister(H))], [0x7C]);
test!(ld_a_i_, [LD(ByteRegister(A),I)], [0xED, 0x57]);
test!(ld_a_l_, [LD(ByteRegister(A),ByteRegister(L))], [0x7D]);
test!(ld_a_r_, [LD(ByteRegister(A),R)], [0xED, 0x5F]);
test!(ld_b__hl__, [LD(ByteRegister(B),AddressRegister(HL))], [0x46]);
test!(ld_b__ix__, [LD(ByteRegister(B),AddressRegister(IX))], [0xDD, 0x46, 0x00]);
test!(ld_b__iy__, [LD(ByteRegister(B),AddressRegister(IY))], [0xFD, 0x46, 0x00]);
test!(ld_b_a_, [LD(ByteRegister(B),ByteRegister(A))], [0x47]);
test!(ld_b_b_, [LD(ByteRegister(B),ByteRegister(B))], [0x40]);
test!(ld_b_c_, [LD(ByteRegister(B),ByteRegister(C))], [0x41]);
test!(ld_b_d_, [LD(ByteRegister(B),ByteRegister(D))], [0x42]);
test!(ld_b_e_, [LD(ByteRegister(B),ByteRegister(E))], [0x43]);
test!(ld_b_h_, [LD(ByteRegister(B),ByteRegister(H))], [0x44]);
test!(ld_b_l_, [LD(ByteRegister(B),ByteRegister(L))], [0x45]);
test!(ld_bc__hl__, [LD(WordRegister(BC),AddressRegister(HL))], [0x4E, 0x23, 0x46, 0x2B]);
test!(ld_bc__ix__, [LD(WordRegister(BC),AddressRegister(IX))], [0xDD, 0x4E, 0x00, 0xDD, 0x46, 0x01]);
test!(ld_bc__iy__, [LD(WordRegister(BC),AddressRegister(IY))], [0xFD, 0x4E, 0x00, 0xFD, 0x46, 0x01]);
test!(ld_bc_bc_, [LD(WordRegister(BC),WordRegister(BC))], [0x40, 0x49]);
test!(ld_bc_de_, [LD(WordRegister(BC),WordRegister(DE))], [0x42, 0x4B]);
test!(ld_bc_hl_, [LD(WordRegister(BC),WordRegister(HL))], [0x44, 0x4D]);
test!(ld_bc_ix_, [LD(WordRegister(BC),WordRegister(IX))], [0xDD, 0x44, 0xDD, 0x4D]);
test!(ld_bc_iy_, [LD(WordRegister(BC),WordRegister(IY))], [0xFD, 0x44, 0xFD, 0x4D]);
test!(ld_c__hl__, [LD(ByteRegister(C),AddressRegister(HL))], [0x4E]);
test!(ld_c__ix__, [LD(ByteRegister(C),AddressRegister(IX))], [0xDD, 0x4E, 0x00]);
test!(ld_c__iy__, [LD(ByteRegister(C),AddressRegister(IY))], [0xFD, 0x4E, 0x00]);
test!(ld_c_a_, [LD(ByteRegister(C),ByteRegister(A))], [0x4F]);
test!(ld_c_b_, [LD(ByteRegister(C),ByteRegister(B))], [0x48]);
test!(ld_c_c_, [LD(ByteRegister(C),ByteRegister(C))], [0x49]);
test!(ld_c_d_, [LD(ByteRegister(C),ByteRegister(D))], [0x4A]);
test!(ld_c_e_, [LD(ByteRegister(C),ByteRegister(E))], [0x4B]);
test!(ld_c_h_, [LD(ByteRegister(C),ByteRegister(H))], [0x4C]);
test!(ld_c_l_, [LD(ByteRegister(C),ByteRegister(L))], [0x4D]);
test!(ld_d__hl__, [LD(ByteRegister(D),AddressRegister(HL))], [0x56]);
test!(ld_d__ix__, [LD(ByteRegister(D),AddressRegister(IX))], [0xDD, 0x56, 0x00]);
test!(ld_d__iy__, [LD(ByteRegister(D),AddressRegister(IY))], [0xFD, 0x56, 0x00]);
test!(ld_h_a_, [LD(ByteRegister(H),ByteRegister(A))], [0x67]);
test!(ld_h_b_, [LD(ByteRegister(H),ByteRegister(B))], [0x60]);
test!(ld_h_c_, [LD(ByteRegister(H),ByteRegister(C))], [0x61]);
test!(ld_h_d_, [LD(ByteRegister(H),ByteRegister(D))], [0x62]);
test!(ld_h_e_, [LD(ByteRegister(H),ByteRegister(E))], [0x63]);
test!(ld_h_h_, [LD(ByteRegister(H),ByteRegister(H))], [0x64]);
test!(ld_h_l_, [LD(ByteRegister(H),ByteRegister(L))], [0x65]);
test!(ld_hl_bc_, [LD(WordRegister(HL),WordRegister(BC))], [0x60, 0x69]);
test!(ld_hl_de_, [LD(WordRegister(HL),WordRegister(DE))], [0x62, 0x6B]);
test!(ld_hl_hl_, [LD(WordRegister(HL),WordRegister(HL))], [0x64, 0x6D]);
test!(ld_hl_ix_, [LD(WordRegister(HL),WordRegister(IX))], [0xDD, 0xE5, 0xE1]);
test!(ld_hl_iy_, [LD(WordRegister(HL),WordRegister(IY))], [0xFD, 0xE5, 0xE1]);
test!(ld_i_a_, [LD(I,ByteRegister(A))], [0xED, 0x47]);
test!(ld_ix_bc_, [LD(WordRegister(IX),WordRegister(BC))], [0xDD, 0x69, 0xDD, 0x60]);
test!(ld_ix_de_, [LD(WordRegister(IX),WordRegister(DE))], [0xDD, 0x6B, 0xDD, 0x62]);
test!(ld_ix_hl_, [LD(WordRegister(IX),WordRegister(HL))], [0xE5, 0xDD, 0xE1]);
test!(ld_ix_ix_, [LD(WordRegister(IX),WordRegister(IX))], [0xDD, 0x6D, 0xDD, 0x64]);
test!(ld_ix_iy_, [LD(WordRegister(IX),WordRegister(IY))], [0xFD, 0xE5, 0xDD, 0xE1]);
test!(ld_iy_bc_, [LD(WordRegister(IY),WordRegister(BC))], [0xFD, 0x69, 0xFD, 0x60]);
test!(ld_iy_de_, [LD(WordRegister(IY),WordRegister(DE))], [0xFD, 0x6B, 0xFD, 0x62]);
test!(ld_iy_hl_, [LD(WordRegister(IY),WordRegister(HL))], [0xE5, 0xFD, 0xE1]);
test!(ld_iy_ix_, [LD(WordRegister(IY),WordRegister(IX))], [0xDD, 0xE5, 0xFD, 0xE1]);
test!(ld_iy_iy_, [LD(WordRegister(IY),WordRegister(IY))], [0xFD, 0x6D, 0xFD, 0x64]);
test!(ld_l__hl__, [LD(ByteRegister(L),AddressRegister(HL))], [0x6E]);
test!(ld_l__ix__, [LD(ByteRegister(L),AddressRegister(IX))], [0xDD, 0x6E, 0x00]);
test!(ld_l__iy__, [LD(ByteRegister(L),AddressRegister(IY))], [0xFD, 0x6E, 0x00]);
test!(ld_l_a_, [LD(ByteRegister(L),ByteRegister(A))], [0x6F]);
test!(ld_l_b_, [LD(ByteRegister(L),ByteRegister(B))], [0x68]);
test!(ld_l_c_, [LD(ByteRegister(L),ByteRegister(C))], [0x69]);
test!(ld_l_d_, [LD(ByteRegister(L),ByteRegister(D))], [0x6A]);
test!(ld_l_e_, [LD(ByteRegister(L),ByteRegister(E))], [0x6B]);
test!(ld_l_h_, [LD(ByteRegister(L),ByteRegister(H))], [0x6C]);
test!(ld_l_l_, [LD(ByteRegister(L),ByteRegister(L))], [0x6D]);
test!(ld_r_a_, [LD(R,ByteRegister(A))], [0xED, 0x4F]);
test!(ld_sp_hl_, [LD(WordRegister(SP),WordRegister(HL))], [0xF9]);
test!(ld_sp_ix_, [LD(WordRegister(SP),WordRegister(IX))], [0xDD, 0xF9]);
test!(ld_sp_iy_, [LD(WordRegister(SP),WordRegister(IY))], [0xFD, 0xF9]);
test!(ldd, [LDD], [0xED, 0xA8]);

test_ub!(inc_ixh_, [INC(UndocumentedRegister(IXH))], [0xDD, 0x24]);
test_ub!(inc_ixl_, [INC(UndocumentedRegister(IXL))], [0xDD, 0x2C]);
test_ub!(inc_iyh_, [INC(UndocumentedRegister(IYH))], [0xFD, 0x24]);
test_ub!(inc_iyl_, [INC(UndocumentedRegister(IYL))], [0xFD, 0x2C]);
test_ub!(ld_a_ixh_, [LD(ByteRegister(A),UndocumentedRegister(IXH))], [0xDD, 0x7C]);
test_ub!(ld_a_ixl_, [LD(ByteRegister(A),UndocumentedRegister(IXL))], [0xDD, 0x7D]);
test_ub!(ld_a_iyh_, [LD(ByteRegister(A),UndocumentedRegister(IYH))], [0xFD, 0x7C]);
test_ub!(ld_a_iyl_, [LD(ByteRegister(A),UndocumentedRegister(IYL))], [0xFD, 0x7D]);
test_ub!(ld_b_ixh_, [LD(ByteRegister(B),UndocumentedRegister(IXH))], [0xDD, 0x44]);
test_ub!(ld_b_ixl_, [LD(ByteRegister(B),UndocumentedRegister(IXL))], [0xDD, 0x45]);
test_ub!(ld_b_iyh_, [LD(ByteRegister(B),UndocumentedRegister(IYH))], [0xFD, 0x44]);
test_ub!(ld_b_iyl_, [LD(ByteRegister(B),UndocumentedRegister(IYL))], [0xFD, 0x45]);
test_ub!(ld_c_ixh_, [LD(ByteRegister(C),UndocumentedRegister(IXH))], [0xDD, 0x4C]);
test_ub!(ld_c_ixl_, [LD(ByteRegister(C),UndocumentedRegister(IXL))], [0xDD, 0x4D]);
test_ub!(ld_c_iyh_, [LD(ByteRegister(C),UndocumentedRegister(IYH))], [0xFD, 0x4C]);
test_ub!(ld_c_iyl_, [LD(ByteRegister(C),UndocumentedRegister(IYL))], [0xFD, 0x4D]);
test!(ld_d_a_, [LD(ByteRegister(D),ByteRegister(A))], [0x57]);
test!(ld_d_b_, [LD(ByteRegister(D),ByteRegister(B))], [0x50]);
test!(ld_d_c_, [LD(ByteRegister(D),ByteRegister(C))], [0x51]);
test!(ld_d_d_, [LD(ByteRegister(D),ByteRegister(D))], [0x52]);
test!(ld_d_e_, [LD(ByteRegister(D),ByteRegister(E))], [0x53]);
test!(ld_d_h_, [LD(ByteRegister(D),ByteRegister(H))], [0x54]);
test_ub!(ld_d_ixh_, [LD(ByteRegister(D),UndocumentedRegister(IXH))], [0xDD, 0x54]);
test_ub!(ld_d_ixl_, [LD(ByteRegister(D),UndocumentedRegister(IXL))], [0xDD, 0x55]);
test_ub!(ld_d_iyh_, [LD(ByteRegister(D),UndocumentedRegister(IYH))], [0xFD, 0x54]);
test_ub!(ld_d_iyl_, [LD(ByteRegister(D),UndocumentedRegister(IYL))], [0xFD, 0x55]);
test!(ld_d_l_, [LD(ByteRegister(D),ByteRegister(L))], [0x55]);
test!(ld_e__hl__, [LD(ByteRegister(E),AddressRegister(HL))], [0x5E]);
test!(ld_e__ix__, [LD(ByteRegister(E),AddressRegister(IX))], [0xDD, 0x5E, 0x00]);
test!(ld_e_a_, [LD(ByteRegister(E),ByteRegister(A))], [0x5F]);
test!(ld_e_b_, [LD(ByteRegister(E),ByteRegister(B))], [0x58]);
test!(ld_e_c_, [LD(ByteRegister(E),ByteRegister(C))], [0x59]);
test!(ld_e_d_, [LD(ByteRegister(E),ByteRegister(D))], [0x5A]);
test!(ld_e_e_, [LD(ByteRegister(E),ByteRegister(E))], [0x5B]);
test!(ld_e_h_, [LD(ByteRegister(E),ByteRegister(H))], [0x5C]);
test_ub!(ld_e_ixh_, [LD(ByteRegister(E),UndocumentedRegister(IXH))], [0xDD, 0x5C]);
test_ub!(ld_e_ixl_, [LD(ByteRegister(E),UndocumentedRegister(IXL))], [0xDD, 0x5D]);
test_ub!(ld_e_iyh_, [LD(ByteRegister(E),UndocumentedRegister(IYH))], [0xFD, 0x5C]);
test_ub!(ld_e_iyl_, [LD(ByteRegister(E),UndocumentedRegister(IYL))], [0xFD, 0x5D]);
test!(ld_e_l_, [LD(ByteRegister(E),ByteRegister(L))], [0x5D]);
test!(ld_h__hl__, [LD(ByteRegister(H),AddressRegister(HL))], [0x66]);
test!(ld_h__ix__, [LD(ByteRegister(H),AddressRegister(IX))], [0xDD, 0x66, 0x00]);
test!(ld_h__iy__, [LD(ByteRegister(H),AddressRegister(IY))], [0xFD, 0x66, 0x00]);
test!(ld_hl__ix__, [LD(WordRegister(HL),AddressRegister(IX))], [0xDD, 0x6E, 0x00, 0xDD, 0x66, 0x01]);
test!(ld_hl__iy__, [LD(WordRegister(HL),AddressRegister(IY))], [0xFD, 0x6E, 0x00, 0xFD, 0x66, 0x01]);
test_ub!(ld_ixh_a_, [LD(UndocumentedRegister(IXH),ByteRegister(A))], [0xDD, 0x67]);
test_ub!(ld_ixh_b_, [LD(UndocumentedRegister(IXH),ByteRegister(B))], [0xDD, 0x60]);
test_ub!(ld_ixh_c_, [LD(UndocumentedRegister(IXH),ByteRegister(C))], [0xDD, 0x61]);
test_ub!(ld_ixh_d_, [LD(UndocumentedRegister(IXH),ByteRegister(D))], [0xDD, 0x62]);
test_ub!(ld_ixh_e_, [LD(UndocumentedRegister(IXH),ByteRegister(E))], [0xDD, 0x63]);
test_ub!(ld_ixh_ixh_, [LD(UndocumentedRegister(IXH),UndocumentedRegister(IXH))], [0xDD, 0x64]);
test_ub!(ld_ixh_ixl_, [LD(UndocumentedRegister(IXH),UndocumentedRegister(IXL))], [0xDD, 0x65]);
test_ub!(ld_ixl_a_, [LD(UndocumentedRegister(IXL),ByteRegister(A))], [0xDD, 0x6F]);
test_ub!(ld_ixl_b_, [LD(UndocumentedRegister(IXL),ByteRegister(B))], [0xDD, 0x68]);
test_ub!(ld_ixl_c_, [LD(UndocumentedRegister(IXL),ByteRegister(C))], [0xDD, 0x69]);
test_ub!(ld_ixl_d_, [LD(UndocumentedRegister(IXL),ByteRegister(D))], [0xDD, 0x6A]);
test_ub!(ld_ixl_e_, [LD(UndocumentedRegister(IXL),ByteRegister(E))], [0xDD, 0x6B]);
test_ub!(ld_ixl_ixh_, [LD(UndocumentedRegister(IXL),UndocumentedRegister(IXH))], [0xDD, 0x6C]);
test_ub!(ld_ixl_ixl_, [LD(UndocumentedRegister(IXL),UndocumentedRegister(IXL))], [0xDD, 0x6D]);
test_ub!(ld_iyh_a_, [LD(UndocumentedRegister(IYH),ByteRegister(A))], [0xFD, 0x67]);
test_ub!(ld_iyh_b_, [LD(UndocumentedRegister(IYH),ByteRegister(B))], [0xFD, 0x60]);
test_ub!(ld_iyh_c_, [LD(UndocumentedRegister(IYH),ByteRegister(C))], [0xFD, 0x61]);
test_ub!(ld_iyh_d_, [LD(UndocumentedRegister(IYH),ByteRegister(D))], [0xFD, 0x62]);
test_ub!(ld_iyh_e_, [LD(UndocumentedRegister(IYH),ByteRegister(E))], [0xFD, 0x63]);
test_ub!(ld_iyh_iyh_, [LD(UndocumentedRegister(IYH),UndocumentedRegister(IYH))], [0xFD, 0x64]);
test_ub!(ld_iyh_iyl_, [LD(UndocumentedRegister(IYH),UndocumentedRegister(IYL))], [0xFD, 0x65]);
test_ub!(ld_iyl_a_, [LD(UndocumentedRegister(IYL),ByteRegister(A))], [0xFD, 0x6F]);
test_ub!(ld_iyl_b_, [LD(UndocumentedRegister(IYL),ByteRegister(B))], [0xFD, 0x68]);
test_ub!(ld_iyl_c_, [LD(UndocumentedRegister(IYL),ByteRegister(C))], [0xFD, 0x69]);
test_ub!(ld_iyl_d_, [LD(UndocumentedRegister(IYL),ByteRegister(D))], [0xFD, 0x6A]);
test_ub!(ld_iyl_e_, [LD(UndocumentedRegister(IYL),ByteRegister(E))], [0xFD, 0x6B]);
test_ub!(ld_iyl_iyh_, [LD(UndocumentedRegister(IYL),UndocumentedRegister(IYH))], [0xFD, 0x6C]);
test_ub!(ld_iyl_iyl_, [LD(UndocumentedRegister(IYL),UndocumentedRegister(IYL))], [0xFD, 0x6D]);
test!(lddr, [LDDR], [0xED, 0xB8]);
test!(ldi, [LDI], [0xED, 0xA0]);
test!(ldir, [LDIR], [0xED, 0xB0]);
test!(neg, [NEG], [0xED, 0x44]);
test!(nop, [NOP], [0x00]);
test!(or__hl__, [OR(AddressRegister(HL))], [0xB6]);
test!(or__ix__, [OR(AddressRegister(IX))], [0xDD, 0xB6, 0x00]);
test!(or__iy__, [OR(AddressRegister(IY))], [0xFD, 0xB6, 0x00]);
test!(or_a_, [OR(ByteRegister(A))], [0xB7]);
test!(or_b_, [OR(ByteRegister(B))], [0xB0]);
test!(or_c_, [OR(ByteRegister(C))], [0xB1]);
test!(or_d_, [OR(ByteRegister(D))], [0xB2]);
test!(or_e_, [OR(ByteRegister(E))], [0xB3]);
test!(or_h_, [OR(ByteRegister(H))], [0xB4]);
test_ub!(or_ixh_, [OR(UndocumentedRegister(IXH))], [0xDD, 0xB4]);
test_ub!(or_ixl_, [OR(UndocumentedRegister(IXL))], [0xDD, 0xB5]);
test_ub!(or_iyh_, [OR(UndocumentedRegister(IYH))], [0xFD, 0xB4]);
test_ub!(or_iyl_, [OR(UndocumentedRegister(IYL))], [0xFD, 0xB5]);
test!(or_l_, [OR(ByteRegister(L))], [0xB5]);
test!(otdr, [OTDR], [0xED, 0xBB]);
test!(otir, [OTIR], [0xED, 0xB3]);
test_ub!(out__c__0_, [OUT(PortRegister(C),Constant(0))], [0xED, 0x71]);
test!(out__c__a_, [OUT(PortRegister(C),ByteRegister(A))], [0xED, 0x79]);
test!(out__c__b_, [OUT(PortRegister(C),ByteRegister(B))], [0xED, 0x41]);
test!(out__c__c_, [OUT(PortRegister(C),ByteRegister(C))], [0xED, 0x49]);
test!(out__c__d_, [OUT(PortRegister(C),ByteRegister(D))], [0xED, 0x51]);
test!(out__c__e_, [OUT(PortRegister(C),ByteRegister(E))], [0xED, 0x59]);
test!(out__c__h_, [OUT(PortRegister(C),ByteRegister(H))], [0xED, 0x61]);
test!(out__c__l_, [OUT(PortRegister(C),ByteRegister(L))], [0xED, 0x69]);
test!(outd, [OUTD], [0xED, 0xAB]);
test!(outi, [OUTI], [0xED, 0xA3]);
test!(pop_af_, [POP(WordRegister(AF))], [0xF1]);
test!(pop_bc_, [POP(WordRegister(BC))], [0xC1]);
test!(pop_de_, [POP(WordRegister(DE))], [0xD1]);
test!(pop_hl_, [POP(WordRegister(HL))], [0xE1]);
test!(pop_ix_, [POP(WordRegister(IX))], [0xDD, 0xE1]);
test!(pop_iy_, [POP(WordRegister(IY))], [0xFD, 0xE1]);
test!(push_af_, [PUSH(WordRegister(AF))], [0xF5]);
test!(push_bc_, [PUSH(WordRegister(BC))], [0xC5]);
test!(push_de_, [PUSH(WordRegister(DE))], [0xD5]);
test!(push_hl_, [PUSH(WordRegister(HL))], [0xE5]);
test!(push_ix_, [PUSH(WordRegister(IX))], [0xDD, 0xE5]);
test!(push_iy_, [PUSH(WordRegister(IY))], [0xFD, 0xE5]);
test!(res_0__hl__, [RES(0,AddressRegister(HL))], [0xCB, 0x86]);
test!(res_0__ix__, [RES(0,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x86]);
test!(res_0_a_, [RES(0,ByteRegister(A))], [0xCB, 0x87]);
test!(res_0_b_, [RES(0,ByteRegister(B))], [0xCB, 0x80]);
test!(res_0_c_, [RES(0,ByteRegister(C))], [0xCB, 0x81]);
test!(res_0_d_, [RES(0,ByteRegister(D))], [0xCB, 0x82]);
test!(res_0_e_, [RES(0,ByteRegister(E))], [0xCB, 0x83]);
test!(res_0_h_, [RES(0,ByteRegister(H))], [0xCB, 0x84]);
test!(res_0_l_, [RES(0,ByteRegister(L))], [0xCB, 0x85]);
test!(res_0__ix_DIS__, [RES(0,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x86], DIS: i8);
test!(res_0__iy__, [RES(0,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x86]);
test!(res_0__iy_DIS__, [RES(0,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x86], DIS: i8);
test!(res_1_a_, [RES(1,ByteRegister(A))], [0xCB, 0x8F]);
test!(res_1_b_, [RES(1,ByteRegister(B))], [0xCB, 0x88]);
test!(res_1_c_, [RES(1,ByteRegister(C))], [0xCB, 0x89]);
test!(res_1_d_, [RES(1,ByteRegister(D))], [0xCB, 0x8A]);
test!(res_1_e_, [RES(1,ByteRegister(E))], [0xCB, 0x8B]);
test!(res_1_h_, [RES(1,ByteRegister(H))], [0xCB, 0x8C]);
test!(res_1_l_, [RES(1,ByteRegister(L))], [0xCB, 0x8D]);
test!(res_1__hl__, [RES(1,AddressRegister(HL))], [0xCB, 0x8E]);
test!(res_1__ix__, [RES(1,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x8E]);
test!(res_1__ix_DIS__, [RES(1,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x8E], DIS: i8);
test!(res_1__iy__, [RES(1,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x8E]);
test!(res_1__iy_DIS__, [RES(1,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x8E], DIS: i8);
test!(res_2_a_, [RES(2,ByteRegister(A))], [0xCB, 0x97]);
test!(res_2_b_, [RES(2,ByteRegister(B))], [0xCB, 0x90]);
test!(res_2_c_, [RES(2,ByteRegister(C))], [0xCB, 0x91]);
test!(res_2_d_, [RES(2,ByteRegister(D))], [0xCB, 0x92]);
test!(res_2_e_, [RES(2,ByteRegister(E))], [0xCB, 0x93]);
test!(res_2_h_, [RES(2,ByteRegister(H))], [0xCB, 0x94]);
test!(res_2_l_, [RES(2,ByteRegister(L))], [0xCB, 0x95]);
test!(res_2__hl__, [RES(2,AddressRegister(HL))], [0xCB, 0x96]);
test!(res_2__ix__, [RES(2,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x96]);
test!(res_2__ix_DIS__, [RES(2,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x96], DIS: i8);
test!(res_2__iy__, [RES(2,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x96]);
test!(res_2__iy_DIS__, [RES(2,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x96], DIS: i8);
test!(res_3_a_, [RES(3,ByteRegister(A))], [0xCB, 0x9F]);
test!(res_3_b_, [RES(3,ByteRegister(B))], [0xCB, 0x98]);
test!(res_3_c_, [RES(3,ByteRegister(C))], [0xCB, 0x99]);
test!(res_3_d_, [RES(3,ByteRegister(D))], [0xCB, 0x9A]);
test!(res_3_e_, [RES(3,ByteRegister(E))], [0xCB, 0x9B]);
test!(res_3_h_, [RES(3,ByteRegister(H))], [0xCB, 0x9C]);
test!(res_3_l_, [RES(3,ByteRegister(L))], [0xCB, 0x9D]);
test!(res_3__hl__, [RES(3,AddressRegister(HL))], [0xCB, 0x9E]);
test!(res_3__ix__, [RES(3,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x9E]);
test!(res_3__ix_DIS__, [RES(3,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x9E], DIS: i8);
test!(res_3__iy__, [RES(3,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x9E]);
test!(res_3__iy_DIS__, [RES(3,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x9E], DIS: i8);
test!(res_4_a_, [RES(4,ByteRegister(A))], [0xCB, 0xA7]);
test!(res_4_b_, [RES(4,ByteRegister(B))], [0xCB, 0xA0]);
test!(res_4_c_, [RES(4,ByteRegister(C))], [0xCB, 0xA1]);
test!(res_4_d_, [RES(4,ByteRegister(D))], [0xCB, 0xA2]);
test!(res_4_e_, [RES(4,ByteRegister(E))], [0xCB, 0xA3]);
test!(res_4_h_, [RES(4,ByteRegister(H))], [0xCB, 0xA4]);
test!(res_4_l_, [RES(4,ByteRegister(L))], [0xCB, 0xA5]);
test!(res_4__hl__, [RES(4,AddressRegister(HL))], [0xCB, 0xA6]);
test!(res_4__ix__, [RES(4,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xA6]);
test!(res_4__ix_DIS__, [RES(4,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xA6], DIS: i8);
test!(res_4__iy__, [RES(4,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xA6]);
test!(res_4__iy_DIS__, [RES(4,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xA6], DIS: i8);
test!(res_5_a_, [RES(5,ByteRegister(A))], [0xCB, 0xAF]);
test!(res_5_b_, [RES(5,ByteRegister(B))], [0xCB, 0xA8]);
test!(res_5_c_, [RES(5,ByteRegister(C))], [0xCB, 0xA9]);
test!(res_5_d_, [RES(5,ByteRegister(D))], [0xCB, 0xAA]);
test!(res_5_e_, [RES(5,ByteRegister(E))], [0xCB, 0xAB]);
test!(res_5_h_, [RES(5,ByteRegister(H))], [0xCB, 0xAC]);
test!(res_5_l_, [RES(5,ByteRegister(L))], [0xCB, 0xAD]);
test!(res_5__hl__, [RES(5,AddressRegister(HL))], [0xCB, 0xAE]);
test!(res_5__ix__, [RES(5,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xAE]);
test!(res_5__ix_DIS__, [RES(5,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xAE], DIS: i8);
test!(res_5__iy__, [RES(5,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xAE]);
test!(res_5__iy_DIS__, [RES(5,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xAE], DIS: i8);
test!(res_6_a_, [RES(6,ByteRegister(A))], [0xCB, 0xB7]);
test!(res_6_b_, [RES(6,ByteRegister(B))], [0xCB, 0xB0]);
test!(res_6_c_, [RES(6,ByteRegister(C))], [0xCB, 0xB1]);
test!(res_6_d_, [RES(6,ByteRegister(D))], [0xCB, 0xB2]);
test!(res_6_e_, [RES(6,ByteRegister(E))], [0xCB, 0xB3]);
test!(res_6_h_, [RES(6,ByteRegister(H))], [0xCB, 0xB4]);
test!(res_6_l_, [RES(6,ByteRegister(L))], [0xCB, 0xB5]);
test!(res_6__hl__, [RES(6,AddressRegister(HL))], [0xCB, 0xB6]);
test!(res_6__ix__, [RES(6,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xB6]);
test!(res_6__ix_DIS__, [RES(6,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xB6], DIS: i8);
test!(res_6__iy__, [RES(6,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xB6]);
test!(res_6__iy_DIS__, [RES(6,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xB6], DIS: i8);
test!(res_7_a_, [RES(7,ByteRegister(A))], [0xCB, 0xBF]);
test!(res_7_b_, [RES(7,ByteRegister(B))], [0xCB, 0xB8]);
test!(res_7_c_, [RES(7,ByteRegister(C))], [0xCB, 0xB9]);
test!(res_7_d_, [RES(7,ByteRegister(D))], [0xCB, 0xBA]);
test!(res_7_e_, [RES(7,ByteRegister(E))], [0xCB, 0xBB]);
test!(res_7_h_, [RES(7,ByteRegister(H))], [0xCB, 0xBC]);
test!(res_7_l_, [RES(7,ByteRegister(L))], [0xCB, 0xBD]);
test!(res_7__hl__, [RES(7,AddressRegister(HL))], [0xCB, 0xBE]);
test!(res_7__ix__, [RES(7,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xBE]);
test!(res_7__ix_DIS__, [RES(7,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xBE], DIS: i8);
test!(res_7__iy__, [RES(7,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xBE]);
test!(res_7__iy_DIS__, [RES(7,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xBE], DIS: i8);
test!(ret, [RET(None)], [0xC9]);
test!(ret_c_, [RET(Some(Condition::C))], [0xD8]);
test!(ret_m_, [RET(Some(Condition::M))], [0xF8]);
test!(ret_nc_, [RET(Some(Condition::NC))], [0xD0]);
test!(ret_nz_, [RET(Some(Condition::NZ))], [0xC0]);
test!(ret_p_, [RET(Some(Condition::P))], [0xF0]);
test!(ret_pe_, [RET(Some(Condition::PE))], [0xE8]);
test!(ret_po_, [RET(Some(Condition::PO))], [0xE0]);
test!(ret_z_, [RET(Some(Condition::Z))], [0xC8]);
test!(reti, [RETI], [0xED, 0x4D]);
test!(retn, [RETN], [0xED, 0x45]);
test!(rl__hl__, [RL(AddressRegister(HL))], [0xCB, 0x16]);
test!(rl__ix__, [RL(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x16]);
test!(rl__iy__, [RL(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x16]);
test!(rl__ix_DIS__, [RL(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x16], DIS: i8);
test!(rl__iy_DIS__, [RL(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x16], DIS: i8);
test!(rl_a_, [RL(ByteRegister(A))], [0xCB, 0x17]);
test!(rl_b_, [RL(ByteRegister(B))], [0xCB, 0x10]);
test!(rl_bc_, [RL(WordRegister(BC))], [0xCB, 0x11, 0xCB, 0x10]);
test!(rl_c_, [RL(ByteRegister(C))], [0xCB, 0x11]);
test!(rl_d_, [RL(ByteRegister(D))], [0xCB, 0x12]);
test!(rl_de_, [RL(WordRegister(DE))], [0xCB, 0x13, 0xCB, 0x12]);
test!(rl_e_, [RL(ByteRegister(E))], [0xCB, 0x13]);
test!(rl_h_, [RL(ByteRegister(H))], [0xCB, 0x14]);
test!(rl_hl_, [RL(WordRegister(HL))], [0xCB, 0x15, 0xCB, 0x14]);
test!(rl_l_, [RL(ByteRegister(L))], [0xCB, 0x15]);
test!(rla, [RLA], [0x17]);
test!(rlc__hl__, [RLC(AddressRegister(HL))], [0xCB, 0x06]);
test!(rlc__ix__, [RLC(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x06]);
test!(rlc__ix_DIS__, [RLC(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x06], DIS: i8);
test!(rlc__iy__, [RLC(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x06]);
test!(rlc__iy_DIS__, [RLC(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x06], DIS: i8);
test!(rlc_a_, [RLC(ByteRegister(A))], [0xCB, 0x07]);
test!(rlc_b_, [RLC(ByteRegister(B))], [0xCB, 0x00]);
test!(rlc_c_, [RLC(ByteRegister(C))], [0xCB, 0x01]);
test!(rlc_d_, [RLC(ByteRegister(D))], [0xCB, 0x02]);
test!(rlc_e_, [RLC(ByteRegister(E))], [0xCB, 0x03]);
test!(rlc_h_, [RLC(ByteRegister(H))], [0xCB, 0x04]);
test!(rlc_l_, [RLC(ByteRegister(L))], [0xCB, 0x05]);
test!(rlca, [RLCA], [0x07]);
test!(rld, [RLD], [0xED, 0x6F]);
test!(rr__hl__, [RR(AddressRegister(HL))], [0xCB, 0x1E]);
test!(rr__ix__, [RR(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x1E]);
test!(rr__ix_DIS__, [RR(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x1E], DIS: i8);
test!(rr__iy__, [RR(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x1E]);
test!(rr__iy_DIS__, [RR(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x1E], DIS: i8);
test!(rr_a_, [RR(ByteRegister(A))], [0xCB, 0x1F]);
test!(rr_b_, [RR(ByteRegister(B))], [0xCB, 0x18]);
test!(rr_bc_, [RR(WordRegister(BC))], [0xCB, 0x18, 0xCB, 0x19]);
test!(rr_c_, [RR(ByteRegister(C))], [0xCB, 0x19]);
test!(rr_d_, [RR(ByteRegister(D))], [0xCB, 0x1A]);
test!(rr_de_, [RR(WordRegister(DE))], [0xCB, 0x1A, 0xCB, 0x1B]);
test!(rr_e_, [RR(ByteRegister(E))], [0xCB, 0x1B]);
test!(rr_h_, [RR(ByteRegister(H))], [0xCB, 0x1C]);
test!(rr_hl_, [RR(WordRegister(HL))], [0xCB, 0x1C, 0xCB, 0x1D]);
test!(rr_l_, [RR(ByteRegister(L))], [0xCB, 0x1D]);
test!(rra, [RRA], [0x1F]);
test!(rrc__hl__, [RRC(AddressRegister(HL))], [0xCB, 0x0E]);
test!(rrc__ix__, [RRC(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x0E]);
test!(rrc__ix_DIS__, [RRC(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x0E], DIS: i8);
test!(rrc__iy__, [RRC(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x0E]);
test!(rrc__iy_DIS__, [RRC(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x0E], DIS: i8);
test!(rrc_a_, [RRC(ByteRegister(A))], [0xCB, 0x0F]);
test!(rrc_b_, [RRC(ByteRegister(B))], [0xCB, 0x08]);
test!(rrc_c_, [RRC(ByteRegister(C))], [0xCB, 0x09]);
test!(rrc_d_, [RRC(ByteRegister(D))], [0xCB, 0x0A]);
test!(rrc_e_, [RRC(ByteRegister(E))], [0xCB, 0x0B]);
test!(rrc_h_, [RRC(ByteRegister(H))], [0xCB, 0x0C]);
test!(rrc_l_, [RRC(ByteRegister(L))], [0xCB, 0x0D]);
test!(rrca, [RRCA], [0x0F]);
test!(rrd, [RRD], [0xED, 0x67]);
test!(rst_0_, [RST(0)], [0xC7]);
test!(rst_8_, [RST(8)], [0xCF]);
test!(rst_16_, [RST(16)], [0xD7]);
test!(rst_24_, [RST(24)], [0xDF]);
test!(rst_32_, [RST(32)], [0xE7]);
test!(rst_40_, [RST(40)], [0xEF]);
test!(rst_48_, [RST(48)], [0xF7]);
test!(rst_56_, [RST(56)], [0xFF]);
test!(sbc_a__hl__, [SBC(ByteRegister(A),AddressRegister(HL))], [0x9E]);
test!(sbc_a__ix__, [SBC(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x9E, 0x00]);
test!(sbc_a__ix_DIS__, [SBC(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x9E, DIS], DIS: i8);
test!(sbc_a__iy__, [SBC(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x9E, 0x00]);
test!(sbc_a__iy_DIS__, [SBC(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x9E, DIS], DIS: i8);
test!(sbc_a_a_, [SBC(ByteRegister(A),ByteRegister(A))], [0x9F]);
test!(sbc_a_b_, [SBC(ByteRegister(A),ByteRegister(B))], [0x98]);
test!(sbc_a_c_, [SBC(ByteRegister(A),ByteRegister(C))], [0x99]);
test!(sbc_a_d_, [SBC(ByteRegister(A),ByteRegister(D))], [0x9A]);
test!(sbc_a_e_, [SBC(ByteRegister(A),ByteRegister(E))], [0x9B]);
test!(sbc_a_h_, [SBC(ByteRegister(A),ByteRegister(H))], [0x9C]);
test_ub!(sbc_a_ixh_, [SBC(ByteRegister(A),UndocumentedRegister(IXH))], [0xDD, 0x9C]);
test_ub!(sbc_a_ixl_, [SBC(ByteRegister(A),UndocumentedRegister(IXL))], [0xDD, 0x9D]);
test_ub!(sbc_a_iyh_, [SBC(ByteRegister(A),UndocumentedRegister(IYH))], [0xFD, 0x9C]);
test_ub!(sbc_a_iyl_, [SBC(ByteRegister(A),UndocumentedRegister(IYL))], [0xFD, 0x9D]);
test!(sbc_a_l_, [SBC(ByteRegister(A),ByteRegister(L))], [0x9D]);
test!(sbc_hl_bc_, [SBC(WordRegister(HL),WordRegister(BC))], [0xED, 0x42]);
test!(sbc_hl_de_, [SBC(WordRegister(HL),WordRegister(DE))], [0xED, 0x52]);
test!(sbc_hl_hl_, [SBC(WordRegister(HL),WordRegister(HL))], [0xED, 0x62]);
test!(sbc_hl_sp_, [SBC(WordRegister(HL),WordRegister(SP))], [0xED, 0x72]);
test!(scf, [SCF], [0x37]);
test!(set_0__hl__, [SET(0,AddressRegister(HL))], [0xCB, 0xC6]);
test!(set_0__ix__, [SET(0,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xC6]);
test!(set_0__ix_DIS__, [SET(0,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xC6], DIS: i8);
test!(set_0__iy__, [SET(0,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xC6]);
test!(set_0__iy_DIS__, [SET(0,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xC6], DIS: i8);
test!(set_0_a_, [SET(0,ByteRegister(A))], [0xCB, 0xC7]);
test!(set_0_b_, [SET(0,ByteRegister(B))], [0xCB, 0xC0]);
test!(set_0_c_, [SET(0,ByteRegister(C))], [0xCB, 0xC1]);
test!(set_0_d_, [SET(0,ByteRegister(D))], [0xCB, 0xC2]);
test!(set_0_e_, [SET(0,ByteRegister(E))], [0xCB, 0xC3]);
test!(set_0_h_, [SET(0,ByteRegister(H))], [0xCB, 0xC4]);
test!(set_0_l_, [SET(0,ByteRegister(L))], [0xCB, 0xC5]);
test!(set_1__hl__, [SET(1,AddressRegister(HL))], [0xCB, 0xCE]);
test!(set_1__ix__, [SET(1,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xCE]);
test!(set_1__ix_DIS__, [SET(1,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xCE], DIS: i8);
test!(set_1__iy__, [SET(1,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xCE]);
test!(set_1__iy_DIS__, [SET(1,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xCE], DIS: i8);
test!(set_1_a_, [SET(1,ByteRegister(A))], [0xCB, 0xCF]);
test!(set_1_b_, [SET(1,ByteRegister(B))], [0xCB, 0xC8]);
test!(set_1_c_, [SET(1,ByteRegister(C))], [0xCB, 0xC9]);
test!(set_1_d_, [SET(1,ByteRegister(D))], [0xCB, 0xCA]);
test!(set_1_e_, [SET(1,ByteRegister(E))], [0xCB, 0xCB]);
test!(set_1_h_, [SET(1,ByteRegister(H))], [0xCB, 0xCC]);
test!(set_1_l_, [SET(1,ByteRegister(L))], [0xCB, 0xCD]);
test!(set_2__hl__, [SET(2,AddressRegister(HL))], [0xCB, 0xD6]);
test!(set_2__ix__, [SET(2,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xD6]);
test!(set_2__ix_DIS__, [SET(2,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xD6], DIS: i8);
test!(set_2__iy__, [SET(2,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xD6]);
test!(set_2__iy_DIS__, [SET(2,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xD6], DIS: i8);
test!(set_2_a_, [SET(2,ByteRegister(A))], [0xCB, 0xD7]);
test!(set_2_b_, [SET(2,ByteRegister(B))], [0xCB, 0xD0]);
test!(set_2_c_, [SET(2,ByteRegister(C))], [0xCB, 0xD1]);
test!(set_2_d_, [SET(2,ByteRegister(D))], [0xCB, 0xD2]);
test!(set_2_e_, [SET(2,ByteRegister(E))], [0xCB, 0xD3]);
test!(set_2_h_, [SET(2,ByteRegister(H))], [0xCB, 0xD4]);
test!(set_2_l_, [SET(2,ByteRegister(L))], [0xCB, 0xD5]);
test!(set_3__hl__, [SET(3,AddressRegister(HL))], [0xCB, 0xDE]);
test!(set_3__ix__, [SET(3,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xDE]);
test!(set_3__ix_DIS__, [SET(3,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xDE], DIS: i8);
test!(set_3__iy__, [SET(3,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xDE]);
test!(set_3__iy_DIS__, [SET(3,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xDE], DIS: i8);
test!(set_3_a_, [SET(3,ByteRegister(A))], [0xCB, 0xDF]);
test!(set_3_b_, [SET(3,ByteRegister(B))], [0xCB, 0xD8]);
test!(set_3_c_, [SET(3,ByteRegister(C))], [0xCB, 0xD9]);
test!(set_3_d_, [SET(3,ByteRegister(D))], [0xCB, 0xDA]);
test!(set_3_e_, [SET(3,ByteRegister(E))], [0xCB, 0xDB]);
test!(set_3_h_, [SET(3,ByteRegister(H))], [0xCB, 0xDC]);
test!(set_3_l_, [SET(3,ByteRegister(L))], [0xCB, 0xDD]);
test!(set_4__hl__, [SET(4,AddressRegister(HL))], [0xCB, 0xE6]);
test!(set_4__ix__, [SET(4,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xE6]);
test!(set_4__ix_DIS__, [SET(4,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xE6], DIS: i8);
test!(set_4__iy__, [SET(4,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xE6]);
test!(set_4__iy_DIS__, [SET(4,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xE6], DIS: i8);
test!(set_4_a_, [SET(4,ByteRegister(A))], [0xCB, 0xE7]);
test!(set_4_b_, [SET(4,ByteRegister(B))], [0xCB, 0xE0]);
test!(set_4_c_, [SET(4,ByteRegister(C))], [0xCB, 0xE1]);
test!(set_4_d_, [SET(4,ByteRegister(D))], [0xCB, 0xE2]);
test!(set_4_e_, [SET(4,ByteRegister(E))], [0xCB, 0xE3]);
test!(set_4_h_, [SET(4,ByteRegister(H))], [0xCB, 0xE4]);
test!(set_4_l_, [SET(4,ByteRegister(L))], [0xCB, 0xE5]);
test!(set_5__hl__, [SET(5,AddressRegister(HL))], [0xCB, 0xEE]);
test!(set_5__ix__, [SET(5,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xEE]);
test!(set_5__ix_DIS__, [SET(5,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xEE], DIS: i8);
test!(set_5__iy__, [SET(5,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xEE]);
test!(set_5__iy_DIS__, [SET(5,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xEE], DIS: i8);
test!(set_5_a_, [SET(5,ByteRegister(A))], [0xCB, 0xEF]);
test!(set_5_b_, [SET(5,ByteRegister(B))], [0xCB, 0xE8]);
test!(set_5_c_, [SET(5,ByteRegister(C))], [0xCB, 0xE9]);
test!(set_5_d_, [SET(5,ByteRegister(D))], [0xCB, 0xEA]);
test!(set_5_e_, [SET(5,ByteRegister(E))], [0xCB, 0xEB]);
test!(set_5_h_, [SET(5,ByteRegister(H))], [0xCB, 0xEC]);
test!(set_5_l_, [SET(5,ByteRegister(L))], [0xCB, 0xED]);
test!(set_6__hl__, [SET(6,AddressRegister(HL))], [0xCB, 0xF6]);
test!(set_6__ix__, [SET(6,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xF6]);
test!(set_6__ix_DIS__, [SET(6,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xF6], DIS: i8);
test!(set_6__iy__, [SET(6,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xF6]);
test!(set_6__iy_DIS__, [SET(6,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xF6], DIS: i8);
test!(set_6_a_, [SET(6,ByteRegister(A))], [0xCB, 0xF7]);
test!(set_6_b_, [SET(6,ByteRegister(B))], [0xCB, 0xF0]);
test!(set_6_c_, [SET(6,ByteRegister(C))], [0xCB, 0xF1]);
test!(set_6_d_, [SET(6,ByteRegister(D))], [0xCB, 0xF2]);
test!(set_6_e_, [SET(6,ByteRegister(E))], [0xCB, 0xF3]);
test!(set_6_h_, [SET(6,ByteRegister(H))], [0xCB, 0xF4]);
test!(set_6_l_, [SET(6,ByteRegister(L))], [0xCB, 0xF5]);
test!(set_7__hl__, [SET(7,AddressRegister(HL))], [0xCB, 0xFE]);
test!(set_7__ix__, [SET(7,AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0xFE]);
test!(set_7__ix_DIS__, [SET(7,AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0xFE], DIS: i8);
test!(set_7__iy__, [SET(7,AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0xFE]);
test!(set_7__iy_DIS__, [SET(7,AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0xFE], DIS: i8);
test!(set_7_a_, [SET(7,ByteRegister(A))], [0xCB, 0xFF]);
test!(set_7_b_, [SET(7,ByteRegister(B))], [0xCB, 0xF8]);
test!(set_7_c_, [SET(7,ByteRegister(C))], [0xCB, 0xF9]);
test!(set_7_d_, [SET(7,ByteRegister(D))], [0xCB, 0xFA]);
test!(set_7_e_, [SET(7,ByteRegister(E))], [0xCB, 0xFB]);
test!(set_7_h_, [SET(7,ByteRegister(H))], [0xCB, 0xFC]);
test!(set_7_l_, [SET(7,ByteRegister(L))], [0xCB, 0xFD]);
test!(sla_a_, [SLA(ByteRegister(A))], [0xCB, 0x27]);
test!(sla_b_, [SLA(ByteRegister(B))], [0xCB, 0x20]);
test!(sla_bc_, [SLA(WordRegister(BC))], [0xCB, 0x21, 0xCB, 0x10]);
test!(sla_c_, [SLA(ByteRegister(C))], [0xCB, 0x21]);
test!(sla_d_, [SLA(ByteRegister(D))], [0xCB, 0x22]);
test!(sla_de_, [SLA(WordRegister(DE))], [0xCB, 0x23, 0xCB, 0x12]);
test!(sla_e_, [SLA(ByteRegister(E))], [0xCB, 0x23]);
test!(sla_h_, [SLA(ByteRegister(H))], [0xCB, 0x24]);
test!(sla_hl_, [SLA(WordRegister(HL))], [0x29]);
test!(sla_l_, [SLA(ByteRegister(L))], [0xCB, 0x25]);
test!(sla__hl__, [SLA(AddressRegister(HL))], [0xCB, 0x26]);
test!(sla__ix__, [SLA(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x26]);
test!(sla__ix_DIS__, [SLA(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x26], DIS: i8);
test!(sla__iy__, [SLA(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x26]);
test!(sla__iy_DIS__, [SLA(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x26], DIS: i8);
test_ub!(sll_a_, [SLL(ByteRegister(A))], [0xCB, 0x37]);
test_ub!(sll_b_, [SLL(ByteRegister(B))], [0xCB, 0x30]);
test_ub!(sll_bc_, [SLL(WordRegister(BC))], [0xCB, 0x31, 0xCB, 0x10]);
test_ub!(sll_c_, [SLL(ByteRegister(C))], [0xCB, 0x31]);
test_ub!(sll_d_, [SLL(ByteRegister(D))], [0xCB, 0x32]);
test_ub!(sll_de_, [SLL(WordRegister(DE))], [0xCB, 0x33, 0xCB, 0x12]);
test_ub!(sll_e_, [SLL(ByteRegister(E))], [0xCB, 0x33]);
test_ub!(sll_h_, [SLL(ByteRegister(H))], [0xCB, 0x34]);
test_ub!(sll_hl_, [SLL(WordRegister(HL))], [0xCB, 0x35, 0xCB, 0x14]);
test_ub!(sll_l_, [SLL(ByteRegister(L))], [0xCB, 0x35]);
test_ub!(sll__hl__, [SLL(AddressRegister(HL))], [0xCB, 0x36]);
test_ub!(sll__ix__, [SLL(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x36]);
test_ub!(sll__ix_DIS__, [SLL(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x36], DIS: i8);
test_ub!(sll__iy__, [SLL(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x36]);
test_ub!(sll__iy_DIS__, [SLL(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x36], DIS: i8);
test!(sra_a_, [SRA(ByteRegister(A))], [0xCB, 0x2F]);
test!(sra_b_, [SRA(ByteRegister(B))], [0xCB, 0x28]);
test!(sra_bc_, [SRA(WordRegister(BC))], [0xCB, 0x28, 0xCB, 0x19]);
test!(sra_c_, [SRA(ByteRegister(C))], [0xCB, 0x29]);
test!(sra_d_, [SRA(ByteRegister(D))], [0xCB, 0x2A]);
test!(sra_de_, [SRA(WordRegister(DE))], [0xCB, 0x2A, 0xCB, 0x1B]);
test!(sra_e_, [SRA(ByteRegister(E))], [0xCB, 0x2B]);
test!(sra_h_, [SRA(ByteRegister(H))], [0xCB, 0x2C]);
test!(sra_hl_, [SRA(WordRegister(HL))], [0xCB, 0x2C, 0xCB, 0x1D]);
test!(sra_l_, [SRA(ByteRegister(L))], [0xCB, 0x2D]);
test!(sra__hl__, [SRA(AddressRegister(HL))], [0xCB, 0x2E]);
test!(sra__ix__, [SRA(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x2E]);
test!(sra__ix_DIS__, [SRA(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x2E], DIS: i8);
test!(sra__iy__, [SRA(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x2E]);
test!(sra__iy_DIS__, [SRA(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x2E], DIS: i8);
test!(srl_a_, [SRL(ByteRegister(A))], [0xCB, 0x3F]);
test!(srl_b_, [SRL(ByteRegister(B))], [0xCB, 0x38]);
test!(srl_c_, [SRL(ByteRegister(C))], [0xCB, 0x39]);
test!(srl_d_, [SRL(ByteRegister(D))], [0xCB, 0x3A]);
test!(srl_e_, [SRL(ByteRegister(E))], [0xCB, 0x3B]);
test!(srl_h_, [SRL(ByteRegister(H))], [0xCB, 0x3C]);
test!(srl_bc_, [SRL(WordRegister(BC))], [0xCB, 0x38, 0xCB, 0x19]);
test!(srl_de_, [SRL(WordRegister(DE))], [0xCB, 0x3A, 0xCB, 0x1B]);
test!(srl_hl_, [SRL(WordRegister(HL))], [0xCB, 0x3C, 0xCB, 0x1D]);
test!(srl__hl__, [SRL(AddressRegister(HL))], [0xCB, 0x3E]);
test!(srl__ix__, [SRL(AddressRegister(IX))], [0xDD, 0xCB, 0x00, 0x3E]);
test!(srl__ix_DIS__, [SRL(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xCB, DIS, 0x3E], DIS: i8);
test!(srl__iy__, [SRL(AddressRegister(IY))], [0xFD, 0xCB, 0x00, 0x3E]);
test!(srl__iy_DIS__, [SRL(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xCB, DIS, 0x3E], DIS: i8);
test!(srl_l_, [SRL(ByteRegister(L))], [0xCB, 0x3D]);
test!(sub_a_, [SUB(ByteRegister(A))], [0x97]);
test!(sub_b_, [SUB(ByteRegister(B))], [0x90]);
test!(sub_c_, [SUB(ByteRegister(C))], [0x91]);
test!(sub_d_, [SUB(ByteRegister(D))], [0x92]);
test!(sub_e_, [SUB(ByteRegister(E))], [0x93]);
test!(sub_h_, [SUB(ByteRegister(H))], [0x94]);
test!(sub__hl__, [SUB(AddressRegister(HL))], [0x96]);
test!(sub__ix__, [SUB(AddressRegister(IX))], [0xDD, 0x96, 0x00]);
test!(sub__ix_DIS__, [SUB(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x96, DIS], DIS: i8);
test!(sub__iy__, [SUB(AddressRegister(IY))], [0xFD, 0x96, 0x00]);
test!(sub__iy_DIS__, [SUB(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x96, DIS], DIS: i8);
test_ub!(sub_ixh_, [SUB(UndocumentedRegister(IXH))], [0xDD, 0x94]);
test_ub!(sub_ixl_, [SUB(UndocumentedRegister(IXL))], [0xDD, 0x95]);
test_ub!(sub_iyh_, [SUB(UndocumentedRegister(IYH))], [0xFD, 0x94]);
test_ub!(sub_iyl_, [SUB(UndocumentedRegister(IYL))], [0xFD, 0x95]);
test!(sub_l_, [SUB(ByteRegister(L))], [0x95]);
test!(xor_a_, [XOR(ByteRegister(A))], [0xAF]);
test!(xor_b_, [XOR(ByteRegister(B))], [0xA8]);
test!(xor_c_, [XOR(ByteRegister(C))], [0xA9]);
test!(xor_d_, [XOR(ByteRegister(D))], [0xAA]);
test!(xor_e_, [XOR(ByteRegister(E))], [0xAB]);
test!(xor_h_, [XOR(ByteRegister(H))], [0xAC]);
test!(xor__hl__, [XOR(AddressRegister(HL))], [0xAE]);
test!(xor__ix__, [XOR(AddressRegister(IX))], [0xDD, 0xAE, 0x00]);
test!(xor__ix_DIS__, [XOR(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xAE, DIS], DIS: i8);
test!(xor__iy__, [XOR(AddressRegister(IY))], [0xFD, 0xAE, 0x00]);
test!(xor__iy_DIS__, [XOR(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xAE, DIS], DIS: i8);
test_ub!(xor_ixh_, [XOR(UndocumentedRegister(IXH))], [0xDD, 0xAC]);
test_ub!(xor_ixl_, [XOR(UndocumentedRegister(IXL))], [0xDD, 0xAD]);
test_ub!(xor_iyh_, [XOR(UndocumentedRegister(IYH))], [0xFD, 0xAC]);
test_ub!(xor_iyl_, [XOR(UndocumentedRegister(IYL))], [0xFD, 0xAD]);
test!(xor_l_, [XOR(ByteRegister(L))], [0xAD]);
test!(ld__hl__N_, [LD(AddressRegister(HL),Constant(n as i32))], [0x36, n], n: u8);
test!(ld__ix__N_, [LD(AddressRegister(IX),Constant(n as i32))], [0xDD, 0x36, 0x00, n], n: u8);
test!(ld__iy__N_, [LD(AddressRegister(IY),Constant(n as i32))], [0xFD, 0x36, 0x00, n], n: u8);
test!(ld_a__bc__, [LD(ByteRegister(A),AddressRegister(BC))], [0x0A]);
test!(inc__ix_DIS__, [INC(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x34, DIS], DIS: i8);
test!(inc__iy_DIS__, [INC(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x34, DIS], DIS: i8);
test!(in_a__N__, [IN(ByteRegister(A),Port(n))], [0xDB, n], n: u8);
test_ub!(in_f__c__, [IN(F,PortRegister(C))], [0xED, 0x70]);
test!(jp_NN_, [JP(None, Constant(NN as i32))], [0xC3, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_c_NN_, [JP(Some(Condition::C),Constant(NN as i32))], [0xDA, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_m_NN_, [JP(Some(Condition::M),Constant(NN as i32))], [0xFA, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_nc_NN_, [JP(Some(Condition::NC),Constant(NN as i32))], [0xD2, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_nz_NN_, [JP(Some(Condition::NZ),Constant(NN as i32))], [0xC2, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_p_NN_, [JP(Some(Condition::P),Constant(NN as i32))], [0xF2, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_pe_NN_, [JP(Some(Condition::PE),Constant(NN as i32))], [0xEA, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_po_NN_, [JP(Some(Condition::PO),Constant(NN as i32))], [0xE2, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(jp_z_NN_, [JP(Some(Condition::Z),Constant(NN as i32))], [0xCA, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(djnz_NN_, [DJNZ(d)], [0x10, d], d: i8);
test!(jr_d_, [JR(None, Constant(d as i32))], [0x18, d], d: i8);
test!(jr_c_d_, [JR(Some(Condition::C),Constant(d as i32))], [0x38, d], d: i8);
test!(jr_nc_d_, [JR(Some(Condition::NC),Constant(d as i32))], [0x30, d], d: i8);
test!(jr_z_d_, [JR(Some(Condition::Z),Constant(d as i32))], [0x28, d], d: i8);
test!(jr_nz_d_, [JR(Some(Condition::NZ),Constant(d as i32))], [0x20, d], d: i8);
test!(ld__NN__a_, [LD(Address(NN),ByteRegister(A))], [0x32, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__bc_, [LD(Address(NN),WordRegister(BC))], [0xED, 0x43, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__de_, [LD(Address(NN),WordRegister(DE))], [0xED, 0x53, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__hl_, [LD(Address(NN),WordRegister(HL))], [0x22, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__ix_, [LD(Address(NN),WordRegister(IX))], [0xDD, 0x22, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__iy_, [LD(Address(NN),WordRegister(IY))], [0xFD, 0x22, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__NN__sp_, [LD(Address(NN),WordRegister(SP))], [0xED, 0x73, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld__ix_DIS__N_, [LD(AddressRegisterWithOffset(IX, DIS),Constant(n as i32))], [0xDD, 0x36, DIS, n], DIS: i8, n: u8);
test!(ld__ix_DIS__a_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0x77, DIS], DIS: i8);
test!(ld__ix_DIS__b_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0x70, DIS], DIS: i8);
test!(ld__ix_DIS__bc_, [LD(AddressRegisterWithOffset(IX, DIS),WordRegister(BC))], [0xDD, 0x71, DIS, 0xDD, 0x70, DIS+1], DIS: i8);
test!(ld__ix_DIS__c_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0x71, DIS], DIS: i8);
test!(ld__ix_DIS__d_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0x72, DIS], DIS: i8);
test!(ld__ix_DIS__de_, [LD(AddressRegisterWithOffset(IX, DIS),WordRegister(DE))], [0xDD, 0x73, DIS, 0xDD, 0x72, DIS+1], DIS: i8);
test!(ld__ix_DIS__e_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0x73, DIS], DIS: i8);
test!(ld__ix_DIS__h_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0x74, DIS], DIS: i8);
test!(ld__ix_DIS__hl_, [LD(AddressRegisterWithOffset(IX, DIS),WordRegister(HL))], [0xDD, 0x75, DIS, 0xDD, 0x74, DIS+1], DIS: i8);
test!(ld__ix_DIS__l_, [LD(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0x75, DIS], DIS: i8);
test!(ld__iy_DIS__N_, [LD(AddressRegisterWithOffset(IY, DIS),Constant(n as i32))], [0xFD, 0x36, DIS, n], DIS: i8, n: u8);
test!(ld__iy_DIS__a_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0x77, DIS], DIS: i8);
test!(ld__iy_DIS__b_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0x70, DIS], DIS: i8);
test!(ld__iy_DIS__bc_, [LD(AddressRegisterWithOffset(IY, DIS),WordRegister(BC))], [0xFD, 0x71, DIS, 0xFD, 0x70, DIS+1], DIS: i8);
test!(ld__iy_DIS__c_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0x71, DIS], DIS: i8);
test!(ld__iy_DIS__d_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0x72, DIS], DIS: i8);
test!(ld__iy_DIS__de_, [LD(AddressRegisterWithOffset(IY, DIS),WordRegister(DE))], [0xFD, 0x73, DIS, 0xFD, 0x72, DIS+1], DIS: i8);
test!(ld__iy_DIS__e_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0x73, DIS], DIS: i8);
test!(ld__iy_DIS__h_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0x74, DIS], DIS: i8);
test!(ld__iy_DIS__hl_, [LD(AddressRegisterWithOffset(IY, DIS),WordRegister(HL))], [0xFD, 0x75, DIS, 0xFD, 0x74, DIS+1], DIS: i8);
test!(ld__iy_DIS__l_, [LD(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0x75, DIS], DIS: i8);
test!(ld_a__ix_DIS__, [LD(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x7E, DIS], DIS: i8);
test!(ld_a__iy_DIS__, [LD(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x7E, DIS], DIS: i8);
test!(ld_a_N_, [LD(ByteRegister(A),Constant(n as i32))], [0x3E, n], n: u8);
test!(ld_b__ix_DIS__, [LD(ByteRegister(B),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x46, DIS], DIS: i8);
test!(ld_b__iy_DIS__, [LD(ByteRegister(B),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x46, DIS], DIS: i8);
test!(ld_b_N_, [LD(ByteRegister(B),Constant(n as i32))], [0x06, n], n: u8);
test!(ld_bc__NN__, [LD(WordRegister(BC),Address(NN))], [0xED, 0x4B, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_bc__ix_DIS__, [LD(WordRegister(BC),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x4E, DIS, 0xDD, 0x46, DIS+1], DIS: i8);
test!(ld_bc__iy_DIS__, [LD(WordRegister(BC),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x4E, DIS, 0xFD, 0x46, DIS+1], DIS: i8);
test!(ld_bc_NN_, [LD(WordRegister(BC),Constant(NN as i32))], [0x01, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_c__ix_DIS__, [LD(ByteRegister(C),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x4E, DIS], DIS: i8);
test!(ld_c__iy_DIS__, [LD(ByteRegister(C),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x4E, DIS], DIS: i8);
test!(ld_c_N_, [LD(ByteRegister(C),Constant(n as i32))], [0x0E, n], n: u8);
test!(ld_d__ix_DIS__, [LD(ByteRegister(D),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x56, DIS], DIS: i8);
test!(ld_d__iy_DIS__, [LD(ByteRegister(D),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x56, DIS], DIS: i8);
test!(ld_d_N_, [LD(ByteRegister(D),Constant(n as i32))], [0x16, n], n: u8);
test!(ld_de__NN__, [LD(WordRegister(DE),Address(NN))], [0xED, 0x5B, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_de__hl__, [LD(WordRegister(DE),AddressRegister(HL))], [0x5E, 0x23, 0x56, 0x2B]);
test!(ld_de__ix__, [LD(WordRegister(DE),AddressRegister(IX))], [0xDD, 0x5E, 0x00, 0xDD, 0x56, 0x01]);
test!(ld_de__ix_DIS__, [LD(WordRegister(DE),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x5E, DIS, 0xDD, 0x56, DIS+1], DIS: i8);
test!(ld_de__iy__, [LD(WordRegister(DE),AddressRegister(IY))], [0xFD, 0x5E, 0x00, 0xFD, 0x56, 0x01]);
test!(ld_de__iy_DIS__, [LD(WordRegister(DE),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x5E, DIS, 0xFD, 0x56, DIS+1], DIS: i8);
test!(ld_de_NN_, [LD(WordRegister(DE),Constant(NN as i32))], [0x11, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_de_bc_, [LD(WordRegister(DE),WordRegister(BC))], [0x50, 0x59]);
test!(ld_de_de_, [LD(WordRegister(DE),WordRegister(DE))], [0x52, 0x5B]);
test!(ld_de_hl_, [LD(WordRegister(DE),WordRegister(HL))], [0x54, 0x5D]);
test!(ld_de_ix_, [LD(WordRegister(DE),WordRegister(IX))], [0xDD, 0x54, 0xDD, 0x5D]);
test!(ld_de_iy_, [LD(WordRegister(DE),WordRegister(IY))], [0xFD, 0x54, 0xFD, 0x5D]);
test!(ld_e__ix_DIS__, [LD(ByteRegister(E),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x5E, DIS], DIS: i8);
test!(ld_e__iy__, [LD(ByteRegister(E),AddressRegister(IY))], [0xFD, 0x5E, 0x00]);
test!(ld_e__iy_DIS__, [LD(ByteRegister(E),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x5E, DIS], DIS: i8);
test!(ld_e_N_, [LD(ByteRegister(E),Constant(n as i32))], [0x1E, n], n: u8);
test!(ld_h__ix_DIS__, [LD(ByteRegister(H),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x66, DIS], DIS: i8);
test!(ld_h__iy_DIS__, [LD(ByteRegister(H),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x66, DIS], DIS: i8);
test!(ld_h_N_, [LD(ByteRegister(H),Constant(n as i32))], [0x26, n], n: u8);
test!(ld_hl__NN__, [LD(WordRegister(HL),Address(NN))], [0x2A, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_hl__ix_DIS__, [LD(WordRegister(HL),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x6E, DIS, 0xDD, 0x66, DIS+1], DIS: i8);
test!(ld_hl__iy_DIS__, [LD(WordRegister(HL),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x6E, DIS, 0xFD, 0x66, DIS+1], DIS: i8);
test!(ld_hl_NN_, [LD(WordRegister(HL),Constant(NN as i32))], [0x21, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_ix__NN__, [LD(WordRegister(IX),Address(NN))], [0xDD, 0x2A, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_ix_NN_, [LD(WordRegister(IX),Constant(NN as i32))], [0xDD, 0x21, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test_ub!(ld_ixh_N_, [LD(UndocumentedRegister(IXH),Constant(n as i32))], [0xDD, 0x26, n], n: u8);
test_ub!(ld_ixl_N_, [LD(UndocumentedRegister(IXL),Constant(n as i32))], [0xDD, 0x2E, n], n: u8);
test!(ld_iy__NN__, [LD(WordRegister(IY),Address(NN))], [0xFD, 0x2A, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_iy_NN_, [LD(WordRegister(IY),Constant(NN as i32))], [0xFD, 0x21, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test_ub!(ld_iyh_N_, [LD(UndocumentedRegister(IYH),Constant(n as i32))], [0xFD, 0x26, n], n: u8);
test_ub!(ld_iyl_N_, [LD(UndocumentedRegister(IYL),Constant(n as i32))], [0xFD, 0x2E, n], n: u8);
test!(ld_l__ix_DIS__, [LD(ByteRegister(L),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x6E, DIS], DIS: i8);
test!(ld_l__iy_DIS__, [LD(ByteRegister(L),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x6E, DIS], DIS: i8);
test!(ld_l_N_, [LD(ByteRegister(L),Constant(n as i32))], [0x2E, n], n: u8);
test!(ld_sp__NN__, [LD(WordRegister(SP),Address(NN))], [0xED, 0x7B, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
test!(ld_sp_NN_, [LD(WordRegister(SP),Constant(NN as i32))], [0x31, (NN & 0xFF), ((NN >> 8) & 0xFF)], NN: u16);
//test!(ldd__bc__a_, [LDD(AddressRegister(BC),ByteRegister(A))], [0x02, 0x0B]);
//test!(ldd__de__a_, [LDD(AddressRegister(DE),ByteRegister(A))], [0x12, 0x1B]);
//test!(ldd__hl__N_, [LDD(AddressRegister(HL),Constant(n as i32))], [0x36, n, 0x2B], n: u8);
//test!(ldd__hl__a_, [LDD(AddressRegister(HL),ByteRegister(A))], [0x77, 0x2B]);
//test!(ldd__hl__b_, [LDD(AddressRegister(HL),ByteRegister(B))], [0x70, 0x2B]);
//test!(ldd__hl__c_, [LDD(AddressRegister(HL),ByteRegister(C))], [0x71, 0x2B]);
//test!(ldd__hl__d_, [LDD(AddressRegister(HL),ByteRegister(D))], [0x72, 0x2B]);
//test!(ldd__hl__e_, [LDD(AddressRegister(HL),ByteRegister(E))], [0x73, 0x2B]);
//test!(ldd__hl__h_, [LDD(AddressRegister(HL),ByteRegister(H))], [0x74, 0x2B]);
//test!(ldd__hl__l_, [LDD(AddressRegister(HL),ByteRegister(L))], [0x75, 0x2B]);
//test!(ldd__ix__N_, [LDD(AddressRegister(IX),n)], [0xDD, 0x36, 0x00, n, 0xDD, 0x2B], n: u8);
//test!(ldd__ix__a_, [LDD(AddressRegister(IX),ByteRegister(A))], [0xDD, 0x77, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__b_, [LDD(AddressRegister(IX),ByteRegister(B))], [0xDD, 0x70, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__c_, [LDD(AddressRegister(IX),ByteRegister(C))], [0xDD, 0x71, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__d_, [LDD(AddressRegister(IX),ByteRegister(D))], [0xDD, 0x72, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__e_, [LDD(AddressRegister(IX),ByteRegister(E))], [0xDD, 0x73, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__h_, [LDD(AddressRegister(IX),ByteRegister(H))], [0xDD, 0x74, 0x00, 0xDD, 0x2B]);
//test!(ldd__ix__l_, [LDD(AddressRegister(IX),ByteRegister(L))], [0xDD, 0x75, 0x00, 0xDD, 0x2B]);
//test!(ldd__iy__a_, [LDD(AddressRegister(IY),ByteRegister(A))], [0xFD, 0x77, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__b_, [LDD(AddressRegister(IY),ByteRegister(B))], [0xFD, 0x70, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__c_, [LDD(AddressRegister(IY),ByteRegister(C))], [0xFD, 0x71, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__d_, [LDD(AddressRegister(IY),ByteRegister(D))], [0xFD, 0x72, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__e_, [LDD(AddressRegister(IY),ByteRegister(E))], [0xFD, 0x73, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__h_, [LDD(AddressRegister(IY),ByteRegister(H))], [0xFD, 0x74, 0x00, 0xFD, 0x2B]);
//test!(ldd__iy__l_, [LDD(AddressRegister(IY),ByteRegister(L))], [0xFD, 0x75, 0x00, 0xFD, 0x2B]);
//test!(ldd__ix_DIS__N_, [LDD(AddressRegisterWithOffset(IX, DIS),Constant(n as i32))], [0xDD, 0x36, DIS, n, 0xDD, 0x2B], DIS: i8, n: u8);
//test!(ldd__ix_DIS__a_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0x77, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__b_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0x70, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__c_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0x71, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__d_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0x72, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__e_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0x73, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__h_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0x74, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__ix_DIS__l_, [LDD(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0x75, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd__iy__N_, [LDD(AddressRegister(IY),Constant(n as i32))], [0xFD, 0x36, 0x00, n, 0xFD, 0x2B], n: u8);
//test!(ldd__iy_DIS__N_, [LDD(AddressRegisterWithOffset(IY, DIS),Constant(n as i32))], [0xFD, 0x36, DIS, n, 0xFD, 0x2B], DIS: i8, n: u8);
//test!(ldd__iy_DIS__a_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0x77, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__b_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0x70, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__c_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0x71, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__d_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0x72, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__e_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0x73, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__h_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0x74, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd__iy_DIS__l_, [LDD(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0x75, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_a__bc__, [LDD(ByteRegister(A),AddressRegister(BC))], [0x0A, 0x0B]);
//test!(ldd_a__de__, [LDD(ByteRegister(A),AddressRegister(DE))], [0x1A, 0x1B]);
//test!(ldd_a__hl__, [LDD(ByteRegister(A),AddressRegister(HL))], [0x7E, 0x2B]);
//test!(ldd_a__ix__, [LDD(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x7E, 0x00, 0xDD, 0x2B]);
//test!(ldd_a__ix_DIS__, [LDD(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x7E, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_a__iy__, [LDD(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x7E, 0x00, 0xFD, 0x2B]);
//test!(ldd_a__iy_DIS__, [LDD(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x7E, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_b__hl__, [LDD(ByteRegister(B),AddressRegister(HL))], [0x46, 0x2B]);
//test!(ldd_b__ix__, [LDD(ByteRegister(B),AddressRegister(IX))], [0xDD, 0x46, 0x00, 0xDD, 0x2B]);
//test!(ldd_b__ix_DIS__, [LDD(ByteRegister(B),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x46, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_b__iy__, [LDD(ByteRegister(B),AddressRegister(IY))], [0xFD, 0x46, 0x00, 0xFD, 0x2B]);
//test!(ldd_b__iy_DIS__, [LDD(ByteRegister(B),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x46, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_c__hl__, [LDD(ByteRegister(C),AddressRegister(HL))], [0x4E, 0x2B]);
//test!(ldd_c__ix__, [LDD(ByteRegister(C),AddressRegister(IX))], [0xDD, 0x4E, 0x00, 0xDD, 0x2B]);
//test!(ldd_c__ix_DIS__, [LDD(ByteRegister(C),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x4E, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_c__iy__, [LDD(ByteRegister(C),AddressRegister(IY))], [0xFD, 0x4E, 0x00, 0xFD, 0x2B]);
//test!(ldd_c__iy_DIS__, [LDD(ByteRegister(C),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x4E, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_d__hl__, [LDD(ByteRegister(D),AddressRegister(HL))], [0x56, 0x2B]);
//test!(ldd_d__ix__, [LDD(ByteRegister(D),AddressRegister(IX))], [0xDD, 0x56, 0x00, 0xDD, 0x2B]);
//test!(ldd_d__ix_DIS__, [LDD(ByteRegister(D),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x56, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_d__iy__, [LDD(ByteRegister(D),AddressRegister(IY))], [0xFD, 0x56, 0x00, 0xFD, 0x2B]);
//test!(ldd_d__iy_DIS__, [LDD(ByteRegister(D),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x56, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_e__hl__, [LDD(ByteRegister(E),AddressRegister(HL))], [0x5E, 0x2B]);
//test!(ldd_e__ix__, [LDD(ByteRegister(E),AddressRegister(IX))], [0xDD, 0x5E, 0x00, 0xDD, 0x2B]);
//test!(ldd_e__ix_DIS__, [LDD(ByteRegister(E),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x5E, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_e__iy__, [LDD(ByteRegister(E),AddressRegister(IY))], [0xFD, 0x5E, 0x00, 0xFD, 0x2B]);
//test!(ldd_e__iy_DIS__, [LDD(ByteRegister(E),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x5E, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_h__hl__, [LDD(ByteRegister(H),AddressRegister(HL))], [0x66, 0x2B]);
//test!(ldd_h__ix__, [LDD(ByteRegister(H),AddressRegister(IX))], [0xDD, 0x66, 0x00, 0xDD, 0x2B]);
//test!(ldd_h__ix_DIS__, [LDD(ByteRegister(H),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x66, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_h__iy__, [LDD(ByteRegister(H),AddressRegister(IY))], [0xFD, 0x66, 0x00, 0xFD, 0x2B]);
//test!(ldd_h__iy_DIS__, [LDD(ByteRegister(H),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x66, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldd_l__hl__, [LDD(ByteRegister(L),AddressRegister(HL))], [0x6E, 0x2B]);
//test!(ldd_l__ix__, [LDD(ByteRegister(L),AddressRegister(IX))], [0xDD, 0x6E, 0x00, 0xDD, 0x2B]);
//test!(ldd_l__ix_DIS__, [LDD(ByteRegister(L),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x6E, DIS, 0xDD, 0x2B], DIS: i8);
//test!(ldd_l__iy__, [LDD(ByteRegister(L),AddressRegister(IY))], [0xFD, 0x6E, 0x00, 0xFD, 0x2B]);
//test!(ldd_l__iy_DIS__, [LDD(ByteRegister(L),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x6E, DIS, 0xFD, 0x2B], DIS: i8);
//test!(ldi__bc__a_, [LDI(AddressRegister(BC),ByteRegister(A))], [0x02, 0x03]);
//test!(ldi__de__a_, [LDI(AddressRegister(DE),ByteRegister(A))], [0x12, 0x13]);
//test!(ldi__hl__N_, [LDI(AddressRegister(HL),Constant(n as i32))], [0x36, n, 0x23], n: u8);
//test!(ldi__hl__a_, [LDI(AddressRegister(HL),ByteRegister(A))], [0x77, 0x23]);
//test!(ldi__hl__b_, [LDI(AddressRegister(HL),ByteRegister(B))], [0x70, 0x23]);
//test!(ldi__hl__bc_, [LDI(AddressRegister(HL),WordRegister(BC))], [0x71, 0x23, 0x70, 0x23]);
//test!(ldi__hl__c_, [LDI(AddressRegister(HL),ByteRegister(C))], [0x71, 0x23]);
//test!(ldi__hl__d_, [LDI(AddressRegister(HL),ByteRegister(D))], [0x72, 0x23]);
//test!(ldi__hl__de_, [LDI(AddressRegister(HL),WordRegister(DE))], [0x73, 0x23, 0x72, 0x23]);
//test!(ldi__hl__e_, [LDI(AddressRegister(HL),ByteRegister(E))], [0x73, 0x23]);
//test!(ldi__hl__h_, [LDI(AddressRegister(HL),ByteRegister(H))], [0x74, 0x23]);
//test!(ldi__hl__l_, [LDI(AddressRegister(HL),ByteRegister(L))], [0x75, 0x23]);
//test!(ldi__ix__N_, [LDI(AddressRegister(IX),Constant(n as i32))], [0xDD, 0x36, 0x00, n, 0xDD, 0x23], n: u8);
//test!(ldi__ix__a_, [LDI(AddressRegister(IX),ByteRegister(A))], [0xDD, 0x77, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__b_, [LDI(AddressRegister(IX),ByteRegister(B))], [0xDD, 0x70, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__bc_, [LDI(AddressRegister(IX),WordRegister(BC))], [0xDD, 0x71, 0x00, 0xDD, 0x23, 0xDD, 0x70, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__c_, [LDI(AddressRegister(IX),ByteRegister(C))], [0xDD, 0x71, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__d_, [LDI(AddressRegister(IX),ByteRegister(D))], [0xDD, 0x72, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__de_, [LDI(AddressRegister(IX),WordRegister(DE))], [0xDD, 0x73, 0x00, 0xDD, 0x23, 0xDD, 0x72, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__e_, [LDI(AddressRegister(IX),ByteRegister(E))], [0xDD, 0x73, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__h_, [LDI(AddressRegister(IX),ByteRegister(H))], [0xDD, 0x74, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__hl_, [LDI(AddressRegister(IX),WordRegister(HL))], [0xDD, 0x75, 0x00, 0xDD, 0x23, 0xDD, 0x74, 0x00, 0xDD, 0x23]);
//test!(ldi__ix__l_, [LDI(AddressRegister(IX),ByteRegister(L))], [0xDD, 0x75, 0x00, 0xDD, 0x23]);
//test!(ldi__ix_DIS__N_, [LDI(AddressRegisterWithOffset(IX, DIS),Constant(n as i32))], [0xDD, 0x36, DIS, n, 0xDD, 0x23], DIS: i8, n: u8);
//test!(ldi__ix_DIS__a_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0x77, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__b_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0x70, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__bc_, [LDI(AddressRegisterWithOffset(IX, DIS),WordRegister(BC))], [0xDD, 0x71, DIS, 0xDD, 0x23, 0xDD 0x70 DIS 0xDD 0x23], DIS: i8);
//test!(ldi__ix_DIS__c_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0x71, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__d_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0x72, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__de_, [LDI(AddressRegisterWithOffset(IX, DIS),WordRegister(DE))], [0xDD, 0x73, DIS, 0xDD, 0x23, 0xDD 0x72 DIS 0xDD 0x23], DIS: i8);
//test!(ldi__ix_DIS__e_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0x73, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__h_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0x74, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__ix_DIS__hl_, [LDI(AddressRegisterWithOffset(IX, DIS),WordRegister(HL))], [0xDD, 0x75, DIS, 0xDD, 0x23, 0xDD 0x74 DIS 0xDD 0x23], DIS: i8);
//test!(ldi__ix_DIS__l_, [LDI(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0x75, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi__iy__N_, [LDI(AddressRegister(IY),n)], [0xFD, 0x36, 0x00, n, 0xFD, 0x23], n: u8);
//test!(ldi__iy__a_, [LDI(AddressRegister(IY),ByteRegister(A))], [0xFD, 0x77, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__b_, [LDI(AddressRegister(IY),ByteRegister(B))], [0xFD, 0x70, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__bc_, [LDI(AddressRegister(IY),WordRegister(BC))], [0xFD, 0x71, 0x00, 0xFD, 0x23, 0xFD, 0x70, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__c_, [LDI(AddressRegister(IY),ByteRegister(C))], [0xFD, 0x71, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__d_, [LDI(AddressRegister(IY),ByteRegister(D))], [0xFD, 0x72, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__de_, [LDI(AddressRegister(IY),WordRegister(DE))], [0xFD, 0x73, 0x00, 0xFD, 0x23, 0xFD, 0x72, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__e_, [LDI(AddressRegister(IY),ByteRegister(E))], [0xFD, 0x73, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__h_, [LDI(AddressRegister(IY),ByteRegister(H))], [0xFD, 0x74, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__hl_, [LDI(AddressRegister(IY),WordRegister(HL))], [0xFD, 0x75, 0x00, 0xFD, 0x23, 0xFD, 0x74, 0x00, 0xFD, 0x23]);
//test!(ldi__iy__l_, [LDI(AddressRegister(IY),ByteRegister(L))], [0xFD, 0x75, 0x00, 0xFD, 0x23]);
//test!(ldi__iy_DIS__N_, [LDI(AddressRegisterWithOffset(IY, DIS),n)], [0xFD, 0x36, DIS, n, 0xFD, 0x23], DIS: i8, n: u8);
//test!(ldi__iy_DIS__a_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0x77, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__b_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0x70, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__bc_, [LDI(AddressRegisterWithOffset(IY, DIS),WordRegister(BC))], [0xFD, 0x71, DIS, 0xFD, 0x23, 0xFD 0x70 DIS 0xFD 0x23], DIS: i8);
//test!(ldi__iy_DIS__c_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0x71, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__d_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0x72, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__de_, [LDI(AddressRegisterWithOffset(IY, DIS),WordRegister(DE))], [0xFD, 0x73, DIS, 0xFD, 0x23, 0xFD 0x72 DIS 0xFD 0x23], DIS: i8);
//test!(ldi__iy_DIS__e_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0x73, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__h_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0x74, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi__iy_DIS__hl_, [LDI(AddressRegisterWithOffset(IY, DIS),WordRegister(HL))], [0xFD, 0x75, DIS, 0xFD, 0x23, 0xFD 0x74 DIS 0xFD 0x23], DIS: i8);
//test!(ldi__iy_DIS__l_, [LDI(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0x75, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_a__bc__, [LDI(ByteRegister(A),AddressRegister(BC))], [0x0A, 0x03]);
//test!(ldi_a__de__, [LDI(ByteRegister(A),AddressRegister(DE))], [0x1A, 0x13]);
//test!(ldi_a__hl__, [LDI(ByteRegister(A),AddressRegister(HL))], [0x7E, 0x23]);
//test!(ldi_a__ix__, [LDI(ByteRegister(A),AddressRegister(IX))], [0xDD, 0x7E, 0x00, 0xDD, 0x23]);
//test!(ldi_a__ix_DIS__, [LDI(ByteRegister(A),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x7E, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_a__iy__, [LDI(ByteRegister(A),AddressRegister(IY))], [0xFD, 0x7E, 0x00, 0xFD, 0x23]);
//test!(ldi_a__iy_DIS__, [LDI(ByteRegister(A),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x7E, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_b__hl__, [LDI(ByteRegister(B),AddressRegister(HL))], [0x46, 0x23]);
//test!(ldi_b__ix__, [LDI(ByteRegister(B),AddressRegister(IX))], [0xDD, 0x46, 0x00, 0xDD, 0x23]);
//test!(ldi_b__ix_DIS__, [LDI(ByteRegister(B),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x46, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_b__iy__, [LDI(ByteRegister(B),AddressRegister(IY))], [0xFD, 0x46, 0x00, 0xFD, 0x23]);
//test!(ldi_b__iy_DIS__, [LDI(ByteRegister(B),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x46, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_bc__hl__, [LDI(WordRegister(BC),AddressRegister(HL))], [0x4E, 0x23, 0x46, 0x23]);
//test!(ldi_bc__ix__, [LDI(WordRegister(BC),AddressRegister(IX))], [0xDD, 0x4E, 0x00, 0xDD, 0x23, 0xDD, 0x46, 0x00, 0xDD, 0x23]);
//test!(ldi_bc__ix_DIS__, [LDI(WordRegister(BC),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x4E, DIS, 0xDD, 0x23, 0xDD 0x46 DIS 0xDD 0x23], DIS: i8);
//test!(ldi_bc__iy__, [LDI(WordRegister(BC),AddressRegister(IY))], [0xFD, 0x4E, 0x00, 0xFD, 0x23, 0xFD, 0x46, 0x00, 0xFD, 0x23]);
//test!(ldi_bc__iy_DIS__, [LDI(WordRegister(BC),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x4E, DIS, 0xFD, 0x23, 0xFD 0x46 DIS 0xFD 0x23], DIS: i8);
//test!(ldi_c__hl__, [LDI(ByteRegister(C),AddressRegister(HL))], [0x4E, 0x23]);
//test!(ldi_c__ix__, [LDI(ByteRegister(C),AddressRegister(IX))], [0xDD, 0x4E, 0x00, 0xDD, 0x23]);
//test!(ldi_c__ix_DIS__, [LDI(ByteRegister(C),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x4E, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_c__iy__, [LDI(ByteRegister(C),AddressRegister(IY))], [0xFD, 0x4E, 0x00, 0xFD, 0x23]);
//test!(ldi_c__iy_DIS__, [LDI(ByteRegister(C),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x4E, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_d__hl__, [LDI(ByteRegister(D),AddressRegister(HL))], [0x56, 0x23]);
//test!(ldi_d__ix__, [LDI(ByteRegister(D),AddressRegister(IX))], [0xDD, 0x56, 0x00, 0xDD, 0x23]);
//test!(ldi_d__ix_DIS__, [LDI(ByteRegister(D),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x56, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_d__iy__, [LDI(ByteRegister(D),AddressRegister(IY))], [0xFD, 0x56, 0x00, 0xFD, 0x23]);
//test!(ldi_d__iy_DIS__, [LDI(ByteRegister(D),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x56, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_de__hl__, [LDI(WordRegister(DE),AddressRegister(HL))], [0x5E, 0x23, 0x56, 0x23]);
//test!(ldi_de__ix__, [LDI(WordRegister(DE),AddressRegister(IX))], [0xDD, 0x5E, 0x00, 0xDD, 0x23, 0xDD, 0x56, 0x00, 0xDD, 0x23]);
//test!(ldi_de__ix_DIS__, [LDI(WordRegister(DE),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x5E, DIS, 0xDD, 0x23, 0xDD 0x56 DIS 0xDD 0x23], DIS: i8);
//test!(ldi_de__iy__, [LDI(WordRegister(DE),AddressRegister(IY))], [0xFD, 0x5E, 0x00, 0xFD, 0x23, 0xFD, 0x56, 0x00, 0xFD, 0x23]);
//test!(ldi_de__iy_DIS__, [LDI(WordRegister(DE),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x5E, DIS, 0xFD, 0x23, 0xFD 0x56 DIS 0xFD 0x23], DIS: i8);
//test!(ldi_e__hl__, [LDI(ByteRegister(E),AddressRegister(HL))], [0x5E, 0x23]);
//test!(ldi_e__ix__, [LDI(ByteRegister(E),AddressRegister(IX))], [0xDD, 0x5E, 0x00, 0xDD, 0x23]);
//test!(ldi_e__ix_DIS__, [LDI(ByteRegister(E),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x5E, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_e__iy__, [LDI(ByteRegister(E),AddressRegister(IY))], [0xFD, 0x5E, 0x00, 0xFD, 0x23]);
//test!(ldi_e__iy_DIS__, [LDI(ByteRegister(E),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x5E, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_h__hl__, [LDI(ByteRegister(H),AddressRegister(HL))], [0x66, 0x23]);
//test!(ldi_h__ix__, [LDI(ByteRegister(H),AddressRegister(IX))], [0xDD, 0x66, 0x00, 0xDD, 0x23]);
//test!(ldi_h__ix_DIS__, [LDI(ByteRegister(H),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x66, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_h__iy__, [LDI(ByteRegister(H),AddressRegister(IY))], [0xFD, 0x66, 0x00, 0xFD, 0x23]);
//test!(ldi_h__iy_DIS__, [LDI(ByteRegister(H),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x66, DIS, 0xFD, 0x23], DIS: i8);
//test!(ldi_hl__ix__, [LDI(WordRegister(HL),AddressRegister(IX))], [0xDD, 0x6E, 0x00, 0xDD, 0x23, 0xDD, 0x66, 0x00, 0xDD, 0x23]);
//test!(ldi_hl__ix_DIS__, [LDI(WordRegister(HL),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x6E, DIS, 0xDD, 0x23, 0xDD 0x66 DIS 0xDD 0x23], DIS: i8);
//test!(ldi_hl__iy__, [LDI(WordRegister(HL),AddressRegister(IY))], [0xFD, 0x6E, 0x00, 0xFD, 0x23, 0xFD, 0x66, 0x00, 0xFD, 0x23]);
//test!(ldi_hl__iy_DIS__, [LDI(WordRegister(HL),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x6E, DIS, 0xFD, 0x23, 0xFD 0x66 DIS 0xFD 0x23], DIS: i8);
//test!(ldi_l__hl__, [LDI(ByteRegister(L),AddressRegister(HL))], [0x6E, 0x23]);
//test!(ldi_l__ix__, [LDI(ByteRegister(L),AddressRegister(IX))], [0xDD, 0x6E, 0x00, 0xDD, 0x23]);
//test!(ldi_l__ix_DIS__, [LDI(ByteRegister(L),AddressRegisterWithOffset(IX, DIS))], [0xDD, 0x6E, DIS, 0xDD, 0x23], DIS: i8);
//test!(ldi_l__iy__, [LDI(ByteRegister(L),AddressRegister(IY))], [0xFD, 0x6E, 0x00, 0xFD, 0x23]);
//test!(ldi_l__iy_DIS__, [LDI(ByteRegister(L),AddressRegisterWithOffset(IY, DIS))], [0xFD, 0x6E, DIS, 0xFD, 0x23], DIS: i8);
test!(or__ix_DIS__, [OR(AddressRegisterWithOffset(IX, DIS))], [0xDD, 0xB6, DIS], DIS: i8);
test!(or__iy_DIS__, [OR(AddressRegisterWithOffset(IY, DIS))], [0xFD, 0xB6, DIS], DIS: i8);
test!(or_N_, [OR(Constant(n as i32))], [0xF6, n], n: u8);
test!(out__N__a_, [OUT(Port(n),ByteRegister(A))], [0xD3, n], n: u8);
//test!(rl__ix__a_, [RL(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x17]);
//test!(rl__ix__b_, [RL(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x10]);
//test!(rl__ix__c_, [RL(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x11]);
//test!(rl__ix__d_, [RL(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x12]);
//test!(rl__ix__e_, [RL(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x13]);
//test!(rl__ix__h_, [RL(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x14]);
//test!(rl__ix__l_, [RL(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x15]);
//test!(rl__ix_DIS__a_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x17], DIS: i8);
//test!(rl__ix_DIS__b_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x10], DIS: i8);
//test!(rl__ix_DIS__c_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x11], DIS: i8);
//test!(rl__ix_DIS__d_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x12], DIS: i8);
//test!(rl__ix_DIS__e_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x13], DIS: i8);
//test!(rl__ix_DIS__h_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x14], DIS: i8);
//test!(rl__ix_DIS__l_, [RL(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x15], DIS: i8);
//test!(rl__iy__a_, [RL(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x17]);
//test!(rl__iy__b_, [RL(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x10]);
//test!(rl__iy__c_, [RL(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x11]);
//test!(rl__iy__d_, [RL(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x12]);
//test!(rl__iy__e_, [RL(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x13]);
//test!(rl__iy__h_, [RL(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x14]);
//test!(rl__iy__l_, [RL(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x15]);
//test!(rl__iy_DIS__a_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x17], DIS: i8);
//test!(rl__iy_DIS__b_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x10], DIS: i8);
//test!(rl__iy_DIS__c_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x11], DIS: i8);
//test!(rl__iy_DIS__d_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x12], DIS: i8);
//test!(rl__iy_DIS__e_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x13], DIS: i8);
//test!(rl__iy_DIS__h_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x14], DIS: i8);
//test!(rl__iy_DIS__l_, [RL(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x15], DIS: i8);
//test!(rlc__ix__a_, [RLC(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x07]);
//test!(rlc__ix__b_, [RLC(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x00]);
//test!(rlc__ix__c_, [RLC(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x01]);
//test!(rlc__ix__d_, [RLC(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x02]);
//test!(rlc__ix__e_, [RLC(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x03]);
//test!(rlc__ix__h_, [RLC(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x04]);
//test!(rlc__ix__l_, [RLC(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x05]);
//test!(rlc__ix_DIS__a_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x07], DIS: i8);
//test!(rlc__ix_DIS__b_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x00], DIS: i8);
//test!(rlc__ix_DIS__c_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x01], DIS: i8);
//test!(rlc__ix_DIS__d_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x02], DIS: i8);
//test!(rlc__ix_DIS__e_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x03], DIS: i8);
//test!(rlc__ix_DIS__h_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x04], DIS: i8);
//test!(rlc__ix_DIS__l_, [RLC(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x05], DIS: i8);
//test!(rlc__iy__a_, [RLC(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x07]);
//test!(rlc__iy__b_, [RLC(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x00]);
//test!(rlc__iy__c_, [RLC(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x01]);
//test!(rlc__iy__d_, [RLC(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x02]);
//test!(rlc__iy__e_, [RLC(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x03]);
//test!(rlc__iy__h_, [RLC(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x04]);
//test!(rlc__iy__l_, [RLC(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x05]);
//test!(rlc__iy_DIS__a_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x07], DIS: i8);
//test!(rlc__iy_DIS__b_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x00], DIS: i8);
//test!(rlc__iy_DIS__c_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x01], DIS: i8);
//test!(rlc__iy_DIS__d_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x02], DIS: i8);
//test!(rlc__iy_DIS__e_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x03], DIS: i8);
//test!(rlc__iy_DIS__h_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x04], DIS: i8);
//test!(rlc__iy_DIS__l_, [RLC(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x05], DIS: i8);
//test!(rr__ix__a_, [RR(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x1F]);
//test!(rr__ix__b_, [RR(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x18]);
//test!(rr__ix__c_, [RR(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x19]);
//test!(rr__ix__d_, [RR(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x1A]);
//test!(rr__ix__e_, [RR(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x1B]);
//test!(rr__ix__h_, [RR(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x1C]);
//test!(rr__ix__l_, [RR(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x1D]);
//test!(rr__ix_DIS__a_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x1F], DIS: i8);
//test!(rr__ix_DIS__b_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x18], DIS: i8);
//test!(rr__ix_DIS__c_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x19], DIS: i8);
//test!(rr__ix_DIS__d_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x1A], DIS: i8);
//test!(rr__ix_DIS__e_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x1B], DIS: i8);
//test!(rr__ix_DIS__h_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x1C], DIS: i8);
//test!(rr__ix_DIS__l_, [RR(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x1D], DIS: i8);
//test!(rr__iy__a_, [RR(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x1F]);
//test!(rr__iy__b_, [RR(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x18]);
//test!(rr__iy__c_, [RR(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x19]);
//test!(rr__iy__d_, [RR(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x1A]);
//test!(rr__iy__e_, [RR(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x1B]);
//test!(rr__iy__h_, [RR(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x1C]);
//test!(rr__iy__l_, [RR(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x1D]);
//test!(rr__iy_DIS__a_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x1F], DIS: i8);
//test!(rr__iy_DIS__b_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x18], DIS: i8);
//test!(rr__iy_DIS__c_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x19], DIS: i8);
//test!(rr__iy_DIS__d_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x1A], DIS: i8);
//test!(rr__iy_DIS__e_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x1B], DIS: i8);
//test!(rr__iy_DIS__h_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x1C], DIS: i8);
//test!(rr__iy_DIS__l_, [RR(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x1D], DIS: i8);
//test!(rrc__ix__a_, [RRC(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x0F]);
//test!(rrc__ix__b_, [RRC(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x08]);
//test!(rrc__ix__c_, [RRC(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x09]);
//test!(rrc__ix__d_, [RRC(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x0A]);
//test!(rrc__ix__e_, [RRC(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x0B]);
//test!(rrc__ix__h_, [RRC(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x0C]);
//test!(rrc__ix__l_, [RRC(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x0D]);
//test!(rrc__ix_DIS__a_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x0F], DIS: i8);
//test!(rrc__ix_DIS__b_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x08], DIS: i8);
//test!(rrc__ix_DIS__c_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x09], DIS: i8);
//test!(rrc__ix_DIS__d_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x0A], DIS: i8);
//test!(rrc__ix_DIS__e_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x0B], DIS: i8);
//test!(rrc__ix_DIS__h_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x0C], DIS: i8);
//test!(rrc__ix_DIS__l_, [RRC(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x0D], DIS: i8);
//test!(rrc__iy__a_, [RRC(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x0F]);
//test!(rrc__iy__b_, [RRC(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x08]);
//test!(rrc__iy__c_, [RRC(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x09]);
//test!(rrc__iy__d_, [RRC(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x0A]);
//test!(rrc__iy__e_, [RRC(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x0B]);
//test!(rrc__iy__h_, [RRC(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x0C]);
//test!(rrc__iy__l_, [RRC(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x0D]);
//test!(rrc__iy_DIS__a_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x0F], DIS: i8);
//test!(rrc__iy_DIS__b_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x08], DIS: i8);
//test!(rrc__iy_DIS__c_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x09], DIS: i8);
//test!(rrc__iy_DIS__d_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x0A], DIS: i8);
//test!(rrc__iy_DIS__e_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x0B], DIS: i8);
//test!(rrc__iy_DIS__h_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x0C], DIS: i8);
//test!(rrc__iy_DIS__l_, [RRC(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x0D], DIS: i8);
//test!(sla__ix__a_, [SLA(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x27]);
//test!(sla__ix__b_, [SLA(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x20]);
//test!(sla__ix__c_, [SLA(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x21]);
//test!(sla__ix__d_, [SLA(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x22]);
//test!(sla__ix__e_, [SLA(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x23]);
//test!(sla__ix__h_, [SLA(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x24]);
//test!(sla__ix__l_, [SLA(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x25]);
//test!(sla__ix_DIS__a_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x27], DIS: i8);
//test!(sla__ix_DIS__b_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x20], DIS: i8);
//test!(sla__ix_DIS__c_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x21], DIS: i8);
//test!(sla__ix_DIS__d_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x22], DIS: i8);
//test!(sla__ix_DIS__e_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x23], DIS: i8);
//test!(sla__ix_DIS__h_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x24], DIS: i8);
//test!(sla__ix_DIS__l_, [SLA(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x25], DIS: i8);
//test!(sla__iy__a_, [SLA(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x27]);
//test!(sla__iy__b_, [SLA(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x20]);
//test!(sla__iy__c_, [SLA(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x21]);
//test!(sla__iy__d_, [SLA(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x22]);
//test!(sla__iy__e_, [SLA(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x23]);
//test!(sla__iy__h_, [SLA(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x24]);
//test!(sla__iy__l_, [SLA(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x25]);
//test!(sla__iy_DIS__a_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x27], DIS: i8);
//test!(sla__iy_DIS__b_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x20], DIS: i8);
//test!(sla__iy_DIS__c_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x21], DIS: i8);
//test!(sla__iy_DIS__d_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x22], DIS: i8);
//test!(sla__iy_DIS__e_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x23], DIS: i8);
//test!(sla__iy_DIS__h_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x24], DIS: i8);
//test!(sla__iy_DIS__l_, [SLA(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x25], DIS: i8);
//test!(sll__ix_DIS__a_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x37], DIS: i8);
//test!(sll__ix_DIS__b_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x30], DIS: i8);
//test!(sll__ix_DIS__c_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x31], DIS: i8);
//test!(sll__ix_DIS__d_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x32], DIS: i8);
//test!(sll__ix_DIS__e_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x33], DIS: i8);
//test!(sll__ix_DIS__h_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x34], DIS: i8);
//test!(sll__ix_DIS__l_, [SLL(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x35], DIS: i8);
//test!(sll__iy__a_, [SLL(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x37]);
//test!(sll__iy__b_, [SLL(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x30]);
//test!(sll__iy__c_, [SLL(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x31]);
//test!(sll__iy__d_, [SLL(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x32]);
//test!(sll__iy__e_, [SLL(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x33]);
//test!(sll__iy__h_, [SLL(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x34]);
//test!(sll__iy__l_, [SLL(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x35]);
//test!(sll__iy_DIS__a_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x37], DIS: i8);
//test!(sll__iy_DIS__b_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x30], DIS: i8);
//test!(sll__iy_DIS__c_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x31], DIS: i8);
//test!(sll__iy_DIS__d_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x32], DIS: i8);
//test!(sll__iy_DIS__e_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x33], DIS: i8);
//test!(sll__iy_DIS__h_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x34], DIS: i8);
//test!(sll__iy_DIS__l_, [SLL(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x35], DIS: i8);
//test!(sra__ix__a_, [SRA(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x2F]);
//test!(sra__ix__b_, [SRA(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x28]);
//test!(sra__ix__c_, [SRA(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x29]);
//test!(sra__ix__d_, [SRA(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x2A]);
//test!(sra__ix__e_, [SRA(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x2B]);
//test!(sra__ix__h_, [SRA(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x2C]);
//test!(sra__ix__l_, [SRA(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x2D]);
//test!(sra__ix_DIS__a_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x2F], DIS: i8);
//test!(sra__ix_DIS__b_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x28], DIS: i8);
//test!(sra__ix_DIS__c_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x29], DIS: i8);
//test!(sra__ix_DIS__d_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x2A], DIS: i8);
//test!(sra__ix_DIS__e_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x2B], DIS: i8);
//test!(sra__ix_DIS__h_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x2C], DIS: i8);
//test!(sra__ix_DIS__l_, [SRA(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x2D], DIS: i8);
//test!(sra__iy__a_, [SRA(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x2F]);
//test!(sra__iy__b_, [SRA(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x28]);
//test!(sra__iy__c_, [SRA(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x29]);
//test!(sra__iy__d_, [SRA(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x2A]);
//test!(sra__iy__e_, [SRA(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x2B]);
//test!(sra__iy__h_, [SRA(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x2C]);
//test!(sra__iy__l_, [SRA(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x2D]);
//test_ub!(sll__ix__a_, [SLL(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x37]);
//test_ub!(sll__ix__b_, [SLL(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x30]);
//test_ub!(sll__ix__c_, [SLL(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x31]);
//test_ub!(sll__ix__d_, [SLL(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x32]);
//test_ub!(sll__ix__e_, [SLL(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x33]);
//test_ub!(sll__ix__h_, [SLL(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x34]);
//test_ub!(sll__ix__l_, [SLL(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x35]);
//test!(sra__iy_DIS__a_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x2F], DIS: i8);
//test!(sra__iy_DIS__b_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x28], DIS: i8);
//test!(sra__iy_DIS__c_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x29], DIS: i8);
//test!(sra__iy_DIS__d_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x2A], DIS: i8);
//test!(sra__iy_DIS__e_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x2B], DIS: i8);
//test!(sra__iy_DIS__h_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x2C], DIS: i8);
//test!(sra__iy_DIS__l_, [SRA(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x2D], DIS: i8);
//test!(srl__ix__a_, [SRL(AddressRegister(IX),ByteRegister(A))], [0xDD, 0xCB, 0x00, 0x3F]);
//test!(srl__ix__b_, [SRL(AddressRegister(IX),ByteRegister(B))], [0xDD, 0xCB, 0x00, 0x38]);
//test!(srl__ix__c_, [SRL(AddressRegister(IX),ByteRegister(C))], [0xDD, 0xCB, 0x00, 0x39]);
//test!(srl__ix__d_, [SRL(AddressRegister(IX),ByteRegister(D))], [0xDD, 0xCB, 0x00, 0x3A]);
//test!(srl__ix__e_, [SRL(AddressRegister(IX),ByteRegister(E))], [0xDD, 0xCB, 0x00, 0x3B]);
//test!(srl__ix__h_, [SRL(AddressRegister(IX),ByteRegister(H))], [0xDD, 0xCB, 0x00, 0x3C]);
//test!(srl__ix__l_, [SRL(AddressRegister(IX),ByteRegister(L))], [0xDD, 0xCB, 0x00, 0x3D]);
//test!(srl__ix_DIS__a_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(A))], [0xDD, 0xCB, DIS, 0x3F], DIS: i8);
//test!(srl__ix_DIS__b_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(B))], [0xDD, 0xCB, DIS, 0x38], DIS: i8);
//test!(srl__ix_DIS__c_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(C))], [0xDD, 0xCB, DIS, 0x39], DIS: i8);
//test!(srl__ix_DIS__d_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(D))], [0xDD, 0xCB, DIS, 0x3A], DIS: i8);
//test!(srl__ix_DIS__e_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(E))], [0xDD, 0xCB, DIS, 0x3B], DIS: i8);
//test!(srl__ix_DIS__h_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(H))], [0xDD, 0xCB, DIS, 0x3C], DIS: i8);
//test!(srl__ix_DIS__l_, [SRL(AddressRegisterWithOffset(IX, DIS),ByteRegister(L))], [0xDD, 0xCB, DIS, 0x3D], DIS: i8);
//test!(srl__iy__a_, [SRL(AddressRegister(IY),ByteRegister(A))], [0xFD, 0xCB, 0x00, 0x3F]);
//test!(srl__iy__b_, [SRL(AddressRegister(IY),ByteRegister(B))], [0xFD, 0xCB, 0x00, 0x38]);
//test!(srl__iy__c_, [SRL(AddressRegister(IY),ByteRegister(C))], [0xFD, 0xCB, 0x00, 0x39]);
//test!(srl__iy__d_, [SRL(AddressRegister(IY),ByteRegister(D))], [0xFD, 0xCB, 0x00, 0x3A]);
//test!(srl__iy__e_, [SRL(AddressRegister(IY),ByteRegister(E))], [0xFD, 0xCB, 0x00, 0x3B]);
//test!(srl__iy__h_, [SRL(AddressRegister(IY),ByteRegister(H))], [0xFD, 0xCB, 0x00, 0x3C]);
//test!(srl__iy__l_, [SRL(AddressRegister(IY),ByteRegister(L))], [0xFD, 0xCB, 0x00, 0x3D]);
//test!(srl__iy_DIS__a_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(A))], [0xFD, 0xCB, DIS, 0x3F], DIS: i8);
//test!(srl__iy_DIS__b_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(B))], [0xFD, 0xCB, DIS, 0x38], DIS: i8);
//test!(srl__iy_DIS__c_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(C))], [0xFD, 0xCB, DIS, 0x39], DIS: i8);
//test!(srl__iy_DIS__d_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(D))], [0xFD, 0xCB, DIS, 0x3A], DIS: i8);
//test!(srl__iy_DIS__e_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(E))], [0xFD, 0xCB, DIS, 0x3B], DIS: i8);
//test!(srl__iy_DIS__h_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(H))], [0xFD, 0xCB, DIS, 0x3C], DIS: i8);
//test!(srl__iy_DIS__l_, [SRL(AddressRegisterWithOffset(IY, DIS),ByteRegister(L))], [0xFD, 0xCB, DIS, 0x3D], DIS: i8);
//test!(sub_hl_bc_, [SUB(WordRegister(HL),WordRegister(BC))], [0xB7, 0xED, 0x42]);
//test!(sub_hl_de_, [SUB(WordRegister(HL),WordRegister(DE))], [0xB7, 0xED, 0x52]);
//test!(sub_hl_hl_, [SUB(WordRegister(HL),WordRegister(HL))], [0xB7, 0xED, 0x62]);
//test!(sub_hl_sp_, [SUB(WordRegister(HL),WordRegister(SP))], [0xB7, 0xED, 0x72]);
test!(sbc_a_N_, [SBC(ByteRegister(A),Constant(n as i32))], [0xDE, n], n: u8);
test!(sub_N_, [SUB(Constant(n as i32))], [0xD6, n], n: u8);
test!(xor_N_, [XOR(Constant(n as i32))], [0xEE, n], n: u8);
/*
res  0,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0x87
res  0,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0x80
res  0,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0x81
res  0,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0x82
res  0,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0x83
res  0,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0x84
res  0,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0x85
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0x87
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0x80
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0x81
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0x82
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0x83
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0x84
res  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0x85
res  0,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0x87
res  0,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0x80
res  0,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0x81
res  0,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0x82
res  0,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0x83
res  0,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0x84
res  0,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0x85
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0x87
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0x80
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0x81
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0x82
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0x83
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0x84
res  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0x85
res  1,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0x8F
res  1,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0x88
res  1,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0x89
res  1,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0x8A
res  1,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0x8B
res  1,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0x8C
res  1,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0x8D
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0x8F
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0x88
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0x89
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0x8A
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0x8B
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0x8C
res  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0x8D
res  1,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0x8F
res  1,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0x88
res  1,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0x89
res  1,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0x8A
res  1,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0x8B
res  1,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0x8C
res  1,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0x8D
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0x8F
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0x88
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0x89
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0x8A
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0x8B
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0x8C
res  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0x8D
res  2,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0x97
res  2,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0x90
res  2,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0x91
res  2,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0x92
res  2,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0x93
res  2,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0x94
res  2,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0x95
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0x97
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0x90
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0x91
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0x92
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0x93
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0x94
res  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0x95
res  2,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0x97
res  2,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0x90
res  2,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0x91
res  2,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0x92
res  2,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0x93
res  2,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0x94
res  2,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0x95
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0x97
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0x90
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0x91
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0x92
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0x93
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0x94
res  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0x95
res  3,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0x9F
res  3,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0x98
res  3,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0x99
res  3,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0x9A
res  3,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0x9B
res  3,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0x9C
res  3,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0x9D
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0x9F
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0x98
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0x99
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0x9A
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0x9B
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0x9C
res  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0x9D
res  3,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0x9F
res  3,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0x98
res  3,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0x99
res  3,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0x9A
res  3,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0x9B
res  3,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0x9C
res  3,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0x9D
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0x9F
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0x98
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0x99
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0x9A
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0x9B
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0x9C
res  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0x9D
res  4,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xA7
res  4,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xA0
res  4,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xA1
res  4,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xA2
res  4,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xA3
res  4,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xA4
res  4,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xA5
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xA7
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xA0
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xA1
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xA2
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xA3
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xA4
res  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xA5
res  4,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xA7
res  4,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xA0
res  4,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xA1
res  4,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xA2
res  4,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xA3
res  4,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xA4
res  4,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xA5
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xA7
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xA0
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xA1
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xA2
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xA3
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xA4
res  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xA5
res  5,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xAF
res  5,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xA8
res  5,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xA9
res  5,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xAA
res  5,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xAB
res  5,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xAC
res  5,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xAD
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xAF
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xA8
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xA9
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xAA
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xAB
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xAC
res  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xAD
res  5,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xAF
res  5,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xA8
res  5,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xA9
res  5,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xAA
res  5,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xAB
res  5,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xAC
res  5,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xAD
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xAF
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xA8
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xA9
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xAA
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xAB
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xAC
res  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xAD
res  6,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xB7
res  6,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xB0
res  6,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xB1
res  6,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xB2
res  6,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xB3
res  6,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xB4
res  6,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xB5
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xB7
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xB0
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xB1
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xB2
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xB3
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xB4
res  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xB5
res  6,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xB7
res  6,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xB0
res  6,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xB1
res  6,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xB2
res  6,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xB3
res  6,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xB4
res  6,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xB5
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xB7
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xB0
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xB1
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xB2
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xB3
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xB4
res  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xB5
res  7,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xBF
res  7,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xB8
res  7,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xB9
res  7,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xBA
res  7,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xBB
res  7,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xBC
res  7,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xBD
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xBF
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xB8
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xB9
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xBA
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xBB
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xBC
res  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xBD
res  7,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xBF
res  7,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xB8
res  7,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xB9
res  7,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xBA
res  7,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xBB
res  7,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xBC
res  7,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xBD
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xBF
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xB8
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xB9
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xBA
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xBB
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xBC
res  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xBD
set  0,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xC7
set  0,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xC0
set  0,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xC1
set  0,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xC2
set  0,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xC3
set  0,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xC4
set  0,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xC5
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xC7
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xC0
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xC1
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xC2
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xC3
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xC4
set  0,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xC5
set  0,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xC7
set  0,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xC0
set  0,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xC1
set  0,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xC2
set  0,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xC3
set  0,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xC4
set  0,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xC5
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xC7
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xC0
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xC1
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xC2
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xC3
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xC4
set  0,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xC5
set  1,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xCF
set  1,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xC8
set  1,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xC9
set  1,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xCA
set  1,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xCB
set  1,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xCC
set  1,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xCD
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xCF
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xC8
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xC9
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xCA
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xCB
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xCC
set  1,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xCD
set  1,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xCF
set  1,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xC8
set  1,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xC9
set  1,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xCA
set  1,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xCB
set  1,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xCC
set  1,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xCD
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xCF
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xC8
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xC9
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xCA
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xCB
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xCC
set  1,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xCD
set  2,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xD7
set  2,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xD0
set  2,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xD1
set  2,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xD2
set  2,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xD3
set  2,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xD4
set  2,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xD5
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xD7
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xD0
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xD1
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xD2
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xD3
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xD4
set  2,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xD5
set  2,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xD7
set  2,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xD0
set  2,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xD1
set  2,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xD2
set  2,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xD3
set  2,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xD4
set  2,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xD5
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xD7
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xD0
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xD1
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xD2
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xD3
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xD4
set  2,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xD5
set  3,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xDF
set  3,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xD8
set  3,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xD9
set  3,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xDA
set  3,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xDB
set  3,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xDC
set  3,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xDD
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xDF
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xD8
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xD9
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xDA
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xDB
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xDC
set  3,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xDD
set  3,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xDF
set  3,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xD8
set  3,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xD9
set  3,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xDA
set  3,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xDB
set  3,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xDC
set  3,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xDD
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xDF
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xD8
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xD9
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xDA
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xDB
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xDC
set  3,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xDD
set  4,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xE7
set  4,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xE0
set  4,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xE1
set  4,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xE2
set  4,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xE3
set  4,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xE4
set  4,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xE5
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xE7
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xE0
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xE1
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xE2
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xE3
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xE4
set  4,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xE5
set  4,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xE7
set  4,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xE0
set  4,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xE1
set  4,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xE2
set  4,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xE3
set  4,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xE4
set  4,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xE5
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xE7
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xE0
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xE1
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xE2
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xE3
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xE4
set  4,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xE5
set  5,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xEF
set  5,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xE8
set  5,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xE9
set  5,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xEA
set  5,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xEB
set  5,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xEC
set  5,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xED
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xEF
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xE8
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xE9
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xEA
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xEB
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xEC
set  5,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xED
set  5,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xEF
set  5,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xE8
set  5,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xE9
set  5,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xEA
set  5,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xEB
set  5,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xEC
set  5,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xED
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xEF
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xE8
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xE9
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xEA
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xEB
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xEC
set  5,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xED
set  6,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xF7
set  6,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xF0
set  6,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xF1
set  6,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xF2
set  6,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xF3
set  6,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xF4
set  6,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xF5
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xF7
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xF0
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xF1
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xF2
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xF3
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xF4
set  6,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xF5
set  6,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xF7
set  6,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xF0
set  6,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xF1
set  6,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xF2
set  6,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xF3
set  6,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xF4
set  6,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xF5
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xF7
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xF0
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xF1
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xF2
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xF3
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xF4
set  6,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xF5
set  7,(WordRegister(IX)),ByteRegister(A)      ; 0xDD 0xCB 0x00 0xFF
set  7,(WordRegister(IX)),ByteRegister(B)      ; 0xDD 0xCB 0x00 0xF8
set  7,(WordRegister(IX)),ByteRegister(C)      ; 0xDD 0xCB 0x00 0xF9
set  7,(WordRegister(IX)),ByteRegister(D)      ; 0xDD 0xCB 0x00 0xFA
set  7,(WordRegister(IX)),ByteRegister(E)      ; 0xDD 0xCB 0x00 0xFB
set  7,(WordRegister(IX)),ByteRegister(H)      ; 0xDD 0xCB 0x00 0xFC
set  7,(WordRegister(IX)),ByteRegister(L)      ; 0xDD 0xCB 0x00 0xFD
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(A)  ; 0xDD 0xCB DIS 0xFF
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(B)  ; 0xDD 0xCB DIS 0xF8
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(C)  ; 0xDD 0xCB DIS 0xF9
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(D)  ; 0xDD 0xCB DIS 0xFA
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(E)  ; 0xDD 0xCB DIS 0xFB
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(H)  ; 0xDD 0xCB DIS 0xFC
set  7,AddressRegisterWithOffset(IX, DIS),ByteRegister(L)  ; 0xDD 0xCB DIS 0xFD
set  7,(WordRegister(IY)),ByteRegister(A)      ; 0xFD 0xCB 0x00 0xFF
set  7,(WordRegister(IY)),ByteRegister(B)      ; 0xFD 0xCB 0x00 0xF8
set  7,(WordRegister(IY)),ByteRegister(C)      ; 0xFD 0xCB 0x00 0xF9
set  7,(WordRegister(IY)),ByteRegister(D)      ; 0xFD 0xCB 0x00 0xFA
set  7,(WordRegister(IY)),ByteRegister(E)      ; 0xFD 0xCB 0x00 0xFB
set  7,(WordRegister(IY)),ByteRegister(H)      ; 0xFD 0xCB 0x00 0xFC
set  7,(WordRegister(IY)),ByteRegister(L)      ; 0xFD 0xCB 0x00 0xFD
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(A)  ; 0xFD 0xCB DIS 0xFF
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(B)  ; 0xFD 0xCB DIS 0xF8
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(C)  ; 0xFD 0xCB DIS 0xF9
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(D)  ; 0xFD 0xCB DIS 0xFA
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(E)  ; 0xFD 0xCB DIS 0xFB
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(H)  ; 0xFD 0xCB DIS 0xFC
set  7,AddressRegisterWithOffset(IY, DIS),ByteRegister(L)  ; 0xFD 0xCB DIS 0xFD
*/
