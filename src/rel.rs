use crate::ast::{Schema, Symbol, Type, Literal, Integer};
use crate::util::Result;

pub trait Column {
    fn get(&self, pos: usize) -> Result<Cell>;
}

pub trait Tuple {
    fn get(&self, sym: &Symbol) -> Result<Cell>;
}

pub trait Rel {
    type T: Tuple;
    type C: Column;
    fn at(&self, pos: usize) -> Result<Self::T>;
    fn col(&self, sym: &Symbol) -> Result<Self::C>;

    fn symbol_to_offset(&self, sym: &Symbol) -> Result<usize>;
    fn col_by_offset(&self, sym: usize) -> Result<Self::C>;
}

pub struct RowRel {
    tuples: Vec<u8>,
    tuple_nums: usize,
    tuple_size: usize,
    schema: Schema,
}

pub struct RowRelTuple<'a> {
    rel: &'a RowRel,
    row_offset: usize,
}

impl<'a> Tuple for RowRelTuple<'a> {
    fn get(&self, sym: &Symbol) -> Result<Cell> {
        unimplemented!();
    }
}

pub struct RowRelColumn<'a> {
    rel: &'a RowRel,
    col_offset: usize,
}

impl<'a> Column for RowRelColumn<'a> {
    fn get(&self, pos: usize) -> Result<Cell> {
        unimplemented!();
    }
}

pub union Cell {
    cint: Integer,
    clit: Literal,
}

impl<'a> Rel for &'a RowRel {
    type T = RowRelTuple<'a>;
    type C = RowRelColumn<'a>;
    
    fn at(&self, pos: usize) -> Result<RowRelTuple<'a>> {
        if pos >= self.tuple_nums {
            return Err(format!("{} is out of bound ({})", pos, self.tuple_nums));
        }
        let offset = pos * self.tuple_size;
        Ok(RowRelTuple {
            rel: self,
            row_offset: offset,
        })
    }

    fn symbol_to_offset(&self, sym: &Symbol) -> Result<usize> {
        self.schema.symbol_to_offset(sym)
    }

    fn col(&self, sym: &Symbol) -> Result<RowRelColumn<'a>> {
        let offset = self.symbol_to_offset(sym)?;
        self.col_by_offset(offset)
    }

    fn col_by_offset(&self, offset: usize) -> Result<RowRelColumn<'a>> {
        Ok(RowRelColumn {
            rel: self,
            col_offset: offset,
        })
    }
}