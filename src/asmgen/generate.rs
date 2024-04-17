use crate::asm_value;

use super::builder::AsmBuilder;
use super::info::ProgramInfo;
use super::values::AsmValue;
use asmgen::func::FunctionInfo;
use koopa::ir::entities::ValueData;
use koopa::ir::values::{Binary, Branch, Jump, Load, Return, Store};
use koopa::ir::{BasicBlock, BinaryOp, Function, FunctionData, Program, Value, ValueKind};
use std::fs::File;
use std::io::{Result, Write};
/// Trait for generating RISC-V assembly.
pub trait GenerateAsm<'p, 'i> {
    type Out;

    fn generate(&self, file: &mut File, program_info: &'i mut ProgramInfo<'p>)
        -> Result<Self::Out>;
}

/// Trait for generating RISC-V assembly (for values).
trait GenerateValueAsm<'p, 'i> {
    type Out;

    fn generate(
        &self,
        f: &mut File,
        info: &'i mut ProgramInfo<'p>,
        v: &ValueData,
    ) -> Result<Self::Out>;
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Program {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        // generate global allocations
        for &value in self.inst_layout() {
            let data = self.borrow_value(value);
            let name = &data.name().as_ref().unwrap()[1..];
            info.insert_value(value, name.into());
            writeln!(f, "  .data")?;
            writeln!(f, "  .globl {name}")?;
            writeln!(f, "{name}:")?;
            data.generate(f, info)?;
            writeln!(f)?;
        }
        // generate functions
        for &func in self.func_layout() {
            info.set_cur_func(FunctionInfo::new(func));
            self.func(func).generate(f, info)?;
        }
        Ok(())
    }
}
impl<'p, 'i> GenerateAsm<'p, 'i> for Function {
    type Out = &'p str;

    fn generate(&self, _: &mut File, info: &mut ProgramInfo<'p>) -> Result<Self::Out> {
        Ok(&info.program().func(*self).name()[1..])
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for FunctionData {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        // skip declarations
        if self.layout().entry_bb().is_none() {
            return Ok(());
        }
        // allocation stack slots and log argument number
        let func = asm_cur_func_mut!(info);
        for value in self.dfg().values().values() {
            // allocate stack slot
            if value.kind().is_local_inst() && !value.used_by().is_empty() {
                func.alloc_slot(value);
            }
            // log argument number
            if let ValueKind::Call(call) = value.kind() {
                func.log_arg_num(call.args().len());
            }
        }
        // generate basic block names
        for (&bb, data) in self.dfg().bbs() {
            // basic block parameters are not supported
            assert!(data.params().is_empty());
            func.log_bb_name(bb, data.name());
        }
        // generate prologue
        AsmBuilder::new(f, "t0").prologue(self.name(), func)?;
        // generate instructions in basic blocks
        for (bb, node) in self.layout().bbs() {
            let name = bb.generate(f, info)?;
            writeln!(f, "{name}:")?;
            for &inst in node.insts().keys() {
                self.dfg().value(inst).generate(f, info)?;
            }
        }
        writeln!(f)
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for BasicBlock {
    type Out = &'i str;

    fn generate(&self, _: &mut File, info: &'i mut ProgramInfo) -> Result<Self::Out> {
        Ok(asm_cur_func!(info).bb_name(*self))
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for ValueData {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        println!("{:#?}", self.kind());
        match self.kind() {
            ValueKind::Return(v) => v.generate(f, info),
            ValueKind::Alloc(_) => Ok(()),
            ValueKind::Jump(v) => v.generate(f, info),
            ValueKind::Store(v) => v.generate(f, info),
            ValueKind::Load(v) => v.generate(f, info, self),
            ValueKind::Binary(v) => v.generate(f, info, self),
            ValueKind::Branch(v) => v.generate(f, info),
            _ => unimplemented!(),
        }
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Return {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        if let Some(value) = self.value() {
            value.generate(f, info)?.write_to(f, "a0")?;
        }
        AsmBuilder::new(f, "t0").epilogue(asm_cur_func!(info))
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Value {
    type Out = AsmValue<'i>;

    fn generate(&self, _: &mut File, info: &'i mut ProgramInfo) -> Result<Self::Out> {
        if self.is_global() {
            Ok(AsmValue::Global(info.value(*self)))
        } else {
            let func = asm_cur_func!(info);
            let value = info.program().func(func.func()).dfg().value(*self);
            Ok(match value.kind() {
                ValueKind::Integer(i) => AsmValue::Const(i.value()),
                ValueKind::FuncArgRef(i) => AsmValue::Arg(i.index()),
                _ => AsmValue::from(func.slot_offset(value)),
            })
        }
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Jump {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        let label = self.target().generate(f, info)?;
        AsmBuilder::new(f, "t0").j(label)
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Store {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        let sp_offset = asm_cur_func!(info).sp_offset();
        let value = self.value().generate(f, info)?;
        if matches!(value, AsmValue::Arg(_)) {
            value.write_arg_to(f, "t0", sp_offset)?;
        } else {
            value.write_to(f, "t0")?;
        }
        let dest = self.dest().generate(f, info)?;
        if dest.is_ptr() {
            dest.write_to(f, "t1")?;
            AsmBuilder::new(f, "t2").sw("t0", "t1", 0)
        } else {
            dest.read_from(f, "t0", "t1")
        }
    }
}

impl<'p, 'i> GenerateValueAsm<'p, 'i> for Load {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo, v: &ValueData) -> Result<Self::Out> {
        let src = self.src().generate(f, info)?;
        src.write_to(f, "t0")?;
        if src.is_ptr() {
            AsmBuilder::new(f, "t1").lw("t0", "t0", 0)?;
        }
        asm_value!(info, v).read_from(f, "t0", "t1")
    }
}

impl<'p, 'i> GenerateValueAsm<'p, 'i> for Binary {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo, v: &ValueData) -> Result<Self::Out> {
        self.lhs().generate(f, info)?.write_to(f, "t0")?;
        self.rhs().generate(f, info)?.write_to(f, "t1")?;
        let mut builder = AsmBuilder::new(f, "t2");
        match self.op() {
            BinaryOp::NotEq => {
                builder.op2("xor", "t0", "t0", "t1")?;
                builder.op1("snez", "t0", "t0")?;
            }
            BinaryOp::Eq => {
                builder.op2("xor", "t0", "t0", "t1")?;
                builder.op1("seqz", "t0", "t0")?;
            }
            BinaryOp::Gt => builder.op2("sgt", "t0", "t0", "t1")?,
            BinaryOp::Lt => builder.op2("slt", "t0", "t0", "t1")?,
            BinaryOp::Ge => {
                builder.op2("slt", "t0", "t0", "t1")?;
                builder.op1("seqz", "t0", "t0")?;
            }
            BinaryOp::Le => {
                builder.op2("sgt", "t0", "t0", "t1")?;
                builder.op1("seqz", "t0", "t0")?;
            }
            BinaryOp::Add => builder.op2("add", "t0", "t0", "t1")?,
            BinaryOp::Sub => builder.op2("sub", "t0", "t0", "t1")?,
            BinaryOp::Mul => builder.op2("mul", "t0", "t0", "t1")?,
            BinaryOp::Div => builder.op2("div", "t0", "t0", "t1")?,
            BinaryOp::Mod => builder.op2("rem", "t0", "t0", "t1")?,
            BinaryOp::And => builder.op2("and", "t0", "t0", "t1")?,
            BinaryOp::Or => builder.op2("or", "t0", "t0", "t1")?,
            BinaryOp::Xor => builder.op2("xor", "t0", "t0", "t1")?,
            BinaryOp::Shl => builder.op2("sll", "t0", "t0", "t1")?,
            BinaryOp::Shr => builder.op2("srl", "t0", "t0", "t1")?,
            BinaryOp::Sar => builder.op2("sra", "t0", "t0", "t1")?,
        }
        asm_value!(info, v).read_from(f, "t0", "t1")
    }
}

impl<'p, 'i> GenerateAsm<'p, 'i> for Branch {
    type Out = ();

    fn generate(&self, f: &mut File, info: &mut ProgramInfo) -> Result<Self::Out> {
        self.cond().generate(f, info)?.write_to(f, "t0")?;
        let tlabel = self.true_bb().generate(f, info)?;
        AsmBuilder::new(f, "t1").bnez("t0", tlabel)?;
        let flabel = self.false_bb().generate(f, info)?;
        AsmBuilder::new(f, "t1").j(flabel)
    }
}
