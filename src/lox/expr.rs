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

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::BinaryEqual(left, right) => visitor.visit_binary_equal(left, right),
            Expr::BinaryNotEqual(left, right) => visitor.visit_binary_not_equal(left, right),
            Expr::BinaryLess(left, right) => visitor.visit_binary_less(left, right),
            Expr::BinaryLessEqual(left, right) => visitor.visit_binary_less_equal(left, right),
            Expr::BinaryGreater(left, right) => visitor.visit_binary_greater(left, right),
            Expr::BinaryGreaterEqual(left, right) => {
                visitor.visit_binary_greater_equal(left, right)
            }
            Expr::BinaryAdd(left, right) => visitor.visit_binary_add(left, right),
            Expr::BinarySub(left, right) => visitor.visit_binary_sub(left, right),
            Expr::BinaryMul(left, right) => visitor.visit_binary_mul(left, right),
            Expr::BinaryDiv(left, right) => visitor.visit_binary_div(left, right),
            Expr::UnaryBang(expr) => visitor.visit_unary_bang(expr),
            Expr::UnaryMinus(expr) => visitor.visit_unary_minus(expr),
            Expr::LiteralString(value) => visitor.visit_literal_string(value),
            Expr::LiteralNumber(value) => visitor.visit_literal_number(value),
            Expr::False => visitor.visit_false(),
            Expr::True => visitor.visit_true(),
            Expr::Nil => visitor.visit_nil(),
            Expr::Identifier(value) => visitor.visit_identifier(value),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_not_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_less(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_less_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_greater(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_greater_equal(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_add(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_sub(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_mul(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_div(&self, left: &Box<Expr>, right: &Box<Expr>) -> T;

    fn visit_unary_bang(&self, expr: &Box<Expr>) -> T;
    fn visit_unary_minus(&self, expr: &Box<Expr>) -> T;

    fn visit_literal_string(&self, value: &String) -> T;
    fn visit_literal_number(&self, value: &f64) -> T;
    fn visit_false(&self) -> T;
    fn visit_true(&self) -> T;
    fn visit_nil(&self) -> T;
    fn visit_identifier(&self, value: &String) -> T;
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
