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
