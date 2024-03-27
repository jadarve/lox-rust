use super::Token;

///////////////////////////////////////////////////////////////////////////////
// Using enums

// pub enum Statement {
//     Expression(Expression),
// }

// pub enum Expression {
//     Equality(Equality),
// }

// pub enum Equality {
//     Comparison(Comparison),
//     Equality(Comparison, Token, Box<Comparison>),
// }

// pub enum Comparison {
//     Term(Term),
//     Comparison(Term, Token, Box<Comparison>),
// }

// pub enum Term {
//     Factor(Factor),
//     Term(Factor, Token, Box<Term>),
// }
// pub enum Factor {
//     Unary(Unary),
//     Factor(Unary, Token, Box<Factor>),
// }

// pub enum Unary {
//     Primary(Primary),
//     Unary(Token, Box<Unary>),
// }

// pub enum Primary {
//     Literal(Token),
//     Grouping(Box<Expression>),
// }

// TODO: Visitor trait
// TODO: Separate Token according to their usage

///////////////////////////////////////////////////////////////////////////////
// Using structs

// pub struct Declaration {

// }

// pub trait ExprVisitor {
//     fn visit_addition(&self, left: &Box<dyn Expr>, right: &Box<dyn Expr>) -> f64;
//     fn visit_substraction(&self, expr: &BinaryExpr) -> f64;
// }

// pub trait Expr {
//     fn accept(&self, visitor: &dyn ExprVisitor) -> f64;
// }

// pub enum BinaryExpr {
//     Addition(Box<dyn Expr>, Box<dyn Expr>),
//     Subtraction(Box<dyn Expr>, Box<dyn Expr>),
//     Multiplication(Box<dyn Expr>, Box<dyn Expr>),
//     Division(Box<dyn Expr>, Box<dyn Expr>),
//     // ...
// }

// impl Expr for BinaryExpr {
//     fn accept(&self, visitor: &dyn ExprVisitor) -> f64 {
//         match self {
//             BinaryExpr::Addition(left, right) => visitor.visit_addition(left, right),
//             _ => {
//                 todo!()
//             }
//         };
//         // match self {
//         //     BinaryExpr::Addition(left, right) => visitor.visit_binary_expr(self),
//         //     BinaryExpr::Subtraction(left, right) => visitor.visit_binary_expr(self),
//         //     BinaryExpr::Multiplication(left, right) => visitor.visit_binary_expr(self),
//         //     BinaryExpr::Division(left, right) => visitor.visit_binary_expr(self),
//         // }
//         todo!()
//     }
// }

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expr {
    // Binary
    BinaryEqual(Box<Expr>, Box<Expr>),
    BinaryNotEqual(Box<Expr>, Box<Expr>),
    BinaryLess(Box<Expr>, Box<Expr>),
    BinaryLessEqual(Box<Expr>, Box<Expr>),
    BinaryGreater(Box<Expr>, Box<Expr>),
    BinaryGreaterEqual(Box<Expr>, Box<Expr>),
    BinaryAdd(Box<Expr>, Box<Expr>),
    BinarySub(Box<Expr>, Box<Expr>),
    BinaryMul(Box<Expr>, Box<Expr>),
    BinaryDiv(Box<Expr>, Box<Expr>),

    // Unary
    UnaryBang(Box<Expr>),
    UnaryMinus(Box<Expr>),

    // Terminal nodes
    LiteralString(String),
    LiteralNumber(f64),
    False,
    True,
    Nil,
    Identifier(String),
}

#[cfg(test)]
mod tests {

    use super::Expr;

    #[test]
    fn test_partial_eq_number() {
        let expr1 = Expr::LiteralNumber(1.0);
        let expr2 = Expr::LiteralNumber(1.0);
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_partial_eq_string_literal() {
        let expr1 = Expr::LiteralString("hello".to_string());
        let expr2 = Expr::LiteralString("hello".to_string());
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_partial_eq_binary_add() {
        let expr1 = Expr::BinaryAdd(
            Box::new(Expr::LiteralNumber(1.0)),
            Box::new(Expr::LiteralNumber(2.0)),
        );
        let expr2 = Expr::BinaryAdd(
            Box::new(Expr::LiteralNumber(1.0)),
            Box::new(Expr::LiteralNumber(2.0)),
        );
        assert_eq!(expr1, expr2);
    }
}
