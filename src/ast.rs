#[derive(Clone, Debug)]
pub struct Rule {
    head: Atom,
    body: Vec<Atom>,
}

impl Rule {
    pub fn new(head: Atom, body: Vec<Atom>) -> Rule {
        Rule { head, body }
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    rel: String,
    exprs: Vec<Expr>,
}

impl Atom {
    pub fn new(rel: String, exprs: Vec<Expr>) -> Atom {
        Atom { rel, exprs }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(Symbol),
    Int(i32),
    Lit(String),
}

impl Expr {
    pub fn var(s: Symbol) -> Expr {
        Expr::Var(s)
    }
    pub fn int(i: Integer) -> Expr {
        Expr::Int(i)
    }
    pub fn lit(s: String) -> Expr {
        Expr::Lit(s)
    }
}
type Integer = i32;
type Symbol = String;

#[macro_export]
macro_rules! var {
    ($e:expr) => {
        $crate::ast::Expr::var($e)
    };
}

#[macro_export]
macro_rules! int {
    ($e:expr) => {
        $crate::ast::Expr::int($e)
    };
}

#[macro_export]
macro_rules! lit {
    ($e:expr) => {
        $crate::ast::Expr::lit($e)
    };
}

#[macro_export]
macro_rules! rule {
    ($e:expr, $($es:expr),+) => {
        $crate::ast::Rule::new($e, vec!($($es),+))
    };
}

#[macro_export]
macro_rules! atom {
    ($e:expr, $($es:expr),+) => {
        $crate::ast::Atom::new($e, vec!($($es),+))
    };
}

#[cfg(test)]
mod ast_tests {
    use crate::ast::*;
    #[test]
    fn ast_macro_test() {
        let expr1: Expr = lit!("Hello, world".into());
        let expr2: Expr = int!(123);
        let expr3: Expr = var!("x".into());
        let atom1: Atom = atom!("Rel".into(), expr1, expr2, expr3);
        let _rule1: Rule = rule!(atom1.clone(), atom1.clone(), atom1.clone());
    }
}
