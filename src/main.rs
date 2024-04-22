mod ast;
#[macro_use]
mod irgen;
#[macro_use]
mod asmgen;
extern crate koopa;
extern crate lalrpop_util;

use koopa::back::KoopaGenerator;
use koopa::ir::ValueKind;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::{self, Write};
use std::process::exit;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() {
    if let Err(err) = try_compile() {
        eprintln!("{}", err);
        exit(-1);
    }
}

fn try_compile() -> Result<(), Error> {
    //解析命令行参数
    let (mode, input, output) = parse_args()?;

    // 读取输入文件
    let input = read_to_string(input).map_err(Error::File)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();
    // 输出解析得到的 AST
    // println!("{:#?}", comp_unit);
    // println!("==================");
    // generate IR
    let program = irgen::generate_program(&comp_unit).map_err(Error::Generate)?;
    if matches!(mode, Mode::Koopa) {
        return KoopaGenerator::from_path(output.clone())
            .map_err(Error::File)?
            .generate_on(&program)
            .map_err(Error::Io);
    }
    // generate RISC-V assembly
    asmgen::generate_asm(&program, &output).map_err(Error::Io)
}

/// Error returned by `main` procedure.
enum Error {
    InvalidArgs,
    File(io::Error),
    Generate(irgen::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidArgs => write!(
                f,
                r#"Usage: kira MODE INPUT -o OUTPUT

Options:
  MODE:   can be `-koopa`, `-riscv` or `-perf`
  INPUT:  the input SysY source file
  OUTPUT: the output file"#
            ),
            Self::File(err) => write!(f, "invalid input SysY file: {}", err),
            Self::Generate(err) => write!(f, "{}", err),
            Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

enum Mode {
    /// Compile SysY to Koopa IR.
    Koopa,
    /// Compile SysY to RISC-V assembly.
    Riscv,
}

/// Parses the arguments, returns `Error` if error occurred.
fn parse_args() -> Result<(Mode, String, String), Error> {
    let mut args = args();
    args.next();
    match (args.next(), args.next(), args.next(), args.next()) {
        (Some(m), Some(input), Some(o), Some(output)) if o == "-o" => {
            let mode = match m.as_str() {
                "-koopa" => Mode::Koopa,
                "-riscv" => Mode::Riscv,
                _ => return Err(Error::InvalidArgs),
            };
            Ok((mode, input, output))
        }
        _ => Err(Error::InvalidArgs),
    }
}
