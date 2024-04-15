mod ast;
mod irgen;
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
    let asm_riscv = false;
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input).map_err(Error::File)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();
    // 输出解析得到的 AST
    println!("{:#?}", comp_unit);
    // println!("==================");
    // generate IR
    let program = irgen::generate_program(&comp_unit).map_err(Error::Generate)?;
    let _ = KoopaGenerator::from_path(output)
            .map_err(Error::File)?
            .generate_on(&program)
            .map_err(Error::Io);
    Ok(())
}

/// Error returned by `main` procedure.
enum Error {
    InvalidArgs,
    File(io::Error),
    Parse,
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
            Self::Parse => write!(f, "error occurred while parsing"),
            Self::Generate(err) => write!(f, "{}", err),
            Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}
