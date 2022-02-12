use generational_arena::{Arena, Index};

#[derive(Debug)]
pub enum Expression {
    Plus(Index, Index),
    Minus(Index, Index),
    Times(Index, Index),
    Div(Index, Index),
    Literal(u32),
    Var(String),
}

#[derive(Debug)]
pub struct Declaration {
    pub id: String,
    pub val: Index,
}

#[derive(Debug)]
pub enum Statement {
    Decl(Index),
    Print(Index),
}

#[derive(Debug)]
pub enum NodeType {
    Statement(Statement),
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug)]
pub struct AstNode {
    val: NodeType,
    declaration: Option<Index>,
}

impl AstNode {
    pub fn new(a: &mut Arena<AstNode>, n: NodeType) -> Index {
        a.insert(AstNode {
            val: n,
            declaration: None,
        })
    }

    pub fn new_statement(a: &mut Arena<AstNode>, s: Statement) -> Index {
        AstNode::new(a, NodeType::Statement(s))
    }
    pub fn new_expression(a: &mut Arena<AstNode>, e: Expression) -> Index {
        AstNode::new(a, NodeType::Expression(e))
    }
    pub fn new_declaration(a: &mut Arena<AstNode>, d: Declaration) -> Index {
        AstNode::new(a, NodeType::Declaration(d))
    }
}
