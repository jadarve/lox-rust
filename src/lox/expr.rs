pub type ParseTreeId = u32;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expr {
    // Assign
    Assign(ExprAssign),

    // Binary
    BinaryOr(Box<Expr>, Box<Expr>),
    BinaryAnd(Box<Expr>, Box<Expr>),
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

    // Function call
    Call(Box<Expr>, Vec<Expr>),

    // Terminal nodes
    LiteralString(String),
    LiteralNumber(f64),
    False,
    True,
    Nil,
    Identifier(ExprIdentifier),
    // TODO: Parentheses
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Assign(assign) => visitor.visit_assign(&assign),
            Expr::BinaryOr(left, right) => visitor.visit_binary_or(left, right),
            Expr::BinaryAnd(left, right) => visitor.visit_binary_and(left, right),
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
            Expr::Call(callee, arguments) => visitor.visit_call(callee, arguments),
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
    fn visit_assign(&mut self, assign: &ExprAssign) -> T;
    fn visit_binary_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_not_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_less(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_less_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_greater(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_greater_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_add(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_sub(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_mul(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;
    fn visit_binary_div(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> T;

    fn visit_unary_bang(&mut self, expr: &Box<Expr>) -> T;
    fn visit_unary_minus(&mut self, expr: &Box<Expr>) -> T;

    fn visit_literal_string(&mut self, value: &String) -> T;
    fn visit_literal_number(&mut self, value: &f64) -> T;
    fn visit_false(&mut self) -> T;
    fn visit_true(&mut self) -> T;
    fn visit_nil(&mut self) -> T;
    fn visit_identifier(&mut self, value: &ExprIdentifier) -> T;
    fn visit_call(&mut self, callee: &Box<Expr>, arguments: &Vec<Expr>) -> T;
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ExprAssign {
    pub parse_tree_id: ParseTreeId,

    // TODO: left side should be an Expr once we need lvalues
    pub left: String,
    pub right: Box<Expr>,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ExprIdentifier {
    pub parse_tree_id: ParseTreeId,
    pub id: String,
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
