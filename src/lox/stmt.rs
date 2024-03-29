use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Box<Expr>),
    Expr(Box<Expr>),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Expr(expr) => visitor.visit_expr(expr),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_print(&self, expr: &Box<Expr>) -> T;
    fn visit_expr(&self, expr: &Box<Expr>) -> T;
}
