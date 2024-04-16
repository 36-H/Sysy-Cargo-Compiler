use crate::{ast::*, cur_func, cur_func_mut};
use koopa::ir::{builder::*, BinaryOp};
use koopa::ir::{FunctionData, Program, Type};

use super::func::FunctionInfo;
use super::scopes::Scopes;
use super::values::ExpValue;
use super::{Error, Result};

pub trait GenerateProgram<'ast> {
    type Out;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out>;
}

impl<'ast> GenerateProgram<'ast> for CompUnit {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        let func_def = &self.func_def;
        func_def.generate(program, scopes)?;
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for FuncDef {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        let params_ty = vec![];
        let ret_ty = self.func_type.generate(program, scopes)?;
        // create new fucntion
        let mut data = FunctionData::new(format!("@{}", self.ident), params_ty, ret_ty);
        // generate entry block
        let entry = data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let end = data.dfg_mut().new_bb().basic_block(Some("%end".into()));
        let cur = data.dfg_mut().new_bb().basic_block(None);
        // generate return value
        let mut ret_val = None;
        if matches!(self.func_type, FuncType::Int) {
            let alloc = data.dfg_mut().new_value().alloc(Type::get_i32());
            data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
            ret_val = Some(alloc);
        }
        // update function information
        let func = program.new_func(data);
        let mut info = FunctionInfo::new(func, entry, end, ret_val);
        info.push_bb(program, entry);
        if let Some(ret_val) = info.ret_val() {
            info.push_inst(program, ret_val);
        }
        info.push_bb(program, cur);
        scopes.enter();
        // update scope
        scopes.new_func(&self.ident, func)?;
        scopes.cur_func = Some(info);
        // generate function body
        self.block.generate(program, scopes)?;
        scopes.exit();
        // handle end basic block
        let mut info = scopes.cur_func.take().unwrap();
        info.seal_entry(program, cur);
        info.seal_func(program);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for FuncType {
    type Out = Type;

    fn generate(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            Self::Void => Type::get_unit(),
            Self::Int => Type::get_i32(),
        })
    }
}

impl<'ast> GenerateProgram<'ast> for Block {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        scopes.enter();
        self.stmt.generate(program, scopes)?;
        scopes.exit();
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Stmt {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Return(s) => s.generate(program, scopes),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for Return {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        if let Some(ret_val) = cur_func!(scopes).ret_val() {
            // generate store
            if let Some(val) = &self.exp {
                let value = val.generate(program, scopes)?.into_int(program, scopes)?;
                let info = cur_func!(scopes);
                let store = info.new_value(program).store(value, ret_val);
                info.push_inst(program, store);
            }
        } else if self.exp.is_some() {
            return Err(Error::RetValInVoidFunc);
        }
        // jump to the end basic block
        let info = &mut cur_func_mut!(scopes);
        let jump = info.new_value(program).jump(info.end());
        info.push_inst(program, jump);
        // push new basic block
        let next = info.new_bb(program, None);
        info.push_bb(program, next);
        Ok(())
    }
}

impl<'ast> GenerateProgram<'ast> for Exp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        self.lor.generate(program, scopes)
    }
}

impl<'ast> GenerateProgram<'ast> for PrimaryExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Exp(exp) => exp.generate(program, scopes),
            Self::Number(num) => Ok(ExpValue::Int(
                cur_func!(scopes).new_value(program).integer(*num),
            )),
        }
    }
}

impl<'ast> GenerateProgram<'ast> for UnaryExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Primary(exp) => exp.generate(program, scopes),
            Self::Unary(op, exp) => {
                let exp = exp.generate(program, scopes)?.into_int(program, scopes)?;
                let info = cur_func!(scopes);
                let zero = info.new_value(program).integer(0);
                let value = match op {
                    UnaryOp::Neg => info.new_value(program).binary(BinaryOp::Sub, zero, exp),
                    UnaryOp::LNot => info.new_value(program).binary(BinaryOp::Eq, exp, zero),
                };
                info.push_inst(program, value);
                Ok(ExpValue::Int(value))
            }
        }
    }
}

/// Generates logical operators.
macro_rules! generate_logical_ops {
    (
      $lhs:expr, $rhs:expr, $program:expr, $scopes:expr,
      $prefix:literal, $rhs_bb:ident, $end_bb:ident, $tbb:ident, $fbb:ident
    ) => {{
        // generate result
        let result = cur_func!($scopes).new_alloc($program, Type::get_i32(), None);
        // generate left-hand side expression
        let lhs = $lhs
            .generate($program, $scopes)?
            .into_int($program, $scopes)?;
        let info = cur_func_mut!($scopes);
        let zero = info.new_value($program).integer(0);
        let lhs = info.new_value($program).binary(BinaryOp::NotEq, lhs, zero);
        info.push_inst($program, lhs);
        let store = info.new_value($program).store(lhs, result);
        info.push_inst($program, store);
        // generate basic blocks and branch
        let $rhs_bb = info.new_bb($program, Some(concat!("%", $prefix, "_rhs")));
        let $end_bb = info.new_bb($program, Some(concat!("%", $prefix, "_end")));
        let br = info.new_value($program).branch(lhs, $tbb, $fbb);
        info.push_inst($program, br);
        // generate right-hand side expression
        info.push_bb($program, $rhs_bb);
        let rhs = $rhs
            .generate($program, $scopes)?
            .into_int($program, $scopes)?;
        let info = cur_func_mut!($scopes);
        let rhs = info.new_value($program).binary(BinaryOp::NotEq, rhs, zero);
        info.push_inst($program, rhs);
        let store = info.new_value($program).store(rhs, result);
        info.push_inst($program, store);
        // generate jump
        let jump = info.new_value($program).jump($end_bb);
        info.push_inst($program, jump);
        info.push_bb($program, $end_bb);
        // generate load
        let load = info.new_value($program).load(result);
        info.push_inst($program, load);
        Ok(ExpValue::Int(load))
    }};
}

impl<'ast> GenerateProgram<'ast> for LOrExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::LAnd(exp) => exp.generate(program, scopes),
            Self::LOrLAnd(lhs, rhs) => generate_logical_ops! {
              lhs, rhs, program, scopes, "lor", rhs_bb, end_bb, end_bb, rhs_bb
            },
        }
    }
}

impl<'ast> GenerateProgram<'ast> for LAndExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Eq(exp) => exp.generate(program, scopes),
            Self::LAndEq(lhs, rhs) => generate_logical_ops! {
              lhs, rhs, program, scopes, "land", rhs_bb, end_bb, rhs_bb, end_bb
            },
        }
    }
}

impl<'ast> GenerateProgram<'ast> for EqOp {
    type Out = BinaryOp;

    fn generate(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            EqOp::Eq => BinaryOp::Eq,
            EqOp::Neq => BinaryOp::NotEq,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for EqExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Rel(exp) => exp.generate(program, scopes),
            Self::EqRel(lhs, op, rhs) => {
                let lhs = lhs.generate(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.generate(program, scopes)?.into_int(program, scopes)?;
                let op = op.generate(program, scopes)?;
                let info = cur_func!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExpValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for RelExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Add(exp) => exp.generate(program, scopes),
            Self::RelAdd(lhs, op, rhs) => {
                let lhs = lhs.generate(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.generate(program, scopes)?.into_int(program, scopes)?;
                let op = op.generate(program, scopes)?;
                let info = cur_func!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExpValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for AddExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Mul(exp) => exp.generate(program, scopes),
            Self::AddMul(lhs, op, rhs) => {
                let lhs = lhs.generate(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.generate(program, scopes)?.into_int(program, scopes)?;
                let op = op.generate(program, scopes)?;
                let info = cur_func!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExpValue::Int(value))
            }
        }
    }
}

impl<'ast> GenerateProgram<'ast> for AddOp {
    type Out = BinaryOp;

    fn generate(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            AddOp::Add => BinaryOp::Add,
            AddOp::Sub => BinaryOp::Sub,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for RelOp {
    type Out = BinaryOp;

    fn generate(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            RelOp::Lt => BinaryOp::Lt,
            RelOp::Gt => BinaryOp::Gt,
            RelOp::Le => BinaryOp::Le,
            RelOp::Ge => BinaryOp::Ge,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for MulOp {
    type Out = BinaryOp;

    fn generate(&'ast self, _: &mut Program, _: &mut Scopes<'ast>) -> Result<Self::Out> {
        Ok(match self {
            MulOp::Mul => BinaryOp::Mul,
            MulOp::Div => BinaryOp::Div,
            MulOp::Mod => BinaryOp::Mod,
        })
    }
}

impl<'ast> GenerateProgram<'ast> for MulExp {
    type Out = ExpValue;

    fn generate(&'ast self, program: &mut Program, scopes: &mut Scopes<'ast>) -> Result<Self::Out> {
        match self {
            Self::Unary(exp) => exp.generate(program, scopes),
            Self::MulUnary(lhs, op, rhs) => {
                let lhs = lhs.generate(program, scopes)?.into_int(program, scopes)?;
                let rhs = rhs.generate(program, scopes)?.into_int(program, scopes)?;
                let op = op.generate(program, scopes)?;
                let info = cur_func!(scopes);
                let value = info.new_value(program).binary(op, lhs, rhs);
                info.push_inst(program, value);
                Ok(ExpValue::Int(value))
            }
        }
    }
}
