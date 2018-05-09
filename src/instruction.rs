// Guillaume Valadon <guillaume@valadon.net>
// binutils - instruction.rs

use std::cmp;
use std::ffi::CStr;
use std::fmt;

use Error;
use bfd::Bfd;
use helpers;
use opcodes::DisassembleInfo;

#[allow(dead_code)]
pub struct Instruction<'a> {
    pub length: u64,
    pub offset: u64,
    pub opcode: &'a str,
    info: Option<&'a mut DisassembleInfo>,
    pub error: Option<Error>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X} {}", self.offset, self.opcode)
    }
}

pub(crate) fn get_opcode<'a>() -> Result<&'a str, Error> {
    // Compute the index of the first nul byte in the array
    let index = unsafe {
        let addr_end = helpers::buffer_asm_ptr as usize;
        let addr_start = (&helpers::buffer_asm as *const u8) as usize;
        match addr_end.checked_sub(addr_start) {
            Some(i) => i,
            None => return Err(Error::CommonError("checked_sub() failed!".to_string())),
        }
    };

    if index == 0 {
        return Err(Error::CommonError("opcode index is 0!".to_string()));
    }

    // Extract the instruction string
    let opcode_raw =
        unsafe { CStr::from_bytes_with_nul(&helpers::buffer_asm[0..cmp::min(index, 63) + 1]) };
    Ok(opcode_raw?.to_str()?)
}

pub fn get_instruction<'a>(offset: u64, length: u64) -> Result<Instruction<'a>, Error> {
    Ok(Instruction {
        offset,
        length,
        opcode: get_opcode()?,
        info: None,
        error: None,
    })
}

impl<'a> Instruction<'a> {
    pub fn empty_with_error(error: Option<Error>) -> Instruction<'a> {
        Instruction {
            offset: 0,
            length: 0,
            opcode: "",
            info: None,
            error,
        }
    }
    pub fn from_buffer(
        info: &'a mut DisassembleInfo,
        bfd: Bfd,
        buffer: &[u8],
        offset: u64,
    ) -> Instruction<'a> {
        match info.init_buffer(buffer, bfd, offset) {
            Ok(_) => (),
            Err(e) => return Instruction::empty_with_error(Some(e)),
        };

        Instruction {
            offset: 0,
            length: 0,
            opcode: "",
            info: Some(info),
            error: None,
        }
    }
}

impl<'a> Iterator for Instruction<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Temporarily remove info from the structure
        let info = match self.info.take() {
            Some(i) => i,
            None => {
                return Some(Instruction::empty_with_error(Some(
                    Error::DisassembleInfoError("empty".to_string()),
                )))
            }
        };

        let i = info.disassemble();
        match i {
            Some(r) => match r {
                Ok(i) => {
                    self.info = Some(info);
                    Some(i)
                }
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_init() {
        use instruction;

        assert!(instruction::get_opcode().is_err());
        assert!(instruction::get_instruction(0, 0).is_err());
    }
}
