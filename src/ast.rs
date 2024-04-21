#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<GlobalItem>,
}
#[derive(Debug)]
pub enum GlobalItem {
    Decl(Decl),
    FuncDef(FuncDef),
}
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub params: Vec<FuncFParam>,
    pub block: Block,
}
#[derive(Debug)]
pub struct FuncFParam {
    pub id: String,
    pub dims: Option<Vec<ConstExp>>,
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
    ExpStmt(ExpStmt),
    Block(Block),
    If(Box<If>),
    While(Box<While>),
    Break(Break),
    Continue(Continue),
}
#[derive(Debug)]
pub struct Break;
#[derive(Debug)]
pub struct Continue;

#[derive(Debug)]
pub struct While {
    pub cond: Exp,
    pub body: Stmt,
}

#[derive(Debug)]
pub struct If {
    pub cond: Exp,
    pub then: Stmt,
    pub else_then: Option<Stmt>,
}

#[derive(Debug)]
pub struct ExpStmt {
    pub exp: Option<Exp>,
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
    Call(FuncCall),
    Unary(UnaryOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub struct FuncCall {
    pub id: String,
    pub args: Vec<Exp>,
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
