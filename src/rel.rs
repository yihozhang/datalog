use crate::ast::{Schema, Symbol, Type, Literal, Integer};
use crate::util::Result;

pub trait Column {
    fn get(&self, row: usize) -> Result<Cell>;
}

pub trait Tuple {
    type PosInfo;
    
    fn get(&self, sym: &Symbol) -> Result<Cell>;
    fn get_by_pos_info(&self, pos_info: Self::PosInfo) -> Result<Cell>;
}

pub trait Rel {
    type T: Tuple;
    type C: Column;
    type PosInfo;

    fn at(&self, pos: usize) -> Result<Self::T>;
    fn col(&self, sym: &Symbol) -> Result<Self::C>;

    fn symbol_to_pos_info(&self, sym: &Symbol) -> Result<Self::PosInfo>;
    fn col_by_pos_info(&self, pos_info: Self::PosInfo) -> Result<Self::C>;
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
    type PosInfo = (Type, usize);

    fn get(&self, sym: &Symbol) -> Result<Cell> {
        let pos_info = self.rel.symbol_to_pos_info(sym)?;
        self.get_by_pos_info(pos_info)
    }

    fn get_by_pos_info(&self, pos_info: (Type, usize)) -> Result<Cell> {
        let (ty, col_offset) = pos_info;
        unsafe {
            let ptr = self.rel.tuples.as_ptr().add(self.row_offset + col_offset);
            Ok(Cell::ptr_to_cell(ptr, ty))
        }
    }
}

pub struct RowRelColumn<'a> {
    rel: &'a RowRel,
    col_offset: usize,
    ty: Type,
}

impl<'a> Column for RowRelColumn<'a> {
    fn get(&self, row: usize) -> Result<Cell> {
        unsafe {
            let ptr = self.rel.tuples.as_ptr().add(row * self.rel.tuple_nums + self.col_offset);
            Ok(Cell::ptr_to_cell(ptr, self.ty))
        }
    }
}

pub union Cell {
    cint: Integer,
    clit: Literal,
}

impl Cell {
    unsafe fn ptr_to_cell(ptr: *const u8, ty: Type) -> Cell {
        match ty {
            Type::TInt => Cell { cint: *(ptr as *const i32) },
            Type::TString => Cell { clit: *(ptr as *const Literal)}
        }
    }
}

impl<'a> Rel for &'a RowRel {
    type T = RowRelTuple<'a>;
    type C = RowRelColumn<'a>;
    type PosInfo = (Type, usize);
    
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

    fn symbol_to_pos_info(&self, sym: &Symbol) -> Result<(Type, usize)> {
        self.schema.symbol_to_pos_info(sym)
    }

    fn col(&self, sym: &Symbol) -> Result<RowRelColumn<'a>> {
        let pos_info = self.symbol_to_pos_info(sym)?;
        self.col_by_pos_info(pos_info)
    }

    fn col_by_pos_info(&self, pos_info: (Type, usize)) -> Result<RowRelColumn<'a>> {
        Ok(RowRelColumn {
            rel: self,
            col_offset: pos_info.1,
            ty: pos_info.0,
        })
    }
}