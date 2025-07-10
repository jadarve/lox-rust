use crate::lox::expr::Expr;
use std::collections::HashMap;

use super::{ExprAssign, ExprVisitor, ParseTreeId, Stmt, StmtVisitor};

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    interpreter_local_map: HashMap<ParseTreeId, usize>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            interpreter_local_map: HashMap::new(),
        }
    }

    pub fn resolve(
        &mut self,
        statements: &Vec<Stmt>,
    ) -> Result<HashMap<ParseTreeId, usize>, String> {
        self.interpreter_local_map.clear();

        for stmt in statements {
            stmt.accept(self)?;
        }

        Ok(self.interpreter_local_map.clone())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        // if there is a scope, otherwise we are in the global scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, false);
        }
    }

    fn define(&mut self, name: String) {
        // if there is a scope, otherwise we are in the global scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, true);
        }
    }

    fn resolve_local(&mut self, parse_tree_id: ParseTreeId, name: &str) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name) {
                self.interpreter_local_map.insert(parse_tree_id, i);
                println!(
                    "Resolver: resolve_local: parse_tree_id: {}, name: {}, scope_index: {}",
                    parse_tree_id, name, i
                );
            }
        }
    }

    fn resolve_function(
        &mut self,
        arguments: &Vec<String>,
        body: &Box<super::Stmt>,
    ) -> Result<(), String> {
        // create a new scope for the function arguments
        self.begin_scope();
        for arg in arguments {
            self.declare(arg.clone());
            self.define(arg.clone());
        }

        // then resolve the function body
        body.accept(self)?;
        self.end_scope();
        Ok(())
    }
}

impl StmtVisitor<Result<(), String>> for Resolver {
    fn visit_print(&mut self, expr: &Box<Expr>) -> Result<(), String> {
        expr.accept(self)
    }

    fn visit_expr(&mut self, expr: &Box<Expr>) -> Result<(), String> {
        expr.accept(self)
    }

    fn visit_var_declaration(
        &mut self,
        name: &String,
        initializer: &Option<Box<Expr>>,
    ) -> Result<(), String> {
        println!(
            "Resolver: visit_var_declaration: name: {}, initializer: {:?}",
            name, initializer
        );

        self.declare(name.clone());
        if let Some(initializer) = initializer {
            initializer.accept(self)?;
        }
        self.define(name.clone());
        Ok(())
    }

    fn visit_block(&mut self, stmts: &Vec<super::Stmt>) -> Result<(), String> {
        self.begin_scope();
        for stmt in stmts {
            stmt.accept(self)?;
        }
        self.end_scope();

        Ok(())
    }

    fn visit_if(
        &mut self,
        condition: &Box<Expr>,
        then_branch: &Box<super::Stmt>,
        else_branch: &Option<Box<super::Stmt>>,
    ) -> Result<(), String> {
        condition.accept(self)?;
        then_branch.accept(self)?;

        if let Some(else_branch) = else_branch {
            else_branch.accept(self)?;
        }

        Ok(())
    }

    fn visit_while(
        &mut self,
        condition: &Box<Expr>,
        body: &Box<super::Stmt>,
    ) -> Result<(), String> {
        condition.accept(self)?;
        body.accept(self)
    }

    fn visit_function_declaration(
        &mut self,
        name: &String,
        arguments: &Vec<String>,
        body: &Box<super::Stmt>,
    ) -> Result<(), String> {
        // first declare and define the function name
        self.declare(name.clone());
        self.define(name.clone());

        self.resolve_function(arguments, body)
    }
}

impl ExprVisitor<Result<(), String>> for Resolver {
    fn visit_assign(&mut self, assign: &ExprAssign) -> Result<(), String> {
        assign.right.accept(self)?;

        println!(
            "Resolver: visit_assign: parse_tree_id: {}, left: {}",
            assign.parse_tree_id, assign.left
        );

        self.resolve_local(assign.parse_tree_id, &assign.left);
        Ok(())
    }

    fn visit_binary_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_equal(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_not_equal(
        &mut self,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_less(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_less_equal(
        &mut self,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_greater(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_greater_equal(
        &mut self,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_add(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_sub(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_mul(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_binary_div(&mut self, left: &Box<Expr>, right: &Box<Expr>) -> Result<(), String> {
        left.accept(self)?;
        right.accept(self)
    }

    fn visit_unary_bang(&mut self, expr: &Box<Expr>) -> Result<(), String> {
        expr.accept(self)
    }

    fn visit_unary_minus(&mut self, expr: &Box<Expr>) -> Result<(), String> {
        expr.accept(self)
    }

    fn visit_literal_string(&mut self, _value: &String) -> Result<(), String> {
        Ok(())
    }

    fn visit_literal_number(&mut self, _value: &f64) -> Result<(), String> {
        Ok(())
    }

    fn visit_false(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn visit_true(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn visit_nil(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn visit_identifier(&mut self, value: &String) -> Result<(), String> {
        if let Some(scope) = self.scopes.last() {
            if let Some(defined) = scope.get(value) {
                if !defined {
                    return Err(format!(
                        "cannot read local variable \"{value}\" in its own initializer."
                    ));
                }
            }
        }

        Ok(())
    }

    fn visit_call(&mut self, callee: &Box<Expr>, arguments: &Vec<Expr>) -> Result<(), String> {
        callee.accept(self)?;
        for arg in arguments {
            arg.accept(self)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::lox::{Parser, Scanner};

    use super::*;

    #[test]
    fn test_resolver() -> Result<(), String> {
        let source = r#"
            var a = "global";
            {
                var b = "variable b";
                print "before fun declaration: " + b;
                
                fun showA() {

                    print "------------------";
                    print "start showA";
                    print a;
                    print b;
                    print "end showA";
                    print "------------------";
                }

                showA();
                var a = "block";
                showA();

                print "block a: " + a;
            }
        "#;

        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().map_err(|e| e.to_string())?;

        // run the resolver here
        println!("Resolver: executing statements: {}", statements.len());
        let mut resolver = Resolver::new();
        let resolver_map = resolver.resolve(&statements)?;

        println!("Resolver map: {:?}", resolver_map);

        Ok(())
    }
}
