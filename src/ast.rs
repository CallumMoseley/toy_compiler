use slotmap::{new_key_type, SlotMap};
use std::collections::HashMap;

new_key_type! {
    pub struct ExpressionKey;
    pub struct DeclarationKey;
    pub struct StatementKey;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NodeKey {
    Expression(ExpressionKey),
    Declaration(DeclarationKey),
    Statement(StatementKey),
}

#[derive(Debug)]
pub enum Expression {
    Plus(ExpressionKey, ExpressionKey),
    Minus(ExpressionKey, ExpressionKey),
    Times(ExpressionKey, ExpressionKey),
    Div(ExpressionKey, ExpressionKey),
    Literal(u32),
    Var(String),
}

#[derive(Debug)]
pub struct Declaration {
    pub id: String,
    pub val: ExpressionKey,
}

#[derive(Debug)]
pub enum Statement {
    Decl(DeclarationKey),
    Print(ExpressionKey),
}

#[derive(Debug)]
pub struct Ast {
    pub exprs: SlotMap<ExpressionKey, Expression>,
    pub decls: SlotMap<DeclarationKey, Declaration>,
    pub stmts: SlotMap<StatementKey, Statement>,

    pub name_resolution: HashMap<NodeKey, DeclarationKey>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            exprs: SlotMap::with_key(),
            decls: SlotMap::with_key(),
            stmts: SlotMap::with_key(),
            name_resolution: HashMap::new(),
        }
    }
    pub fn insert_expr(&mut self, e: Expression) -> ExpressionKey {
        self.exprs.insert(e)
    }
    pub fn insert_decl(&mut self, d: Declaration) -> DeclarationKey {
        self.decls.insert(d)
    }
    pub fn insert_stmt(&mut self, s: Statement) -> StatementKey {
        self.stmts.insert(s)
    }
}
