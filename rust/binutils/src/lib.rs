// Guillaume Valadon <guillaume@valadon.net>

extern crate libc;

use libc::{c_char, c_int, c_ulong};

use std::ffi::{CStr, CString};
use std::ptr;


// libbfd bindings

// Rust bfd types
// Note: - trick from https://doc.rust-lang.org/nomicon/ffi.html
//       - it allows to use the Rust type checker
pub enum BfdRaw {}

pub struct Bfd {
    bfd: *const BfdRaw,
}

impl Bfd {
    pub fn new() -> Bfd {
        unsafe { bfd_init() };
        Bfd { bfd: ptr::null() }
    }

    pub fn raw(&self) -> *const BfdRaw {
        self.bfd
    }

    pub fn openr(&mut self, filename: &str, target: &str) {
        // TODO: check results!
        let filename_cstring = CString::new(filename).unwrap();
        let target_cstring = CString::new(target).unwrap();

        let bfd_file = unsafe { bfd_openr(filename_cstring.as_ptr(), target_cstring.as_ptr()) };
        if bfd_file.is_null() {
            let error = unsafe { bfd_get_error() };
            let msg = unsafe { bfd_errmsg(error) };
            println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
            return;
        }
        self.bfd = bfd_file;
    }

    pub fn check_format(&self, format: BfdFormat) {
        // TODO: check results!
        if !unsafe { bfd_check_format(self.bfd, format) } {
            let error = unsafe { bfd_get_error() };
            let msg = unsafe { bfd_errmsg(error) };
            println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
            return;
        }
    }

    pub fn get_section_by_name(&self, section_name: &str) -> *const Section {
        let section_name_cstring = CString::new(section_name).unwrap();
        unsafe { bfd_get_section_by_name(self.bfd, section_name_cstring.as_ptr()) }
    }

    pub fn disassembler(
        &self,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfoRaw) -> c_ulong {
        unsafe { disassembler(self.bfd) }
    }

    pub fn get_start_address(&self) -> c_ulong {
        unsafe { get_start_address(self.bfd) }
    }
}

#[allow(non_camel_case_types)] // use the same enum names as libbfd
#[allow(dead_code)] // don't warn that some variants are not used
#[repr(C)]
pub enum BfdFormat {
    bfd_unknown = 0,
    bfd_object,
    bfd_archive,
    bfd_core,
    bfd_type_end,
}


pub enum Section {}

#[link(name = "bfd-2.28-multiarch")]
extern "C" {
    fn bfd_init();

    pub fn bfd_get_error() -> c_int;
    pub fn bfd_errmsg(error_tag: c_int) -> *const c_char;

    pub fn bfd_openr(filename: *const c_char, target: *const c_char) -> *const BfdRaw;

    pub fn bfd_check_format(bfd: *const BfdRaw, bfd_format: BfdFormat) -> bool;

    pub fn bfd_get_section_by_name(bfd: *const BfdRaw, name: *const c_char) -> *const Section;

/*
 * binutils 2.29.1
    fn bfd_get_arch(bfd: *const c_int) -> c_int;
    fn bfd_get_mach(bfd: *const c_int) -> c_long;
*/
}


// libopcodes bindings
pub enum DisassembleInfoRaw {}

pub struct DisassembleInfo {
    info: *const DisassembleInfoRaw,
}

impl DisassembleInfo {
    pub fn new() -> DisassembleInfo {
        let new_info = unsafe { new_disassemble_info() };
        DisassembleInfo { info: new_info }
    }

    pub fn raw(&self) -> *const DisassembleInfoRaw {
        self.info
    }

    pub fn configure(&self, section: *const Section, bfd: *const BfdRaw) {
        unsafe { configure_disassemble_info(self.info, section, bfd) }
    }

    pub fn init(&self) {
        unsafe { disassemble_init_for_target(self.info) };
    }

    pub fn set_print_address_func(
        &self,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    ) {
        unsafe { set_print_address_func(self.info, print_function) }
    }

}

#[link(name = "opcodes-2.28-multiarch")]
extern "C" {
    pub fn disassembler(
        bfd: *const BfdRaw,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfoRaw) -> c_ulong;
    /*
     * binutils 2.29.1
    fn disassembler(arc: c_int, big: bool, mach: c_long, bfd: *const c_int) -> *const c_int;
    */
    fn disassemble_init_for_target(dinfo: *const DisassembleInfoRaw);
}


// Custom bindings that ease disassembler_info manipulation
extern "C" {
    fn new_disassemble_info() -> *const DisassembleInfoRaw;
    fn configure_disassemble_info(
        info: *const DisassembleInfoRaw,
        section: *const Section,
        bfd: *const BfdRaw,
    );
    fn get_start_address(bfd: *const BfdRaw) -> c_ulong;
    pub fn get_section_size(section: *const Section) -> c_ulong;
    fn set_print_address_func(
        info: *const DisassembleInfoRaw,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    );

    pub static tmp_buf_asm: [u8; 64];
    pub static mut tmp_buf_asm_ptr: *mut c_char;
}


pub fn get_instruction() -> String { // Result<String, Error>
    // Return a String that represents the disassembled instruction

    // Look for the first nul byte in the array
    let mut buffer_itr = unsafe { tmp_buf_asm.iter() };
    let index_opt = buffer_itr.position(|&c| c == 0);

    let index = match index_opt {
        Some(i) => i,
        None => return String::from("No nul byte found in disassembly result!"), // TODO: error
    };

    // Extract the instruction String
    let instruction = unsafe { CStr::from_bytes_with_nul_unchecked(&tmp_buf_asm[0..index + 1]) };
    String::from(instruction.to_str().unwrap())
}