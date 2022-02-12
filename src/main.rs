#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub toy);
use generational_arena::Arena;

mod ast;

fn main() {
    let mut arena = Arena::new();

    let ast =
        toy::StatementsParser::new().parse(&mut arena, "a = 3 + 4; b = 6; print a + b * (a - b);");
    println!("{:#?}", ast);
}
