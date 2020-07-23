use crate::util::Result;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::mem::size_of;
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Rule {
    pub head: Atom,
    pub body: Vec<Atom>,
}

impl Rule {
    pub fn new(head: Atom, body: Vec<Atom>) -> Rule {
        Rule { head, body }
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub rel_id: Symbol,
    pub exprs: Vec<Expr>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal {
    id: u32
}

lazy_static! {
    static ref ID_STORE: Mutex<(HashMap<String, u32>, u32)> = Mutex::new((HashMap::new(), 0));
}

impl Literal {
    pub fn new(s: String) -> Literal {
        let mut store = ID_STORE.lock().unwrap();
        let (store, cnt) = &mut *store;
        let id = store.entry(s).or_insert_with(|| {
            *cnt += 1;
            *cnt
        });
        Literal { id: *id }
    }
}

pub type Symbol = String;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    IntType, LitType
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::IntType => size_of::<i32>(),
            Type::LitType => size_of::<String>(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Schema(pub Vec<(Symbol, Type, usize)>);

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

    pub fn symbol_to_pos_info(&self, symbol: &Symbol) -> Result<(Type, usize)> {
        for (sym, ty, sz) in self.0.iter() {
            if sym == symbol {
                return Ok((*ty, *sz));
            }
        }
        Err(format!("symbol {} not found", symbol))
    }
}
