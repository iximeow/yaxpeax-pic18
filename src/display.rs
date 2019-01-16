use crate::{Instruction, Opcode, Operand};
use crate::consts;

use std;
use std::fmt::{Display, Formatter};
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.opcode)?;
        match self.operands[0] {
            Operand::Nothing => return Ok(()),
            x @ _ => {
                write!(f, " {}", x)?;
            }
        };
        match self.operands[1] {
            Operand::Nothing => return Ok(()),
            x @ _ => {
                write!(f, ", {}", x)?;
            }
        };
        Ok(())
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Opcode::Invalid(a, b) => { write!(f, "invalid({:02x}{:02x})", a, b) },
            Opcode::NOP => { write!(f, "nop") },
            Opcode::MOVFF => { write!(f, "movff") },
            Opcode::MOVSF => { write!(f, "movsf") },
            Opcode::MOVSD => { write!(f, "movsd") },
            Opcode::CALL => { write!(f, "call") },
            Opcode::LFSR => { write!(f, "lfsr") },
            Opcode::GOTO => { write!(f, "goto") },
            Opcode::CALLW => { write!(f, "callw") },
            Opcode::CLRWDT => { write!(f, "clrwdt") },
            Opcode::DAW => { write!(f, "daw") },
            Opcode::POP => { write!(f, "pop") },
            Opcode::PUSH => { write!(f, "push") },
            Opcode::RESET => { write!(f, "reset") },
            Opcode::RETFIE => { write!(f, "retfie") },
            Opcode::RETFIE_FAST => { write!(f, "retfie_fast") },
            Opcode::RETURN => { write!(f, "return") },
            Opcode::RETURN_FAST => { write!(f, "return_fast") },
            Opcode::SLEEP => { write!(f, "sleep") },
            Opcode::TBLRD_I_S => { write!(f, "tblrd_i_s") },
            Opcode::TBLRD_S => { write!(f, "tblrd_s") },
            Opcode::TBLRD_S_D => { write!(f, "tblrd_s_d") },
            Opcode::TBLRD_S_I => { write!(f, "tblrd_s_i") },
            Opcode::TBLWT_I_S => { write!(f, "tblwt_i_s") },
            Opcode::TBLWT_S => { write!(f, "tblwt_s") },
            Opcode::TBLWT_S_D => { write!(f, "tblwt_s_d") },
            Opcode::TBLWT_S_I => { write!(f, "tblwt_s_i") },
            Opcode::MOVLB => { write!(f, "movlb") },
            Opcode::ADDLW => { write!(f, "addlw") },
            Opcode::MOVLW => { write!(f, "movlw") },
            Opcode::MULLW => { write!(f, "mullw") },
            Opcode::RETLW => { write!(f, "retlw") },
            Opcode::ANDLW => { write!(f, "andlw") },
            Opcode::XORLW => { write!(f, "xorlw") },
            Opcode::IORLW => { write!(f, "iorlw") },
            Opcode::SUBLW => { write!(f, "sublw") },
            Opcode::IORWF => { write!(f, "iorwf") },
            Opcode::ANDWF => { write!(f, "andwf") },
            Opcode::XORWF => { write!(f, "xorwf") },
            Opcode::COMF => { write!(f, "comf") },
            Opcode::MULWF => { write!(f, "mulwf") },
            Opcode::ADDWFC => { write!(f, "addwfc") },
            Opcode::ADDWF => { write!(f, "addwf") },
            Opcode::INCF => { write!(f, "incf") },
            Opcode::DECF => { write!(f, "decf") },
            Opcode::DECFSZ => { write!(f, "decfsz") },
            Opcode::RRCF => { write!(f, "rrcf") },
            Opcode::RLCF => { write!(f, "rlcf") },
            Opcode::SWAPF => { write!(f, "swapf") },
            Opcode::INCFSZ => { write!(f, "incfsz") },
            Opcode::RRNCF => { write!(f, "rrncf") },
            Opcode::RLNCF => { write!(f, "rlncf") },
            Opcode::INFSNZ => { write!(f, "infsnz") },
            Opcode::DCFSNZ => { write!(f, "dcfsnz") },
            Opcode::MOVF => { write!(f, "movf") },
            Opcode::SUBFWB => { write!(f, "subfwb") },
            Opcode::SUBWFB => { write!(f, "subwfb") },
            Opcode::SUBWF => { write!(f, "subwf") },
            Opcode::CPFSLT => { write!(f, "cpfslt") },
            Opcode::CPFSEQ => { write!(f, "cpfseq") },
            Opcode::CPFSGT => { write!(f, "cpfsgt") },
            Opcode::TSTFSZ => { write!(f, "tstfsz") },
            Opcode::SETF => { write!(f, "setf") },
            Opcode::CLRF => { write!(f, "clrf") },
            Opcode::NEGF => { write!(f, "negf") },
            Opcode::MOVWF => { write!(f, "movwf") },
            Opcode::BTG => { write!(f, "btg") },
            Opcode::BSF => { write!(f, "bsf") },
            Opcode::BCF => { write!(f, "bcf") },
            Opcode::BTFSS => { write!(f, "btfss") },
            Opcode::BTFSC => { write!(f, "btfsc") },
            Opcode::BZ => { write!(f, "bz") },
            Opcode::BNZ => { write!(f, "bnz") },
            Opcode::BC => { write!(f, "bc") },
            Opcode::BNC => { write!(f, "bnc") },
            Opcode::BOV => { write!(f, "bov") },
            Opcode::BNOV => { write!(f, "bnov") },
            Opcode::BN => { write!(f, "bn") },
            Opcode::BNN => { write!(f, "bnn") },
            Opcode::BRA => { write!(f, "bra") },
            Opcode::RCALL => { write!(f, "rcall") }
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operand::ImmediateU8(imm) => {
                write!(f, "#0x{:x}", imm)
            },
            Operand::ImmediateU32(imm) => {
                write!(f, "#0x{:x}", imm)
            },
            Operand::FileFSR(fsr) => {
                write!(f, "[FSR{}]", fsr)
            },
            Operand::File(file, banked) => {
                if *banked {
                    write!(f, "[banked 0x{:x}]", file)
                } else {
                    write!(f, "[{}]", consts::named_file(
                        if *file < 0x80 {
                            (*file as u16)
                        } else {
                            (*file as u16) | 0xf00u16
                        })
                    )
                }
            },
            Operand::AbsoluteFile(file) => {
                write!(f, "[{}]", consts::named_file(*file))
            },
            Operand::RedirectableFile(file, banked, direction) => {
                if *direction {
                    write!(f, "[todo -> F] ")?
                } else {
                    write!(f, "[todo -> W] ")?
                };

                if *banked {
                    write!(f, "[banked 0x{:x}]", file)
                } else {
                    write!(f, "[{}]", consts::named_file(
                        if *file < 0x80 {
                            (*file as u16)
                        } else {
                            (*file as u16) | 0xf00u16
                        })
                    )
                }
            },
            Operand::Nothing => {
                write!(f, "<No Operand>")
            }
        }
    }
}

