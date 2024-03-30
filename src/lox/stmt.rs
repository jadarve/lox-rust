use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Box<Expr>),
    Expr(Box<Expr>),
    VarDeclaration(String, Option<Box<Expr>>),
    Block(Vec<Stmt>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
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
            Stmt::If(condition, then_branch, else_branch) => {
                visitor.visit_if(condition, then_branch, else_branch)
            }
            Stmt::While(condition, body) => visitor.visit_while(condition, body),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_print(&mut self, expr: &Box<Expr>) -> T;
    fn visit_expr(&mut self, expr: &Box<Expr>) -> T;
    fn visit_var_declaration(&mut self, name: &String, initializer: &Option<Box<Expr>>) -> T;
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> T;
    fn visit_if(
        &mut self,
        condition: &Box<Expr>,
        then_branch: &Box<Stmt>,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_while(&mut self, condition: &Box<Expr>, body: &Box<Stmt>) -> T;
}
