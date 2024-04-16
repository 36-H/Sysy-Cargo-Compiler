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
    pub lor: LOrExp,
}
#[derive(Debug)]
pub enum LAndExp {
    Eq(EqExp),
    LAndEq(Box<LAndExp>, EqExp),
}
#[derive(Debug)]
pub enum RelExp {
    Add(AddExp),
    RelAdd(Box<RelExp>, RelOp, AddExp),
}
#[derive(Debug)]
pub enum EqExp {
    Rel(RelExp),
    EqRel(Box<EqExp>, EqOp, RelExp),
}

#[derive(Debug)]
pub enum LOrExp {
    LAnd(LAndExp),
    LOrLAnd(Box<LOrExp>, LAndExp),
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

#[derive(Debug)]
pub enum MulExp {
    Unary(UnaryExp),
    MulUnary(Box<MulExp>, MulOp, UnaryExp),
}

#[derive(Debug)]
pub enum AddExp {
    Mul(MulExp),
    AddMul(Box<AddExp>, AddOp, MulExp),
}

#[derive(Debug)]
pub enum MulOp {
    // *
    Mul,
    // "/"
    Div,
    // "%"
    Mod,
}

#[derive(Debug)]
pub enum AddOp {
    // "+"
    Add,
    // "-"
    Sub,
}

#[derive(Debug)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge,
}
#[derive(Debug)]
pub enum EqOp {
    // "=="
    Eq,
    // "!="
    Neq,
}
