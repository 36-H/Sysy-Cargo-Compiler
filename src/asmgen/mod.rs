#[macro_use]
mod info;
mod func;
mod generate;
#[macro_use]
mod values;
mod builder;

use koopa::ir::{Program, Type};
use std::fs::File;
use std::io::Result;

use self::generate::GenerateAsm;
use self::info::ProgramInfo;
/// from Koopa IR program to RISC-V assembly.
pub fn generate_asm(program: &Program, path: &str) -> Result<()> {
    Type::set_ptr_size(4);
    program.generate(&mut File::create(path)?, &mut ProgramInfo::new(program))
}
