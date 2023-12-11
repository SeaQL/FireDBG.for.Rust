//! # Rust primitives to C++ type mapping
//!
//! This is architecture specific.
use crate::ValueType;
use lldb::BasicType;

#[allow(dead_code)]
pub(super) fn basic_type(value_type: &ValueType) -> BasicType {
    match value_type {
        ValueType::bool => BasicType::Bool,
        ValueType::char => BasicType::Char,
        ValueType::u8 => BasicType::UnsignedChar,
        ValueType::i8 => BasicType::SignedChar,
        ValueType::u16 => BasicType::UnsignedShort,
        ValueType::i16 => BasicType::Short,
        ValueType::u32 => BasicType::UnsignedInt,
        ValueType::i32 => BasicType::Int,
        ValueType::u64 => BasicType::UnsignedLongLong,
        ValueType::i64 => BasicType::LongLong,
        ValueType::u128 => BasicType::UnsignedInt128,
        ValueType::i128 => BasicType::Int128,
        ValueType::usize => BasicType::UnsignedLongLong,
        ValueType::isize => BasicType::LongLong,
        ValueType::f32 => BasicType::Float,
        ValueType::f64 => BasicType::Double,
        _ => panic!("Not primitive"),
    }
}
