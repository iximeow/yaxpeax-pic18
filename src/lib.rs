#[cfg(feature="use-serde")]
#[macro_use] extern crate serde_derive;
#[cfg(feature="use-serde")]
extern crate serde;
//#[cfg(feature="use-serde")]
//use serde::{Serialize, Deserialize};

extern crate yaxpeax_arch;

use yaxpeax_arch::{Arch, AddressDiff, Decoder, LengthedInstruction, Reader, StandardDecodeError};

pub mod consts;
pub mod display;

#[cfg_attr(feature="use-serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct PIC18;

impl Arch for PIC18 {
    type Address = u32;
    type Word = u8;
    type Instruction = Instruction;
    type DecodeError = StandardDecodeError;
    type Decoder = InstDecoder;
    type Operand = Operand;
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: [Operand; 2]
}

impl LengthedInstruction for Instruction {
    type Unit = AddressDiff<<PIC18 as Arch>::Address>;
    fn min_size() -> Self::Unit {
        AddressDiff::from_const(2)
    }
    fn len(&self) -> Self::Unit {
        match self.opcode {
            Opcode::MOVFF
                | Opcode::MOVSF
                | Opcode::MOVSD
                | Opcode::CALL
                | Opcode::LFSR
                | Opcode::GOTO => {
                AddressDiff::from_const(4)
            },
            _ => AddressDiff::from_const(2)
        }
    }
}

impl yaxpeax_arch::Instruction for Instruction {
    // TODO: this is wrong!!
    fn well_defined(&self) -> bool { true }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: Opcode::NOP,
            operands: [Operand::Nothing, Operand::Nothing]
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Invalid(u8, u8),
    NOP,
    MOVFF,
    MOVSF,
    MOVSD,
    CALL,
    LFSR,
    GOTO,
    CALLW,
    CLRWDT,
    DAW,
    POP,
    PUSH,
    RESET,
    RETFIE,
    RETFIE_FAST,
    RETURN,
    RETURN_FAST,
    SLEEP,
    TBLRD_I_S,
    TBLRD_S,
    TBLRD_S_D,
    TBLRD_S_I,
    TBLWT_I_S,
    TBLWT_S,
    TBLWT_S_D,
    TBLWT_S_I,
    MOVLB,
    ADDLW,
    MOVLW,
    MULLW,
    RETLW,
    ANDLW,
    XORLW,
    IORLW,
    SUBLW,
    IORWF,
    ANDWF,
    XORWF,
    COMF,
    MULWF,
    ADDWFC,
    ADDWF,
    INCF,
    DECF,
    DECFSZ,
    RRCF,
    RLCF,
    SWAPF,
    INCFSZ,
    RRNCF,
    RLNCF,
    INFSNZ,
    DCFSNZ,
    MOVF,
    SUBFWB,
    SUBWFB,
    SUBWF,
    CPFSLT,
    CPFSEQ,
    CPFSGT,
    TSTFSZ,
    SETF,
    CLRF,
    NEGF,
    MOVWF,
    BTG,
    BSF,
    BCF,
    BTFSS,
    BTFSC,
    BZ,
    BNZ,
    BC,
    BNC,
    BOV,
    BNOV,
    BN,
    BNN,
    BRA,
    RCALL
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    ImmediateU8(u8),
    ImmediateU32(u32),
    FileFSR(u8),
    File(u8, bool), // a == banked
    AbsoluteFile(u16),
    RedirectableFile(u8, bool, bool), // a == banked, d == direction
    Nothing
}

#[derive(Default, Debug)]
pub struct InstDecoder {}

impl Decoder<PIC18> for InstDecoder {
    fn decode_into<T: Reader<<PIC18 as Arch>::Address, <PIC18 as Arch>::Word>>(&self, inst: &mut Instruction, words: &mut T) -> Result<(), <PIC18 as Arch>::DecodeError> {
        let mut word = [0u8; 2];
        words.next_n(&mut word)?;

//            println!("Decoding {:x?}", word);
        match word[1] {
            0x00 => {
                match word[0] {
                    0x00 => {
                        inst.opcode = Opcode::NOP;
                        Ok(())
                    },
                    0b00000011 => {
                        inst.opcode = Opcode::SLEEP;
                        Ok(())
                    },
                    0b00000100 => {
                        inst.opcode = Opcode::CLRWDT;
                        Ok(())
                    },
                    0b00000101 => {
                        inst.opcode = Opcode::PUSH;
                        Ok(())
                    },
                    0b00000110 => {
                        inst.opcode = Opcode::POP;
                        Ok(())
                    },
                    0b00000111 => {
                        inst.opcode = Opcode::DAW;
                        Ok(())
                    },
                    0b00001000 => {
                        inst.opcode = Opcode::TBLRD_S;
                        Ok(())
                    },
                    0b00001001 => {
                        inst.opcode = Opcode::TBLRD_S_I;
                        Ok(())
                    },
                    0b00001010 => {
                        inst.opcode = Opcode::TBLRD_S_D;
                        Ok(())
                    },
                    0b00001011 => {
                        inst.opcode = Opcode::TBLRD_I_S;
                        Ok(())
                    },
                    0b00001100 => {
                        inst.opcode = Opcode::TBLWT_S;
                        Ok(())
                    },
                    0b00001101 => {
                        inst.opcode = Opcode::TBLWT_S_I;
                        Ok(())
                    },
                    0b00001110 => {
                        inst.opcode = Opcode::TBLWT_S_D;
                        Ok(())
                    },
                    0b00001111 => {
                        inst.opcode = Opcode::TBLWT_I_S;
                        Ok(())
                    },
                    0b00010000 => {
                        inst.opcode = Opcode::RETFIE;
                        Ok(())
                    },
                    0b00010001 => {
                        inst.opcode = Opcode::RETFIE_FAST;
                        Ok(())
                    },
                    0b00010010 => {
                        inst.opcode = Opcode::RETURN;
                        Ok(())
                    },
                    0b00010011 => {
                        inst.opcode = Opcode::RETURN_FAST;
                        Ok(())
                    },
                    0b00010100 => {
                        inst.opcode = Opcode::CALLW;
                        Ok(())
                    },
                    0b11111111 => {
                        inst.opcode = Opcode::RESET;
                        Ok(())
                    },
                    _ => {
                        inst.opcode = Opcode::Invalid(word[0], word[1]);
                        Err(StandardDecodeError::InvalidOpcode)
                    }
                }
            },
            0x01 => {
                inst.opcode = Opcode::MOVLB;
                // this ignores high nibble of low word. ok by isa, but...
                inst.operands[0] = Operand::ImmediateU8(word[0] & 0x0f);
                Ok(())
            },
            0x02 | 0x03 => {
                inst.opcode = Opcode::MULWF;
                let a = (word[1] & 0x01) == 1;
                inst.operands[0] = Operand::File(word[0], a);
                Ok(())
            },
            0x04 | 0x05 | 0x06 | 0x07 => {
                inst.opcode = Opcode::DECF;
                let d = ((word[1] >> 1) & 0x01u8) == 1u8;
                let a = (word[1] & 0x01) == 1;
                inst.operands[0] = Operand::RedirectableFile(word[0], a, d);
                Ok(())
            },
            0x08 => {
                inst.opcode = Opcode::SUBLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x09 => {
                inst.opcode = Opcode::IORLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0a => {
                inst.opcode = Opcode::XORLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0b => {
                inst.opcode = Opcode::ANDLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0c => {
                inst.opcode = Opcode::RETLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0d => {
                inst.opcode = Opcode::MULLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0e => {
                inst.opcode = Opcode::MOVLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0x0f => {
                inst.opcode = Opcode::ADDLW;
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            x if x >= 0x10 && x < 0b01100000 => {
                let da = x & 0b0011;
                let opc = (x >> 2) - 4;
                inst.opcode = [
                    Opcode::IORWF,
                    Opcode::ANDWF,
                    Opcode::XORWF,
                    Opcode::COMF,
                    Opcode::ADDWFC,
                    Opcode::ADDWF,
                    Opcode::INCF,
                    Opcode::DECFSZ,
                    Opcode::RRCF,
                    Opcode::RLCF,
                    Opcode::SWAPF,
                    Opcode::INCFSZ,
                    Opcode::RRNCF,
                    Opcode::RLNCF,
                    Opcode::INFSNZ,
                    Opcode::DCFSNZ,
                    Opcode::MOVF,
                    Opcode::SUBFWB,
                    Opcode::SUBWFB,
                    Opcode::SUBWF
                ][opc as usize];
                inst.operands[0] = Operand::RedirectableFile(word[0], (da & 0x01) == 0x01, (da & 0x02) == 0x02);
                Ok(())
            },
            x if x >= 0b01100000 && x < 0b01110000 => {
                let a = x & 1;
                let opc = (x >> 1) & 0b0000111;
                inst.opcode = [
                    Opcode::CPFSLT,
                    Opcode::CPFSEQ,
                    Opcode::CPFSGT,
                    Opcode::TSTFSZ,
                    Opcode::SETF,
                    Opcode::CLRF,
                    Opcode::NEGF,
                    Opcode::MOVWF
                ][opc as usize];
                inst.operands[0] = Operand::File(word[0], a == 1);
                Ok(())
            },
            x if x >= 0b01110000 && x < 0b11000000 => {
                let a = x & 1;
                let opc = ((x >> 4) & 0b00001111) - 0b111;
                inst.opcode = [
                    Opcode::BTG,
                    Opcode::BSF,
                    Opcode::BCF,
                    Opcode::BTFSS,
                    Opcode::BTFSC
                ][opc as usize];
                let bit = (x >> 1) & 0b0000111;
                inst.operands[0] = Operand::File(word[0], a == 1);
                inst.operands[1] = Operand::ImmediateU8(bit);
                Ok(())
            },
            x if x >= 0b11000000 && x < 0b11010000 => {
                inst.opcode = Opcode::MOVFF;
                let mut word2 = [0u8; 2];
                words.next_n(&mut word2)?;
                if word2[1] & 0xf0 != 0xf0 {
                    return Err(StandardDecodeError::InvalidOperand);
                }

                let src = (word[0] as u16) | ((word[1] as u16 & 0x0f) << 8);
                let dest = (word2[0] as u16) | ((word2[1] as u16 & 0x0f) << 8);
                inst.operands[0] = Operand::AbsoluteFile(src);
                inst.operands[1] = Operand::AbsoluteFile(dest);
                Ok(())
            },
            x if x >= 0b11010000 && x < 0b11100000 => {
                inst.opcode = [
                    Opcode::BRA,
                    Opcode::RCALL
                ][((x >> 3) & 1) as usize];
                inst.operands[0] = Operand::ImmediateU32((((x & 0b111) as u32) << 8) | word[0] as u32);
                Ok(())
            },
            x if x >= 0b11100000 && x < 0b11101000 => {
                let opc = x & 0b00000111;
                inst.opcode = [
                    Opcode::BZ,
                    Opcode::BNZ,
                    Opcode::BC,
                    Opcode::BNC,
                    Opcode::BOV,
                    Opcode::BNOV,
                    Opcode::BN,
                    Opcode::BNN
                ][opc as usize];
                inst.operands[0] = Operand::ImmediateU8(word[0]);
                Ok(())
            },
            0xee => {
                let f_k_msb = word[0];
                let mut word2 = [0u8; 2];
                words.next_n(&mut word2)?;

                if (word2[1] & 0xf0) != 0xf0 {
                    return Err(StandardDecodeError::InvalidOperand);
                }

                inst.opcode = Opcode::LFSR;

                let f = (f_k_msb >> 4) & 0b0011;
                let k_msb = f_k_msb & 0b1111;
                let k_lsb = word2[0];

                inst.operands[0] = Operand::FileFSR(f);
                inst.operands[1] = Operand::ImmediateU32(((k_msb as u32) << 8) | (k_lsb as u32));
                Ok(())
            }
            /* ... */
            0xeb | 0xec => {
                // TODO: respect s bit
                let k_lsb = word[0];
                let mut word2 = [0u8; 2];
                words.next_n(&mut word2)?;

                if (word2[1] & 0xf0) != 0xf0 {
                    return Err(StandardDecodeError::InvalidOperand);
                }

                let k_msb = (((word2[1] & 0xf) as u32) << 8) | word2[0] as u32;

                inst.opcode = Opcode::CALL;
                inst.operands[0] = Operand::ImmediateU32(((k_msb << 8) | k_lsb as u32) << 1);
                Ok(())
            }
            0xef => {
                let k_lsb = word[0];
                let mut word2 = [0u8; 2];
                words.next_n(&mut word2)?;

                if (word2[1] & 0xf0) != 0xf0 {
                    return Err(StandardDecodeError::InvalidOperand);
                }

                let k_msb = (((word2[1] & 0xf) as u32) << 8) | word2[0] as u32;

                inst.opcode = Opcode::GOTO;
                inst.operands[0] = Operand::ImmediateU32(((k_msb << 8) | k_lsb as u32) << 1);
                Ok(())
            }
            _ => Err(StandardDecodeError::InvalidOpcode)
        }
    }
}

