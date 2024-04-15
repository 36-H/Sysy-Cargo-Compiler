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
        self.unary_exp.generate(program, scopes)
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