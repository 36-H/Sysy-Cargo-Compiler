#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}
#[derive(Debug)]
pub enum FuncType {
    Void,
    Int,
}
#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Stmt {
    Return(Return),
}
#[derive(Debug)]
pub struct Return {
    pub exp: Option<Exp>,
}
#[derive(Debug)]
pub struct Exp {
    pub unary_exp: UnaryExp,
}
#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
    Unary(UnaryOp, Box<UnaryExp>),
}
#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(i32),
}
#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    LNot,
}
