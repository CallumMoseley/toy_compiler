use crate::ast::*;
use std::default::Default;

pub trait Visitor<T: Default, E> {
    fn visit_expression(&mut self, ast: &mut Ast, e: ExpressionKey) -> Result<T, E> {
        walk_expression(self, ast, e)
    }
    fn visit_declaration(&mut self, ast: &mut Ast, d: DeclarationKey) -> Result<T, E> {
        walk_declaration(self, ast, d)
    }
    fn visit_statement(&mut self, ast: &mut Ast, s: StatementKey) -> Result<T, E> {
        walk_statement(self, ast, s)
    }
    fn visit_program(&mut self, ast: &mut Ast, p: &Vec<StatementKey>) -> Result<T, E> {
        walk_program(self, ast, p)
    }
}

pub fn walk_expression<T: Default, E, V: Visitor<T, E> + ?Sized>(
    visitor: &mut V,
    ast: &mut Ast,
    e: ExpressionKey,
) -> Result<T, E> {
    match ast.exprs[e] {
        Expression::Plus(a, b)
        | Expression::Minus(a, b)
        | Expression::Times(a, b)
        | Expression::Div(a, b) => {
            visitor.visit_expression(ast, a)?;
            visitor.visit_expression(ast, b)
        }
        _ => Ok(T::default()),
    }
}

pub fn walk_declaration<T: Default, E, V: Visitor<T, E> + ?Sized>(
    visitor: &mut V,
    ast: &mut Ast,
    d: DeclarationKey,
) -> Result<T, E> {
    visitor.visit_expression(ast, ast.decls[d].val)
}

pub fn walk_statement<T: Default, E, V: Visitor<T, E> + ?Sized>(
    visitor: &mut V,
    ast: &mut Ast,
    s: StatementKey,
) -> Result<T, E> {
    match ast.stmts[s] {
        Statement::Decl(d) => visitor.visit_declaration(ast, d),
        Statement::Print(e) => visitor.visit_expression(ast, e),
    }
}

pub fn walk_program<T: Default, E, V: Visitor<T, E> + ?Sized>(
    visitor: &mut V,
    ast: &mut Ast,
    p: &Vec<StatementKey>,
) -> Result<T, E> {
    for sk in p {
        visitor.visit_statement(ast, *sk)?;
    }
    Ok(T::default())
}
