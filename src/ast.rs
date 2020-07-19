use crate::util::Result;

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
    rel_id: Symbol,
    exprs: Vec<Expr>,
}

impl Atom {
    pub fn new(rel_id: String, exprs: Vec<Expr>) -> Atom {
        Atom { rel_id, exprs }
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

pub type Integer = i32;

// TODO: Make literal truly copyable
#[derive(Clone, Copy, Debug)]
pub struct Literal {
    id: u32
}

impl Literal {
    pub fn new(s: String) -> Literal {
        unimplemented!();
    }
}

// impl Copy for Literal {}

pub type Symbol = String;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    TInt, TString
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::TInt => 4,
            Type::TString => 8
        }
    }
}

pub struct Schema(Vec<(Symbol, Type, usize)>);

impl Schema {
    pub fn new(schema: Vec<(Symbol, Type)>) -> Schema {
        let mut offset = 0;
        let vec = schema.into_iter().map(|(sym, ty)| {
            let curr = offset;
            offset += ty.size();
            (sym, ty, curr)
        }).collect();
        Schema(vec)
    }

    pub fn symbol_to_offset(&self, symbol: &Symbol) -> Result<usize> {
        for (sym, _, sz) in self.0.iter() {
            if sym == symbol {
                return Ok(*sz);
            }
        }
        Err(format!("symbol {} not found", symbol))
    }
}
