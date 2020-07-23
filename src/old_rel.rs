use crate::ast::{Integer, Literal, Schema, Symbol, Type};
use crate::util::Result;

pub trait Column: Clone {
    fn get(&self, row: usize) -> Result<Cell>;
}

pub trait Tuple: Clone {
    type PosInfo;
    fn get(&self, sym: &Symbol) -> Result<Cell>;
    fn get_by_pos_info(&self, pos_info: Self::PosInfo) -> Result<Cell>;
}

pub trait Rel<'a>: Clone {
    type T<'a>: Tuple;
    type C<'a>: Column;
    type PosInfo;
    type Iter<'a>: Iterator<Item = Self::T<'a>>;

    fn new(schema: &Schema) -> Self;

    fn at<'a>(&'a self, pos: usize) -> Result<Self::T<'a>>;
    fn col<'a>(&'a self, sym: &Symbol) -> Result<Self::C<'a>>;

    fn symbol_to_pos_info(&self, sym: &Symbol) -> Result<Self::PosInfo>;
    fn col_by_pos_info<'a>(&'a self, pos_info: Self::PosInfo) -> Result<Self::C<'a>>;

    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, tuple: Vec<Cell>);

    fn iter<'a>(&'a self) -> Self::Iter<'a>;
    // fn iter<'a>(&'a self) -> impl Iterator<Item = Self::T<'a>>;
}

#[derive(Clone)]
pub struct RowRel {
    tuples: Vec<u8>,
    tuple_nums: usize,
    tuple_size: usize,
    schema: Schema,
}

impl Rel for RowRel {
    type T<'a> = RowRelTuple<'a>;
    type C<'a> = RowRelColumn<'a>;
    type PosInfo = (Type, usize);
    type Iter<'a> = impl Iterator<Item = Self::T<'a>>;
    
    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        (0..self.tuple_nums).map(move |idx| self.at(idx).unwrap())
    }

    fn new(schema: &Schema) -> RowRel {
        RowRel {
            tuples: Vec::new(),
            tuple_nums: 0,
            tuple_size: schema.0.last().unwrap().2,
            schema: schema.clone(),
        }
    }

    fn at<'a>(&'a self, pos: usize) -> Result<Self::T<'a>> {
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

    fn col<'a>(&'a self, sym: &Symbol) -> Result<Self::C<'a>> {
        let pos_info = self.symbol_to_pos_info(sym)?;
        self.col_by_pos_info(pos_info)
    }

    // TODO: make PosInfo inner structure invisible to outside, so that it's safe not to
    // check the precondition
    fn col_by_pos_info<'a>(&'a self, pos_info: (Type, usize)) -> Result<RowRelColumn<'a>> {
        Ok(RowRelColumn {
            rel: self,
            col_offset: pos_info.1,
            ty: pos_info.0,
        })
    }

    fn size(&self) -> usize {
        self.tuple_nums
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn insert(&mut self, tuple: Vec<Cell>) {
        let it = Cell::cell_iter(&tuple, &self.schema);
        for data in it {
            self.tuples.push(data);
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct RowRelColumn<'a> {
    rel: &'a RowRel,
    col_offset: usize,
    ty: Type,
}

impl<'a> Column for RowRelColumn<'a> {
    fn get(&self, row: usize) -> Result<Cell> {
        if row >= self.rel.tuple_nums {
            return Err(format!(
                "row {} out of bound ({})",
                row, self.rel.tuple_nums
            ));
        }
        unsafe {
            let ptr = self
                .rel
                .tuples
                .as_ptr()
                .add(row * self.rel.tuple_nums + self.col_offset);
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
            Type::IntType => Cell {
                cint: *(ptr as *const i32),
            },
            Type::LitType => Cell {
                clit: *(ptr as *const Literal),
            },
        }
    }

    fn cell_iter<'a>(cells: &'a Vec<Cell>, schema: &'a Schema) -> impl Iterator<Item = u8> + 'a {
        cells
            .iter()
            .zip(schema.0.iter().map(|(_sym, ty, _sz)| ty))
            .flat_map(|(cell, ty)| {
                let ty_size = ty.size();
                let cell_ptr = (cell as *const Cell).cast::<u8>();
                (1..ty_size).map(move |i| unsafe { *cell_ptr.add(i) })
            })
    }
}
