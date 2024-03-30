use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Box<Expr>),
    Expr(Box<Expr>),
    VarDeclaration(String, Option<Box<Expr>>),
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::VarDeclaration(name, initializer) => {
                visitor.visit_var_declaration(name, initializer)
            }
            Stmt::Block(stmts) => visitor.visit_block(stmts),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_print(&mut self, expr: &Box<Expr>) -> T;
    fn visit_expr(&mut self, expr: &Box<Expr>) -> T;
    fn visit_var_declaration(&mut self, name: &String, initializer: &Option<Box<Expr>>) -> T;
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> T;
}
