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
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Stmt {
    Assign(Assign),
    Return(Return),
}
#[derive(Debug)]
pub struct Assign {
    pub lval: LVal,
    pub exp: Exp,
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
    LVal(LVal),
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

#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub struct ConstDef {
    pub id: String,
    pub dims: Vec<ConstExp>,
    pub init: ConstInitVal,
}
#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}
#[derive(Debug)]
pub enum ConstInitVal {
    Exp(ConstExp),
}
#[derive(Debug)]
pub struct VarDecl {
    pub defs: Vec<VarDef>,
}
#[derive(Debug)]
pub struct VarDef {
    pub id: String,
    pub dims: Vec<ConstExp>,
    pub init: Option<InitVal>,
}
#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
}
#[derive(Debug)]
pub struct LVal {
    pub id: String,
    pub indices: Vec<Exp>,
}
