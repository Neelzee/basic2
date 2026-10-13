use crate::types::Type;

pub mod consts;
pub mod error;
pub mod types;
pub mod symbol_table;

pub trait ToB2 {
    fn to_b2(&self) -> String;
}

pub trait AsB2Type<'a> {
    fn as_b2_type(&self) -> Type<'a>;
}
