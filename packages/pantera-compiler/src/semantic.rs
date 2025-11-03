use std::rc::Rc;
use pantera_ast::statement::GlobalStatement;
use pantera_ast::statement_visitor::StatementVisitorMut;
use pantera_std::init_compiler_globals;
use crate::errors::CompilerError;
use crate::semantic::break_statement_check::BreakStatementCheck;
use crate::semantic::check::Check;
use crate::semantic::declaration_check::DeclarationCheck;
use crate::semantic::return_statement_check::ReturnStatementCheck;

mod declaration_check;
mod check;
mod break_statement_check;
mod return_statement_check;

fn run_semantic_check<'a, T: StatementVisitorMut + Check>(stmts: &Vec<GlobalStatement>, mut check: T) -> Vec<CompilerError> {
    stmts.iter().for_each(|stmt|{
        stmt.visit(&mut check);
    });

    check.get_errors()
}

pub fn run_all_semantic_checks(stmts: &Vec<GlobalStatement>) -> Result<(), String> {
    let std_lid = Rc::new(init_compiler_globals());
    let errors = vec![
        run_semantic_check(stmts, DeclarationCheck::new(Rc::clone(&std_lid))),
        run_semantic_check(stmts, BreakStatementCheck::new()),
        run_semantic_check(stmts, ReturnStatementCheck::new())
    ].into_iter().flatten().fold(String::new(), |acc, el| {
        return acc + "\n" + &el.get_message()
    });

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}