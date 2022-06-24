use super::ast_types::*;

pub struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        let exprs: [&dyn Expr<String>; 2] = [expr.left.as_ref(), expr.right.as_ref()];
        self.parenthesize(format!("{}", expr.operator).as_str(), &exprs)
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        let exprs = [expr.expression.as_ref()];
        self.parenthesize("group", &exprs)
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        format!("{:#?}", expr)
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        let exprs = [expr.right.as_ref()];
        self.parenthesize(format!("{:?}", expr.operator).as_str(), &exprs)
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &impl Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&dyn Expr]) -> String {
        let mut return_str = format!("({}", name);

        for expr in exprs.iter() {
            return_str.push_str(format!(" {:#?}", expr).as_str());
        }

        return_str.push(')');
        return_str
    }
}
