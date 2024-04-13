use crate::ast::CompUnit;

pub fn to_ir(ast: &CompUnit) -> String {
    format!(
        "fun @{}(): i32 {{\n%entry:\n\tret {}\n}}",
        ast.func_def.ident, ast.func_def.block.stmt.num
    )
}
