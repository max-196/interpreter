use crate::expr::Expr;

use super::expr::{Visitor, ExprType};

pub struct AstPrinter {}

impl <'a> Visitor<'a, String> for AstPrinter {
    fn visit_binary(&mut self, bin: &crate::expr::Bin<'a>) -> String {
        self.parenthesize(bin.operator.lexeme, &[bin.left, bin.right])
    }

    fn visit_grouping(&mut self, grp: &crate::expr::Grp<'a>) -> String {
        self.parenthesize("group", &[grp.expr])
    }

    fn visit_literal(&mut self, lit: &crate::expr::Lit<'a>) -> String {
        if lit.value == "" {return "nil".to_owned()}
        lit.value.to_owned()
    }

    fn visit_unary(&mut self, unr: &crate::expr::Unr<'a>) -> String {
        self.parenthesize(unr.operator.lexeme, &[unr.right])
    }
}

impl <'a> AstPrinter {
    pub fn print(&mut self, expr: ExprType) -> String {
        return expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, expr: &[&'a ExprType<'a>]) -> String {
        let mut st = format!("({name}");
        for e in expr {
            st.push(' ');
            st.push_str(&e.accept(self));
        }
        st.push(')');
        st
    }
}