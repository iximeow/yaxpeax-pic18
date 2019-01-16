extern crate yaxpeax_arch;

use yaxpeax_arch::{Arch, Decodable, LengthedInstruction};

pub mod consts;
pub mod display;

pub struct PIC18;
impl Arch for PIC18 {
    type Address = u32;
    type Instruction = Instruction;
    type Operand = Operand;
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: [Operand; 2]
}

impl LengthedInstruction for Instruction {
    type Unit = <PIC18 as Arch>::Address;
    fn len(&self) -> Self::Unit {
        match self.opcode {
            Opcode::MOVFF
                | Opcode::MOVSF
                | Opcode::MOVSD
                | Opcode::CALL
                | Opcode::LFSR
                | Opcode::GOTO => {
                4
            },
            _ => 2
        }
    }
}

impl Instruction {
    pub fn blank() -> Instruction {
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

impl Decodable for Instruction {
    fn decode<'a, T: IntoIterator<Item=&'a u8>>(bytes: T) -> Option<Self> {
        let mut blank = Instruction::blank();
        match blank.decode_into(bytes) {
            Some(_) => Some(blank),
            None => None
        }
    }
    fn decode_into<'a, T: IntoIterator<Item=&'a u8>>(&mut self, bytes: T) -> Option<()> {
        let mut bytes_iter = bytes.into_iter();
        let word: Vec<&'a u8> = bytes_iter.by_ref().take(2).collect();
        if word.len() != 2 {
            return None;
        }

//            println!("Decoding {:x?}", word);
        match *word[1] {
            0x00 => {
                match *word[0] {
                    0x00 => {
                        self.opcode = Opcode::NOP;
                        Some(())
                    },
                    0b00000011 => {
                        self.opcode = Opcode::SLEEP;
                        Some(())
                    },
                    0b00000100 => {
                        self.opcode = Opcode::CLRWDT;
                        Some(())
                    },
                    0b00000101 => {
                        self.opcode = Opcode::PUSH;
                        Some(())
                    },
                    0b00000110 => {
                        self.opcode = Opcode::POP;
                        Some(())
                    },
                    0b00000111 => {
                        self.opcode = Opcode::DAW;
                        Some(())
                    },
                    0b00001000 => {
                        self.opcode = Opcode::TBLRD_S;
                        Some(())
                    },
                    0b00001001 => {
                        self.opcode = Opcode::TBLRD_S_I;
                        Some(())
                    },
                    0b00001010 => {
                        self.opcode = Opcode::TBLRD_S_D;
                        Some(())
                    },
                    0b00001011 => {
                        self.opcode = Opcode::TBLRD_I_S;
                        Some(())
                    },
                    0b00001100 => {
                        self.opcode = Opcode::TBLWT_S;
                        Some(())
                    },
                    0b00001101 => {
                        self.opcode = Opcode::TBLWT_S_I;
                        Some(())
                    },
                    0b00001110 => {
                        self.opcode = Opcode::TBLWT_S_D;
                        Some(())
                    },
                    0b00001111 => {
                        self.opcode = Opcode::TBLWT_I_S;
                        Some(())
                    },
                    0b00010000 => {
                        self.opcode = Opcode::RETFIE;
                        Some(())
                    },
                    0b00010001 => {
                        self.opcode = Opcode::RETFIE_FAST;
                        Some(())
                    },
                    0b00010010 => {
                        self.opcode = Opcode::RETURN;
                        Some(())
                    },
                    0b00010011 => {
                        self.opcode = Opcode::RETURN_FAST;
                        Some(())
                    },
                    0b00010100 => {
                        self.opcode = Opcode::CALLW;
                        Some(())
                    },
                    0b11111111 => {
                        self.opcode = Opcode::RESET;
                        Some(())
                    },
                    _ => {
                        self.opcode = Opcode::Invalid(*word[0], *word[1]);
                        None
                    }
                }
            },
            0x01 => {
                self.opcode = Opcode::MOVLB;
                // this ignores high nibble of low word. ok by isa, but...
                self.operands[0] = Operand::ImmediateU8(*word[0] & 0x0f);
                Some(())
            },
            0x02 | 0x03 => {
                self.opcode = Opcode::MULWF;
                let a = (word[1] & 0x01) == 1;
                self.operands[0] = Operand::File(*word[0], a);
                Some(())
            },
            0x04 | 0x05 | 0x06 | 0x07 => {
                self.opcode = Opcode::DECF;
                let d = ((word[1] >> 1) & 0x01u8) == 1u8;
                let a = (word[1] & 0x01) == 1;
                self.operands[0] = Operand::RedirectableFile(*word[0], a, d);
                Some(())
            },
            0x08 => {
                self.opcode = Opcode::SUBLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x09 => {
                self.opcode = Opcode::IORLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0a => {
                self.opcode = Opcode::XORLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0b => {
                self.opcode = Opcode::ANDLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0c => {
                self.opcode = Opcode::RETLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0d => {
                self.opcode = Opcode::MULLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0e => {
                self.opcode = Opcode::MOVLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0x0f => {
                self.opcode = Opcode::ADDLW;
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            x if x >= 0x10 && x < 0b01100000 => {
                let da = x & 0b0011;
                let opc = (x >> 2) - 4;
                self.opcode = [
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
                self.operands[0] = Operand::RedirectableFile(*word[0], (da & 0x01) == 0x01, (da & 0x02) == 0x02);
                Some(())
            },
            x if x >= 0b01100000 && x < 0b01110000 => {
                let a = x & 1;
                let opc = (x >> 1) & 0b0000111;
                self.opcode = [
                    Opcode::CPFSLT,
                    Opcode::CPFSEQ,
                    Opcode::CPFSGT,
                    Opcode::TSTFSZ,
                    Opcode::SETF,
                    Opcode::CLRF,
                    Opcode::NEGF,
                    Opcode::MOVWF
                ][opc as usize];
                self.operands[0] = Operand::File(*word[0], a == 1);
                Some(())
            },
            x if x >= 0b01110000 && x < 0b11000000 => {
                let a = x & 1;
                let opc = ((x >> 4) & 0b00001111) - 0b111;
                self.opcode = [
                    Opcode::BTG,
                    Opcode::BSF,
                    Opcode::BCF,
                    Opcode::BTFSS,
                    Opcode::BTFSC
                ][opc as usize];
                let bit = (x >> 1) & 0b0000111;
                self.operands[0] = Operand::File(*word[0], a == 1);
                self.operands[1] = Operand::ImmediateU8(bit);
                Some(())
            },
            x if x >= 0b11000000 && x < 0b11010000 => {
                self.opcode = Opcode::MOVFF;
                let word2: Vec<&'a u8> = bytes_iter.take(2).collect();
                if word2.len() != 2 {
                    return None;
                }
                if word2[1] & 0xf0 != 0xf0 {
                    return None;
                }

                let src = (*word[0] as u16) | ((*word[1] as u16 & 0x0f) << 8);
                let dest = (*word2[0] as u16) | ((*word2[1] as u16 & 0x0f) << 8);
                self.operands[0] = Operand::AbsoluteFile(src);
                self.operands[1] = Operand::AbsoluteFile(dest);
                Some(())
            },
            x if x >= 0b11010000 && x < 0b11100000 => {
                self.opcode = [
                    Opcode::BRA,
                    Opcode::RCALL
                ][((x >> 3) & 1) as usize];
                self.operands[0] = Operand::ImmediateU32((((x & 0b111) as u32) << 8) | *word[0] as u32);
                Some(())
            },
            x if x >= 0b11100000 && x < 0b11101000 => {
                let opc = x & 0b00000111;
                self.opcode = [
                    Opcode::BZ,
                    Opcode::BNZ,
                    Opcode::BC,
                    Opcode::BNC,
                    Opcode::BOV,
                    Opcode::BNOV,
                    Opcode::BN,
                    Opcode::BNN
                ][opc as usize];
                self.operands[0] = Operand::ImmediateU8(*word[0]);
                Some(())
            },
            0xee => {
                let f_k_msb = *word[0];
                let word2: Vec<&'a u8> = bytes_iter.take(2).collect();
                if word2.len() != 2 {
                    return None;
                }

                if (word2[1] & 0xf0) != 0xf0 {
                    return None; // invalid instruction
                }

                self.opcode = Opcode::LFSR;

                let f = (f_k_msb >> 4) & 0b0011;
                let k_msb = f_k_msb & 0b1111;
                let k_lsb = word2[0];

                self.operands[0] = Operand::FileFSR(f);
                self.operands[1] = Operand::ImmediateU32(((k_msb as u32) << 8) | (*k_lsb as u32));
                Some(())
            }
            /* ... */
            0xeb | 0xec => {
                // TODO: respect s bit
                let k_lsb = *word[0];
                let word2: Vec<&'a u8> = bytes_iter.take(2).collect();
                if word2.len() != 2 {
                    return None;
                }

                if (word2[1] & 0xf0) != 0xf0 {
                    return None; // invalid instruction
                }

                let k_msb = (((*word2[1] & 0xf) as u32) << 8) | *word2[0] as u32;

                self.opcode = Opcode::CALL;
                self.operands[0] = Operand::ImmediateU32(((k_msb << 8) | k_lsb as u32) << 1);
                Some(())
            }
            0xef => {
                let k_lsb = *word[0];
                let word2: Vec<&'a u8> = bytes_iter.take(2).collect();
                if word2.len() != 2 {
                    return None;
                }

                if (word2[1] & 0xf0) != 0xf0 {
                    return None; // invalid instruction
                }

                let k_msb = (((*word2[1] & 0xf) as u32) << 8) | *word2[0] as u32;

                self.opcode = Opcode::GOTO;
                self.operands[0] = Operand::ImmediateU32(((k_msb << 8) | k_lsb as u32) << 1);
                Some(())
            }
            _ => None
        }
    }
}

