mod ast;
mod irgen;

use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{Result, Write};
use koopa::ir::ValueKind;

use crate::irgen::to_ir;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
    // 输出解析得到的 AST
    // println!("{:#?}", ast);
    // println!("==================");
    // 输出Koopa IR
    let ir = to_ir(&ast);
    // let driver = koopa::front::Driver::from(ir);
    // let program = driver.generate_program().unwrap();
    println!("{}", ir);
    // let mut file = File::create(output).expect("create failed");
    // file.write_all(ir.as_bytes())?;

    let driver = koopa::front::Driver::from(ir);
    let program = driver.generate_program().unwrap();
    let mut asm_code = String::from("  .text\n");
    for &func in program.func_layout() {
        // func打印如下
        // Function(
        //  1,
        // )
        let func_data = program.func(func);
        // TODO 访问函数
        // println!(
        //     "type:{}\nname:{}\nparams:{:?}\n",
        //     func_data.ty(),func_data.name(),func_data.params()
        // );
        let dfg = func_data.dfg();
        let name = &func_data.name()[1..];
        asm_code.push_str(&format!("  .global {}\n",name));
        asm_code.push_str(&format!("{}:\n",name));
        // 遍历基本块
        for (&_bb, node) in func_data.layout().bbs() {
            // 一些必要的处理
            // println!("bb:{:?}\n",bb);
            // 遍历指令列表
            for &inst in node.insts().keys() {
                let value_data = func_data.dfg().value(inst);
                // println!("{:?}",value_data);
                // 访问指令
                match value_data.kind() {
                    ValueKind::Integer(int) => {
                      // 处理 integer 指令
                      println!("{:?}",int);
                    }
                    ValueKind::Return(ret) => {
                      // 处理 ret 指令
                      // 先拿ret的返回值 
                      let value = ret.value().unwrap();
                      let value_data = dfg.value(value);
                      match value_data.kind() {
                        ValueKind::Integer(i) => asm_code.push_str(&format!("  li a0, {}\n",i.value())),
                        _ => unreachable!(),                         
                      };
                      // 
                      asm_code.push_str(&format!("  ret"));
                    }
                    // 其他种类暂时遇不到
                    _ => unreachable!(),
                  }
            }
        }
    }
    println!("{}",asm_code);
    let mut file = File::create(output).expect("create failed");
    file.write_all(asm_code.as_bytes())?;
    Ok(())
}
