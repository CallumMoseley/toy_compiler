#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub toy);

mod ast;
mod visit;

use ast::*;
use visit::*;

use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::mem::swap;

struct NameResolver {
    names: HashMap<String, DeclarationKey>,
}

impl Visitor<(), Error> for NameResolver {
    fn visit_expression(&mut self, ast: &mut Ast, e: ExpressionKey) -> Result<()> {
        if let Expression::Var(id) = &ast.exprs[e] {
            if let Some(decl) = self.names.get(id) {
                ast.name_resolution.insert(NodeKey::Expression(e), *decl);
            } else {
                return Err(anyhow!("Failed to look up name '{}'", id));
            }
        }
        walk_expression(self, ast, e)
    }
    fn visit_declaration(&mut self, ast: &mut Ast, d: DeclarationKey) -> Result<()> {
        let decl = &ast.decls[d];
        self.names.insert(decl.id.clone(), d);
        walk_declaration(self, ast, d)
    }
}

struct ConstFolder;
impl Visitor<(), Error> for ConstFolder {
    fn visit_expression(&mut self, ast: &mut Ast, e: ExpressionKey) -> Result<()> {
        walk_expression(self, ast, e)?;
        let lits = match ast.exprs[e] {
            Expression::Plus(a, b)
            | Expression::Minus(a, b)
            | Expression::Times(a, b)
            | Expression::Div(a, b) => {
                if let Expression::Literal(a) = ast.exprs[a] {
                    if let Expression::Literal(b) = ast.exprs[b] {
                        Some((a, b))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        let replacement = match (&ast.exprs[e], lits) {
            (Expression::Plus(_, _), Some((a, b))) => Some(Expression::Literal(a + b)),
            (Expression::Minus(_, _), Some((a, b))) => Some(Expression::Literal(a - b)),
            (Expression::Times(_, _), Some((a, b))) => Some(Expression::Literal(a * b)),
            (Expression::Div(_, _), Some((a, b))) => Some(Expression::Literal(a / b)),
            _ => None,
        };
        let expr = &mut ast.exprs[e];
        if let Some(mut replacement) = replacement {
            swap(expr, &mut replacement);
        }
        Ok(())
    }
}

struct Printer;
impl Visitor<(), Error> for Printer {
    fn visit_expression(&mut self, ast: &mut Ast, e: ExpressionKey) -> Result<()> {
        match ast.exprs[e] {
            Expression::Plus(a, b) => {
                print!("(");
                self.visit_expression(ast, a)?;
                print!(" + ");
                self.visit_expression(ast, b)?;
                print!(")");
            }
            Expression::Minus(a, b) => {
                print!("(");
                self.visit_expression(ast, a)?;
                print!(" - ");
                self.visit_expression(ast, b)?;
                print!(")");
            }
            Expression::Times(a, b) => {
                self.visit_expression(ast, a)?;
                print!(" * ");
                self.visit_expression(ast, b)?;
            }
            Expression::Div(a, b) => {
                self.visit_expression(ast, a)?;
                print!(" / ");
                self.visit_expression(ast, b)?;
            }
            Expression::Literal(i) => {
                print!("{}", i);
            }
            _ => (),
        }
        if let Expression::Var(v) = &ast.exprs[e] {
            print!("{}", v);
        }
        Ok(())
    }
    fn visit_declaration(&mut self, ast: &mut Ast, d: DeclarationKey) -> Result<()> {
        print!("{} = ", ast.decls[d].id);
        self.visit_expression(ast, ast.decls[d].val)?;
        Ok(())
    }
    fn visit_statement(&mut self, ast: &mut Ast, s: StatementKey) -> Result<()> {
        match ast.stmts[s] {
            Statement::Decl(d) => self.visit_declaration(ast, d)?,
            Statement::Print(e) => {
                print!("print ");
                self.visit_expression(ast, e)?
            }
        }
        println!(";");
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut ast = ast::Ast::new();
    let program = toy::ProgramParser::new().parse(
        &mut ast,
        "a = 3 + 4; b = 6; print a + (b / 2) * (3 + 5) * (a - b); print 3 * 5 + 6 * (3 - 2 / (2 + 30));",
    )?;
    let mut nr = NameResolver {
        names: HashMap::new(),
    };
    nr.visit_program(&mut ast, &program)?;

    println!("Initial program:");
    let mut p = Printer {};
    p.visit_program(&mut ast, &program)?;

    let mut cf = ConstFolder {};
    cf.visit_program(&mut ast, &program)?;

    println!("");
    println!("Const-folded program:");
    p.visit_program(&mut ast, &program)?;

    Ok(())
}
