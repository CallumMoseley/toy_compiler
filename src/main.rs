#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub toy);

mod ast;
mod visit;

use ast::*;
use visit::*;

use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;

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

fn main() -> Result<()> {
    let mut ast = ast::Ast::new();
    let program =
        toy::ProgramParser::new().parse(&mut ast, "a = 3 + 4; b = 6; print a + b * (a - b);")?;
    let mut nr = NameResolver {
        names: HashMap::new(),
    };
    nr.visit_program(&mut ast, program)?;
    println!("{:?}", nr.names);
    println!("{:?}", ast.name_resolution);

    let program = toy::ProgramParser::new()
        .parse(&mut ast, "a = 3 + 4; b = 6; print a + b * (a - b) / c;")?;
    let mut nr = NameResolver {
        names: HashMap::new(),
    };
    nr.visit_program(&mut ast, program)?;
    Ok(())
}
