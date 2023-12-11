use super::WriteErr;
use crate::get_union_type;
use lldb::{IsValid, SBType, TypeClass};
use std::{
    fmt::{Display, Write},
    ops::Deref,
    rc::Rc,
    str::FromStr,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ValueType {
    #[default]
    Unit,
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64,
    usize,
    isize,
    str,
    Option(Rc<ValueType>),
    Result(Rc<ValueType>, Rc<ValueType>),
    /// ref, Box, Rc, Arc
    Reference(Rc<ValueType>),
    /// Fat pointers: &dyn Trait, Box<dyn>, Rc<dyn>, Arc<dyn>
    DynRef(Rc<ValueType>),
    Array(Rc<ValueType>, usize),
    Slice(Rc<ValueType>),
    Other(String),
}

#[derive(Debug)]
pub enum SizeOfType {
    Sized(usize),
    Unknown,
}

impl ValueType {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Unit // Is () actually primitive?
                | Self::bool
                | Self::char
                | Self::u8
                | Self::i8
                | Self::u16
                | Self::i16
                | Self::u32
                | Self::i32
                | Self::u64
                | Self::i64
                | Self::u128
                | Self::i128
                | Self::f32
                | Self::f64
                | Self::usize
                | Self::isize
        )
    }

    pub fn primitive_name(&self) -> &'static str {
        match self {
            Self::Unit => "()",
            Self::bool => "bool",
            Self::char => "char",
            Self::u8 => "u8",
            Self::i8 => "i8",
            Self::u16 => "u16",
            Self::i16 => "i16",
            Self::u32 => "u32",
            Self::i32 => "i32",
            Self::u64 => "u64",
            Self::i64 => "i64",
            Self::u128 => "u128",
            Self::i128 => "i128",
            Self::f32 => "f32",
            Self::f64 => "f64",
            Self::usize => "usize",
            Self::isize => "isize",
            _ => panic!("Not primitive"),
        }
    }

    pub fn size_of(&self) -> SizeOfType {
        match self {
            ValueType::Unit => SizeOfType::Sized(0),
            ValueType::bool => SizeOfType::Sized(std::mem::size_of::<bool>()),
            ValueType::char => SizeOfType::Sized(std::mem::size_of::<char>()),
            ValueType::u8 => SizeOfType::Sized(std::mem::size_of::<u8>()),
            ValueType::i8 => SizeOfType::Sized(std::mem::size_of::<i8>()),
            ValueType::u16 => SizeOfType::Sized(std::mem::size_of::<u16>()),
            ValueType::i16 => SizeOfType::Sized(std::mem::size_of::<i16>()),
            ValueType::u32 => SizeOfType::Sized(std::mem::size_of::<u32>()),
            ValueType::i32 => SizeOfType::Sized(std::mem::size_of::<i32>()),
            ValueType::u64 => SizeOfType::Sized(std::mem::size_of::<u64>()),
            ValueType::i64 => SizeOfType::Sized(std::mem::size_of::<i64>()),
            ValueType::u128 => SizeOfType::Sized(std::mem::size_of::<u128>()),
            ValueType::i128 => SizeOfType::Sized(std::mem::size_of::<i128>()),
            ValueType::f32 => SizeOfType::Sized(std::mem::size_of::<f32>()),
            ValueType::f64 => SizeOfType::Sized(std::mem::size_of::<f64>()),
            ValueType::usize => SizeOfType::Sized(std::mem::size_of::<usize>()),
            ValueType::isize => SizeOfType::Sized(std::mem::size_of::<isize>()),
            ValueType::str => SizeOfType::Unknown,
            ValueType::Option(_) => SizeOfType::Unknown,
            ValueType::Result(_, _) => SizeOfType::Unknown,
            ValueType::Reference(r) => match r.deref() {
                ValueType::str => SizeOfType::Sized(std::mem::size_of::<&str>()),
                _ => SizeOfType::Sized(std::mem::size_of::<&u8>()),
            },
            ValueType::DynRef(_) => SizeOfType::Sized(std::mem::size_of::<&dyn std::fmt::Debug>()),
            ValueType::Array(ty, len) => match ty.size_of() {
                SizeOfType::Sized(size) => SizeOfType::Sized(size * len),
                SizeOfType::Unknown => SizeOfType::Unknown,
            },
            ValueType::Slice(_) => SizeOfType::Sized(std::mem::size_of::<&[u8]>()),
            ValueType::Other(_) => SizeOfType::Unknown,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            ValueType::Reference(r) => match r.deref() {
                ValueType::str => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_thin_ptr(&self) -> bool {
        match self {
            ValueType::DynRef(_) => false,
            // &str is fat
            ValueType::Reference(r) => !matches!(r.deref(), ValueType::str),
            _ => false,
        }
    }

    pub fn is_fat_ptr(&self) -> bool {
        match self {
            ValueType::DynRef(_) => true,
            // &str is fat
            ValueType::Reference(r) => matches!(r.deref(), ValueType::str),
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            ValueType::u8
            | ValueType::i8
            | ValueType::u16
            | ValueType::i16
            | ValueType::u32
            | ValueType::i32
            | ValueType::u64
            | ValueType::i64
            | ValueType::u128
            | ValueType::i128
            | ValueType::usize
            | ValueType::isize => true,
            _ => false,
        }
    }

    pub fn is_signed_integer(&self) -> bool {
        match self {
            ValueType::i8
            | ValueType::i16
            | ValueType::i32
            | ValueType::i64
            | ValueType::i128
            | ValueType::isize => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::f32 | Self::f64)
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "()")?,
            Self::bool => write!(f, "bool")?,
            Self::char => write!(f, "char")?,
            Self::u8 => write!(f, "u8")?,
            Self::i8 => write!(f, "i8")?,
            Self::u16 => write!(f, "u16")?,
            Self::i16 => write!(f, "i16")?,
            Self::u32 => write!(f, "u32")?,
            Self::i32 => write!(f, "i32")?,
            Self::u64 => write!(f, "u64")?,
            Self::i64 => write!(f, "i64")?,
            Self::u128 => write!(f, "u128")?,
            Self::i128 => write!(f, "i128")?,
            Self::f32 => write!(f, "f32")?,
            Self::f64 => write!(f, "f64")?,
            Self::usize => write!(f, "usize")?,
            Self::isize => write!(f, "isize")?,
            Self::str => write!(f, "str")?,
            Self::Option(a) => write!(f, "core::option::Option<{}>", a)?,
            Self::Result(a, b) => write!(f, "core::result::Result<{}, {}>", a, b)?,
            Self::Reference(r) => write!(f, "&{}", r)?,
            Self::DynRef(r) => write!(f, "&dyn {}", r)?,
            Self::Array(ty, len) => write!(f, "[{ty}; {len}]")?,
            Self::Slice(ty) => write!(f, "&[{ty}]")?,
            Self::Other(t) => write!(f, "{}", t)?,
        }
        Ok(())
    }
}

impl FromStr for ValueType {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("&dyn") {
            return Ok(Self::DynRef(Rc::new(s[5..].parse()?)));
        } else if s.starts_with("&[") && s.ends_with(']') && !s.contains("; ") {
            let s = &s[2..s.len() - 1];
            let ty = Rc::new(s.parse()?);
            return Ok(Self::Slice(ty));
        } else if s.starts_with("&") {
            return Ok(Self::Reference(Rc::new(s[1..].parse()?)));
        } else if s.starts_with("core::option::Option<") {
            let p = "core::option::Option<".len();
            ends_with_gt(s)?;
            return Ok(Self::Option(Rc::new(s[p..s.len() - 1].parse()?)));
        } else if s.starts_with("alloc::boxed::Box<dyn ") {
            let p = "alloc::boxed::Box<".len();
            ends_with_gt(s)?;
            return Ok(Self::DynRef(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("alloc::boxed::Box<") {
            let p = "alloc::boxed::Box<".len();
            ends_with_gt(s)?;
            return Ok(Self::Reference(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("alloc::rc::Rc<dyn ") {
            let p = "alloc::rc::Rc<".len();
            ends_with_gt(s)?;
            return Ok(Self::DynRef(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("alloc::rc::Rc<") {
            let p = "alloc::rc::Rc<".len();
            ends_with_gt(s)?;
            return Ok(Self::Reference(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("alloc::sync::Arc<dyn ") {
            let p = "alloc::sync::Arc<".len();
            ends_with_gt(s)?;
            return Ok(Self::DynRef(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("alloc::sync::Arc<") {
            let p = "alloc::sync::Arc<".len();
            ends_with_gt(s)?;
            return Ok(Self::Reference(Rc::new(maybe_pair(s, p).parse()?)));
        } else if s.starts_with("core::ptr::non_null::NonNull<") {
            let p = "core::ptr::non_null::NonNull<".len();
            ends_with_gt(s)?;
            return Ok(Self::Reference(Rc::new(s[p..s.len() - 1].parse()?)));
        } else if s.starts_with("core::result::Result<") && s.ends_with('>') {
            let s = s
                .split_once("core::result::Result<")
                .expect("starts_with checked")
                .1;
            let p = parse_pair(s)?;
            return Ok(Self::Result(
                Rc::new(s[..p].trim().parse()?),
                Rc::new(s[p + 1..s.len() - 1].trim().parse()?),
            ));
        } else if s.starts_with('[') && s.ends_with(']') && s.contains("; ") {
            let s = &s[1..s.len() - 1];
            let (ty, len) = s
                .rsplit_once("; ")
                .ok_or("array def with type and length")?;
            let ty = Rc::new(ty.parse()?);
            let len = len.parse().map_err(|_| "Fail to parse array length")?;
            return Ok(Self::Array(ty, len));
        }
        Ok(match s {
            "()" => Self::Unit,
            "bool" => Self::bool,
            "char" => Self::char,
            "u8" => Self::u8,
            "i8" => Self::i8,
            "u16" => Self::u16,
            "i16" => Self::i16,
            "u32" => Self::u32,
            "i32" => Self::i32,
            "u64" => Self::u64,
            "i64" => Self::i64,
            "u128" => Self::u128,
            "i128" => Self::i128,
            "f32" => Self::f32,
            "f64" => Self::f64,
            "usize" => Self::usize,
            "isize" => Self::isize,
            "str" => Self::str,
            _ => Self::Other(s.to_owned()),
        })
    }
}

// In Box<T, A = Global>, the A will be dropped
fn maybe_pair(s: &str, p: usize) -> &str {
    let s = &s[p..];
    if let Ok(p) = parse_pair(s) {
        &s[..p]
    } else {
        &s[..s.len() - 1]
    }
}

// return the pivot at the separating `,`
pub(crate) fn parse_pair(s: &str) -> Result<usize, &'static str> {
    let mut depth = 0;
    for (i, c) in s[..s.len() - 1].chars().enumerate() {
        if c == ',' && depth == 0 {
            return Ok(i);
        }
        if c == '<' {
            depth += 1;
        }
        if c == '>' {
            depth -= 1;
        }
    }
    if depth != 0 {
        return Err("unmatched < >");
    }
    return Err("cannot find ,");
}

fn ends_with_gt(s: &str) -> Result<(), &'static str> {
    if s.ends_with(">") {
        Ok(())
    } else {
        return Err("missing closing >");
    }
}

/// Boil down a type to its primitive
///
/// A(B { i: C(i32) }) -> i32
pub(crate) fn boildown(ty: ValueType, sb_type: SBType) -> Result<ValueType, WriteErr> {
    let res = match ty {
        ValueType::Unit
        | ValueType::bool
        | ValueType::char
        | ValueType::u8
        | ValueType::i8
        | ValueType::u16
        | ValueType::i16
        | ValueType::u32
        | ValueType::i32
        | ValueType::u64
        | ValueType::i64
        | ValueType::u128
        | ValueType::i128
        | ValueType::f32
        | ValueType::f64
        | ValueType::usize
        | ValueType::isize
        | ValueType::Reference(_)
        | ValueType::DynRef(_)
        | ValueType::Option(_)
        | ValueType::Result(_, _)
        | ValueType::Array(_, _)
        | ValueType::Slice(_) => ty, // can't be boiled
        ValueType::str => panic!("Impossible"),
        ValueType::Other(_) => {
            let type_class = sb_type.type_class();
            if type_class.contains(TypeClass::Struct) {
                let num = sb_type.number_of_fields();
                match num {
                    0 => ValueType::Unit,
                    1 | 2 => {
                        if num == 2 {
                            let sb_type = sb_type.field_at_index(1).type_();
                            if !sb_type.name().starts_with("core::marker::PhantomData<") {
                                return Ok(ty);
                            }
                        }
                        let sb_type = sb_type.field_at_index(0).type_();
                        let ty = sb_type.name().parse().map_err(|_| WriteErr)?;
                        boildown(ty, sb_type)?
                    }
                    _ => ty,
                }
            } else {
                ty
            }
        }
    };
    Ok(res)
}

/// Compute the alignment of the type.
///
/// Does anything (other than intrinsics) have an alignment greater than 8?
pub(crate) fn alignment_of(sb_type: SBType) -> Result<usize, ()> {
    if let Ok(vtype) = sb_type.name().parse::<ValueType>() {
        if vtype.is_primitive() {
            Ok(match vtype {
                ValueType::Unit => std::mem::align_of::<()>(),
                ValueType::bool => std::mem::align_of::<bool>(),
                ValueType::char => std::mem::align_of::<char>(),
                ValueType::u8 => std::mem::align_of::<u8>(),
                ValueType::i8 => std::mem::align_of::<i8>(),
                ValueType::u16 => std::mem::align_of::<u16>(),
                ValueType::i16 => std::mem::align_of::<i16>(),
                ValueType::u32 => std::mem::align_of::<u32>(),
                ValueType::i32 => std::mem::align_of::<i32>(),
                ValueType::u64 => std::mem::align_of::<u64>(),
                ValueType::i64 => std::mem::align_of::<i64>(),
                ValueType::u128 => std::mem::align_of::<u128>(),
                ValueType::i128 => std::mem::align_of::<i128>(),
                ValueType::f32 => std::mem::align_of::<f32>(),
                ValueType::f64 => std::mem::align_of::<f64>(),
                ValueType::usize => std::mem::align_of::<usize>(),
                ValueType::isize => std::mem::align_of::<isize>(),
                _ => unreachable!(),
            })
        } else if vtype.is_str() {
            Ok(std::mem::align_of::<&str>())
        } else if vtype.is_thin_ptr() {
            Ok(std::mem::align_of::<&u8>())
        } else if vtype.is_fat_ptr() {
            Ok(std::mem::align_of::<&dyn std::fmt::Debug>())
        } else if get_union_type(&sb_type).is_some() {
            // don't know how to handle union types
            Err(())
        } else if sb_type.number_of_fields() == 0 {
            // hopefully this is right
            Ok(sb_type.byte_size() as usize)
        } else {
            let mut max = 1;
            for field in sb_type.fields() {
                let inner = field.type_();
                max = max.max(alignment_of(inner)?);
            }
            Ok(max)
        }
    } else {
        Err(())
    }
}

/// Condense the type { x: i32, y: i64 } -> (i32, i64)
pub(crate) fn condense(sb_type: SBType) -> Result<Vec<ValueType>, ()> {
    if let Ok(vtype) = sb_type.name().parse::<ValueType>() {
        if matches!(vtype.size_of(), SizeOfType::Sized(_)) {
            Ok(vec![vtype])
        } else if get_union_type(&sb_type).is_some() {
            // don't know how to handle union types
            Err(())
        } else if sb_type.number_of_fields() == 0 {
            // hopefully this is right
            match sb_type.byte_size() {
                0 => Ok(vec![]),
                1 => Ok(vec![ValueType::u8]),
                2 => Ok(vec![ValueType::u16]),
                4 => Ok(vec![ValueType::u32]),
                8 => Ok(vec![ValueType::u64]),
                16 => Ok(vec![ValueType::u128]),
                _ => Err(()),
            }
        } else {
            let mut chain = Vec::new();
            for field in sb_type.fields() {
                let inner = field.type_();
                chain.append(&mut condense(inner)?);
            }
            Ok(chain)
        }
    } else {
        Err(())
    }
}

/// alloc::rc::Rc -> core::ptr::non_null::NonNull -> * alloc::rc::RcBox
/// alloc::sync::Arc -> core::ptr::non_null::NonNull -> * alloc::sync::ArcInner
pub(crate) fn get_ref_counted_pointee(pointer: &SBType) -> Result<(SBType, SBType), WriteErr> {
    let field = pointer.field_at_index(0);
    if field.name() != "ptr" {
        return Err(WriteErr);
    }
    let ptr = field.type_();
    if !ptr.is_valid() {
        return Err(WriteErr);
    }
    let field = ptr.field_at_index(0);
    if field.name() != "pointer" {
        return Err(WriteErr);
    }
    let pointer = field.type_();
    if !pointer.is_valid() {
        return Err(WriteErr);
    }
    let pointee = pointer.pointee_type();
    if !pointee.is_valid() {
        return Err(WriteErr);
    }
    Ok((ptr, pointee))
}

pub(crate) fn format_value_type_as_tuple(chain: &[ValueType]) -> String {
    let mut s = format!("(");
    for ty in chain {
        write!(s, "{}, ", ty).unwrap();
    }
    write!(s, ")").unwrap();
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_return_type() {
        assert_eq!("()".parse::<ValueType>().unwrap(), ValueType::Unit);
        assert_eq!("i32".parse::<ValueType>().unwrap(), ValueType::i32);
        assert_eq!("usize".parse::<ValueType>().unwrap(), ValueType::usize);
        assert_eq!(
            "&str".parse::<ValueType>().unwrap(),
            ValueType::Reference(ValueType::str.into())
        );
        assert_eq!(
            "&dyn Car".parse::<ValueType>().unwrap(),
            ValueType::DynRef(ValueType::Other("Car".to_owned()).into())
        );
        assert_eq!("()".parse::<ValueType>().unwrap(), ValueType::Unit);
        assert_eq!(
            "core::result::Result<i32, i64>"
                .parse::<ValueType>()
                .unwrap(),
            ValueType::Result(ValueType::i32.into(), ValueType::i64.into()).into(),
        );
        assert_eq!(
            "core::result::Result<core::result::Result<u8, i8>, i64>"
                .parse::<ValueType>()
                .unwrap(),
            ValueType::Result(
                ValueType::Result(ValueType::u8.into(), ValueType::i8.into()).into(),
                ValueType::i64.into(),
            )
        );
        assert_eq!(
            "core::result::Result<core::result::Result<u8, core::result::Result<(), i8>>, i64>"
                .parse::<ValueType>()
                .unwrap(),
            ValueType::Result(
                ValueType::Result(
                    ValueType::u8.into(),
                    ValueType::Result(ValueType::Unit.into(), ValueType::i8.into()).into()
                )
                .into(),
                ValueType::i64.into(),
            )
        );
        assert_eq!(
            "core::result::Result<core::result::Result<u8, core::result::Result<(), i8>>, core::result::Result<f32, f64>>"
                .parse::<ValueType>()
                .unwrap(),
            ValueType::Result(
                ValueType::Result(
                    ValueType::u8.into(),
                    ValueType::Result(ValueType::Unit.into(), ValueType::i8.into()).into()
                )
                .into(),
                ValueType::Result(ValueType::f32.into(), ValueType::f64.into()).into(),
            )
        );
    }
}
