//! The representation of Rust Value along with serde and pretty-print facilities

use crate::{util::impl_serde_with_str, IndexMap};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use strum::{Display, EnumString};

pub const STD_HASH_MAP: &str = "std::collections::hash::map::HashMap<";
pub const STD_HASH_SET: &str = "std::collections::hash::set::HashSet<";
pub const STD_HASH_STATE: &str = ", std::collections::hash::map::RandomState>";
pub const CORE_REF_CELL: &str = "core::cell::RefCell<";
pub const STD_MUTEX: &str = "std::sync::mutex::Mutex<";
pub const STD_RWLOCK: &str = "std::sync::rwlock::RwLock<";
pub const STD_OS_STRING: &str = "std::ffi::os_str::OsString";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
/// A representation of Rust Value
pub enum RValue {
    /// Aka `()`
    Unit,
    Prim(PValue),
    Bytes {
        typename: String,
        value: Vec<u8>,
    },
    /// This includes a simple `&v`, as well as `Box` and `*const`
    Ref {
        typename: RefType,
        addr: RefAddr,
        value: Box<RValue>,
    },
    /// &dyn or Box<dyn>
    DynRef {
        typename: String,
        addr: RefAddr,
        vtable: RefAddr,
        value: Box<RValue>,
    },
    /// Rc or Arc
    RefCounted {
        typename: RefCountedType,
        addr: RefAddr,
        strong: u64,
        weak: u64,
        value: Box<RValue>,
    },
    /// Rc<dyn> or Arc<dyn>
    DynRefCounted {
        typename: String,
        addr: RefAddr,
        strong: u64,
        weak: u64,
        vtable: RefAddr,
        value: Box<RValue>,
    },
    UnresolvedRef {
        addr: RefAddr,
    },
    Struct {
        typename: String,
        fields: IndexMap<String, RValue>,
    },
    Tuple {
        typename: String,
        items: Vec<RValue>,
    },
    Enum {
        typename: String,
        variant: String,
    },
    String {
        typename: StringType,
        value: String,
    },
    Union {
        typeinfo: UnionType,
        variant: String,
        fields: IndexMap<String, RValue>,
    },
    Option {
        typename: String,
        variant: String,
        value: Option<Box<RValue>>,
    },
    Result {
        typename: String,
        variant: String,
        value: Box<RValue>,
    },
    Array {
        typename: ArrayType,
        data: Vec<RValue>,
    },
    Opaque,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
/// Reference Type
pub enum RefType {
    Box,
    #[strum(serialize = "ref")]
    /// Reference
    Ref,
    #[strum(serialize = "ptr")]
    /// Pointer
    Ptr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
/// Reference-counted Smart Pointers
pub enum RefCountedType {
    Rc,
    Arc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
/// Type of String
pub enum StringType {
    #[strum(serialize = "&str")]
    /// String Literal
    StrLit,
    /// Owned String
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
/// Type of Array
pub enum ArrayType {
    /// Array
    Arr,
    /// Vector
    Vec,
    Slice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Reference Address
pub enum RefAddr {
    Addr(Addr),
    Redacted,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Memory Address. Assumed to be 8 bytes.
pub struct Addr([u8; 8]);

#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "typename", content = "value")]
/// Primitive Value
pub enum PValue {
    bool(bool),
    char(char),
    u8(u8),
    i8(i8),
    u16(u16),
    i16(i16),
    u32(u32),
    i32(i32),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    u64(u64),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    i64(i64),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    /// Most code assumes memory address to be 64 bits
    usize(u64),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    isize(i64),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    u128(u128),
    #[serde(serialize_with = "serialize_as_str", deserialize_with = "deserialize_from_str")]
    i128(i128),
    f32(f32),
    f64(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Aka Rust's complex enums, e.g. `Result`, `Option`.
///
/// The surface syntax of C-style and complex enums may look similar, but the underlying implementation is very different.
/// If you "tweak" a enum from `enum { A }` to `enum { A(u8) }`, everything changes.
pub struct UnionType {
    pub name: String,
    pub variants: Vec<String>,
}

impl RValue {
    pub fn redact_addr(&mut self) {
        match self {
            Self::Unit => (),
            Self::Prim(_) => (),
            Self::Bytes { .. } => (),
            Self::Ref { addr, value, .. } => {
                *addr = RefAddr::Redacted;
                value.redact_addr();
            }
            Self::DynRef {
                addr,
                vtable,
                value,
                ..
            } => {
                *addr = RefAddr::Redacted;
                *vtable = RefAddr::Redacted;
                value.redact_addr();
            }
            Self::RefCounted { addr, value, .. } => {
                *addr = RefAddr::Redacted;
                value.redact_addr();
            }
            Self::DynRefCounted {
                addr,
                vtable,
                value,
                ..
            } => {
                *addr = RefAddr::Redacted;
                *vtable = RefAddr::Redacted;
                value.redact_addr();
            }
            Self::UnresolvedRef { addr } => {
                *addr = RefAddr::Redacted;
            }
            Self::Struct { fields, .. } => {
                for value in fields.values_mut() {
                    value.redact_addr();
                }
            }
            Self::Tuple { items, .. } => {
                for value in items.iter_mut() {
                    value.redact_addr();
                }
            }
            Self::Enum { .. } => (),
            Self::String { .. } => (),
            Self::Union { fields, .. } => {
                for value in fields.values_mut() {
                    value.redact_addr();
                }
            }
            Self::Option { value, .. } => {
                if let Some(value) = value {
                    value.redact_addr();
                }
            }
            Self::Result { value, .. } => {
                value.redact_addr();
            }
            Self::Array { data, .. } => {
                for value in data.iter_mut() {
                    value.redact_addr();
                }
            }
            Self::Opaque => (),
        }
    }

    pub fn prim(&self) -> Option<PValue> {
        if let Self::Prim(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn struct_field(&self, field: &str) -> Option<&RValue> {
        if let Self::Struct { fields, .. } = self {
            fields.get(field)
        } else {
            None
        }
    }
}

impl Display for RValue {
    /// Pretty print `RValue` in a Rust-like syntax. Using `{:#}` would expand to multiple lines.
    ///
    /// Similar to `std::fmt::Debug`, but the goal is to make the format string almost-compile.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = f.alternate();
        let width = f.width().unwrap_or_default();
        match self {
            Self::Unit => write!(f, "()")?,
            Self::Prim(v) => {
                if pretty {
                    write!(f, "{:#}", v)?
                } else {
                    write!(f, "{}", v)?
                }
            }
            Self::Bytes { typename, value } => {
                if typename.starts_with('&') {
                    write!(f, "&")?;
                }
                print_bytes(f, &value)?;
            }
            Self::Ref {
                typename, value, ..
            } => {
                let b = match *typename {
                    RefType::Ref => {
                        write!(f, "&")?;
                        ""
                    }
                    RefType::Ptr => {
                        write!(f, "*")?;
                        ""
                    }
                    RefType::Box => {
                        write!(f, "alloc::boxed::Box::new(")?;
                        ")"
                    }
                };
                if pretty {
                    write!(f, "{value:#width$}", width = width)?;
                } else {
                    write!(f, "{value}")?;
                }
                write!(f, "{}", b)?;
            }
            Self::DynRef {
                typename, value, ..
            } => {
                if typename.starts_with("&dyn ") {
                    write!(f, "{} ", typename)?;
                } else {
                    write!(
                        f,
                        "{}::new(",
                        typename.replacen("alloc::boxed::Box<", "alloc::boxed::Box::<", 1)
                    )?;
                }
                if pretty {
                    write!(f, "{value:#width$}", width = width)?;
                } else {
                    write!(f, "{value}")?;
                }
                if typename.starts_with("&dyn ") {
                    // no bracket
                } else {
                    write!(f, ")")?;
                }
            }
            Self::RefCounted {
                typename, value, ..
            } => {
                write!(
                    f,
                    "{}::new(",
                    match typename {
                        RefCountedType::Rc => "alloc::rc::Rc",
                        RefCountedType::Arc => "alloc::sync::Arc",
                    }
                )?;
                if pretty {
                    write!(f, "{value:#width$}", width = width)?;
                } else {
                    write!(f, "{value}")?;
                }
                write!(f, ")")?;
            }
            Self::DynRefCounted {
                typename, value, ..
            } => {
                write!(
                    f,
                    "{}::new(",
                    typename
                        .replacen("alloc::rc::Rc<", "alloc::rc::Rc::<", 1)
                        .replacen("alloc::sync::Arc<", "alloc::sync::Arc::<", 1)
                )?;
                if pretty {
                    write!(f, "{value:#width$}", width = width)?;
                } else {
                    write!(f, "{value}")?;
                }
                write!(f, ")")?;
            }
            Self::UnresolvedRef { .. } => {
                write!(f, "&()")?;
            }
            Self::Struct { typename, fields } => {
                if fields.is_empty() {
                    return write!(f, "{} {{ }}", typename);
                }
                let is_tuple = fields.keys().next().map(|s| s.as_str()).unwrap_or_default() == "0";
                if is_tuple {
                    write!(f, "{}(", typename)?;
                    for (i, (_, value)) in fields.iter().enumerate() {
                        write!(
                            f,
                            "{}{}",
                            value,
                            if i < fields.len() - 1 { ", " } else { "" }
                        )?;
                    }
                    return write!(f, ")");
                }
                if typename == STD_OS_STRING {
                    print_os_string(f, fields, width, pretty)?;
                } else if (typename.starts_with(STD_HASH_MAP) || typename.starts_with(STD_HASH_SET))
                    && typename.ends_with(STD_HASH_STATE)
                    && (fields.contains_key("items") && fields.contains_key("len"))
                {
                    print_hashmap(f, typename, fields, width, pretty)?;
                } else {
                    write!(f, "{}", typename)?;
                    print_struct(f, typename, fields, width, pretty)?;
                }
            }
            Self::Tuple { items, .. } => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{}{}", item, if i < items.len() - 1 { ", " } else { "" })?;
                }
                write!(f, ")")?;
            }
            Self::Enum { typename, variant } => write!(f, "{}::{}", typename, variant)?,
            Self::String { typename, value } => {
                if typename == &StringType::StrLit {
                    write!(f, "{:?}", value)?;
                } else {
                    write!(f, "String::from({:?})", value)?;
                }
            }
            Self::Union {
                typeinfo,
                variant,
                fields,
            } => {
                let typename = typeinfo.name.replacen('<', "::<", 1);
                write!(f, "{}::{}", typename, variant)?;
                if !fields.is_empty() {
                    let is_tuple = fields.keys().next().unwrap() == "0";
                    if is_tuple {
                        write!(f, "(")?;
                        for (i, (_, value)) in fields.iter().enumerate() {
                            write!(
                                f,
                                "{}{}",
                                value,
                                if i < fields.len() - 1 { ", " } else { "" }
                            )?;
                        }
                        write!(f, ")")?;
                    } else if fields.is_empty() {
                        write!(f, " {{ }}")?;
                    } else {
                        print_struct(f, &typename, fields, width, pretty)?;
                    }
                }
            }
            Self::Option {
                typename,
                variant,
                value,
            } => {
                write!(
                    f,
                    "{}::{}",
                    typename.replacen("core::option::Option<", "core::option::Option::<", 1),
                    variant,
                )?;
                if let Some(value) = value {
                    write!(f, "(")?;
                    if pretty {
                        write!(f, "{value:#width$}", width = width)?;
                    } else {
                        write!(f, "{value}")?;
                    }
                    write!(f, ")")?;
                }
            }
            Self::Result {
                typename,
                variant,
                value,
            } => {
                write!(
                    f,
                    "{}::{}(",
                    typename.replacen("core::result::Result<", "core::result::Result::<", 1),
                    variant
                )?;
                if pretty {
                    write!(f, "{value:#width$}", width = width)?;
                } else {
                    write!(f, "{value}")?;
                }
                write!(f, ")")?;
            }
            Self::Array { typename, data } => {
                match *typename {
                    ArrayType::Arr => (),
                    ArrayType::Vec => write!(f, "vec!")?,
                    ArrayType::Slice => write!(f, "&")?,
                }
                print_arr_items(f, data, width, pretty)?;
            }
            Self::Opaque => write!(f, "(?)")?,
        }
        Ok(())
    }
}

/// A wrapper type only for implementing `Display`.
pub struct ArgumentList<'a>(pub &'a [(String, RValue)]);

impl<'a> Display for ArgumentList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = f.alternate();
        for (name, value) in self.0.iter() {
            write!(f, "{name}: ")?;
            if pretty {
                write!(f, "{value:#}")?;
            } else {
                write!(f, "{value}")?;
            }
        }
        Ok(())
    }
}

fn print_struct(
    f: &mut std::fmt::Formatter<'_>,
    typename: &str,
    fields: &IndexMap<String, RValue>,
    width: usize,
    pretty: bool,
) -> std::fmt::Result {
    if (typename.starts_with(CORE_REF_CELL) && fields.contains_key("value"))
        || (typename.starts_with(STD_MUTEX) && fields.contains_key("data"))
        || (typename.starts_with(STD_RWLOCK) && fields.contains_key("data"))
    {
        let mut value = fields
            .get(if typename.starts_with(CORE_REF_CELL) {
                "value"
            } else {
                "data"
            })
            .unwrap();
        match value {
            RValue::Struct { fields, .. } => {
                if let Some(v) = fields.get("value") {
                    value = v;
                }
            }
            _ => (),
        }
        write!(f, "::new(")?;
        if pretty {
            write!(f, "{}", value)?;
        } else {
            write!(f, "{:#width$}", value, width = width)?;
        }
        write!(f, ")")?;
        return Ok(());
    }

    let nl = if pretty { '\n' } else { ' ' };
    let indent = if pretty {
        String::from_utf8(vec![b' '; (width + 1) * 4]).unwrap()
    } else {
        String::new()
    };
    write!(f, " {{{}", nl)?;
    for (i, (attr, value)) in fields.iter().enumerate() {
        if pretty {
            write!(
                f,
                "{}{}: {:#width$}{}{}",
                indent,
                attr,
                value,
                ",",
                nl,
                width = (width + 1)
            )?;
        } else {
            write!(
                f,
                "{}: {}{}",
                attr,
                value,
                if i < fields.len() - 1 { ", " } else { " " }
            )?;
        }
    }
    if pretty {
        write!(f, "{}", String::from_utf8(vec![b' '; width * 4]).unwrap())?;
    }
    write!(f, "}}")?;
    Ok(())
}

fn print_hashmap(
    f: &mut std::fmt::Formatter<'_>,
    typename: &str,
    fields: &IndexMap<String, RValue>,
    width: usize,
    pretty: bool,
) -> std::fmt::Result {
    let items = match fields.get("items").unwrap() {
        RValue::Array { data, .. } => data,
        _ => {
            write!(f, "{}", typename)?;
            return print_struct(f, typename, fields, width, pretty);
        }
    };

    let nl = if pretty { "\n" } else { "" };
    let indent = if pretty {
        String::from_utf8(vec![b' '; width * 4]).unwrap()
    } else {
        String::new()
    };
    write!(f, "{}[{}", indent, nl)?;
    for (i, value) in items.iter().enumerate() {
        if pretty {
            write!(
                f,
                "    {}{:#width$}{}{}",
                indent,
                value,
                ",",
                nl,
                width = (width + 1)
            )?;
        } else {
            write!(
                f,
                "{}{}",
                value,
                if i < items.len() - 1 { ", " } else { "" }
            )?;
        }
    }
    write!(f, "{}]{}", indent, nl)?;
    write!(f, ".into_iter(){}", nl)?;
    write!(f, ".collect::<{}>()", typename.replace(STD_HASH_STATE, ">"))?;
    Ok(())
}

fn print_os_string(
    f: &mut std::fmt::Formatter<'_>,
    fields: &IndexMap<String, RValue>,
    width: usize,
    pretty: bool,
) -> std::fmt::Result {
    write!(f, "{}::from_encoded_bytes_unchecked(", STD_OS_STRING)?;
    if pretty {
        write!(
            f,
            "{}",
            String::from_utf8(vec![b' '; (width + 1) * 4]).unwrap()
        )?;
    }
    let inner = match fields.get("inner") {
        Some(RValue::Struct { fields, .. }) => {
            if let Some(inner) = fields.get("inner") {
                match inner {
                    RValue::Bytes { value, .. } => value,
                    _ => {
                        write!(f, "?)")?;
                        return Ok(());
                    }
                }
            } else {
                write!(f, "?)")?;
                return Ok(());
            }
        }
        _ => {
            write!(f, "?)")?;
            return Ok(());
        }
    };
    if let Ok(string) = std::str::from_utf8(inner) {
        write!(f, "String::from({:?}).into_bytes()", string)?;
    } else {
        write!(f, "vec!")?;
        print_bytes(f, inner)?;
    }
    if pretty {
        write!(f, "{}", String::from_utf8(vec![b' '; width * 4]).unwrap())?;
    }
    write!(f, ")")?;
    Ok(())
}

fn print_arr_items(
    f: &mut std::fmt::Formatter<'_>,
    data: &[RValue],
    width: usize,
    pretty: bool,
) -> std::fmt::Result {
    write!(f, "[")?;
    if data.is_empty() {
        write!(f, "]")?;
        return Ok(());
    }
    let indent = if pretty {
        write!(f, "\n")?;
        String::from_utf8(vec![b' '; (width + 1) * 4]).unwrap()
    } else {
        String::new()
    };
    for (i, item) in data.iter().enumerate() {
        if pretty {
            write!(f, "{}{:#width$},\n", indent, item, width = (width + 1))?;
        } else {
            write!(f, "{}{}", item, if i < data.len() - 1 { ", " } else { "" })?;
        }
    }
    if pretty {
        write!(f, "{}", String::from_utf8(vec![b' '; width * 4]).unwrap())?;
    }
    write!(f, "]")?;
    Ok(())
}

fn print_bytes(f: &mut std::fmt::Formatter<'_>, bytes: &[u8]) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, b) in bytes.iter().enumerate() {
        write!(
            f,
            "0x{b:02x}{}",
            if i < bytes.len() - 1 { ", " } else { "" }
        )?;
    }
    write!(f, "]")?;
    Ok(())
}

impl RValue {
    pub fn typename(&self) -> String {
        match self {
            Self::Unit => "()".to_owned(),
            Self::Prim(v) => v.typename().to_owned(),
            Self::Bytes { typename, .. } => typename.to_owned(),
            Self::Ref {
                typename, value, ..
            } => match *typename {
                RefType::Ref => {
                    format!("&{}", value.typename())
                }
                RefType::Ptr => {
                    format!("*{}", value.typename())
                }
                ty => {
                    format!("{}<{}>", ty, value.typename())
                }
            },
            Self::DynRef { typename, .. } => typename.to_owned(),
            Self::RefCounted {
                typename, value, ..
            } => {
                format!("{}<{}>", typename, value.typename())
            }
            Self::DynRefCounted { typename, .. } => typename.to_owned(),
            Self::UnresolvedRef { .. } => "&()".to_owned(),
            Self::Struct { typename, .. } => typename.to_owned(),
            Self::Tuple { typename, .. } => typename.to_owned(),
            Self::Enum { typename, .. } => typename.to_owned(),
            Self::String { typename, .. } => typename.to_string(),
            Self::Union { typeinfo, .. } => typeinfo.name.to_owned(),
            Self::Option { typename, .. } => typename.to_owned(),
            Self::Result { typename, .. } => typename.to_owned(),
            Self::Array { typename, data } => {
                let vtype = if data.is_empty() {
                    "(?)".to_owned()
                } else {
                    data[0].typename()
                };
                match *typename {
                    ArrayType::Arr => format!("[{}]", vtype),
                    ArrayType::Vec => format!("Vec<{}>", vtype),
                    ArrayType::Slice => format!("&[{}]", vtype),
                }
            }
            Self::Opaque => "(?)".to_owned(),
        }
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct { .. })
    }

    pub fn is_result(&self) -> bool {
        matches!(self, Self::Result { .. })
    }

    /// # Panics
    /// Panic if self is not of `Result` variant, or variant is not "Ok" or "Err".
    pub fn result_variant(&self) -> Result<(), ()> {
        match self {
            Self::Result { variant, .. } => match variant.as_str() {
                "Ok" => Ok(()),
                "Err" => Err(()),
                e => panic!("Unexpected variant {e}"),
            },
            _ => panic!("Not Result"),
        }
    }
}

impl Display for PValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if f.alternate() { "_" } else { "" };
        match self {
            Self::bool(v) => write!(f, "{v:?}"),
            Self::char(v) => write!(f, "'{v}'"),
            Self::u8(v) => write!(f, "{v}{s}u8"),
            Self::i8(v) => write!(f, "{v}{s}i8"),
            Self::u16(v) => write!(f, "{v}{s}u16"),
            Self::i16(v) => write!(f, "{v}{s}i16"),
            Self::u32(v) => write!(f, "{v}{s}u32"),
            Self::i32(v) => write!(f, "{v}{s}i32"),
            Self::u64(v) => write!(f, "{v}{s}u64"),
            Self::i64(v) => write!(f, "{v}{s}i64"),
            Self::usize(v) => write!(f, "{v}{s}usize"),
            Self::isize(v) => write!(f, "{v}{s}isize"),
            Self::u128(v) => write!(f, "{v}{s}u128"),
            Self::i128(v) => write!(f, "{v}{s}i128"),
            Self::f32(v) => write!(f, "{v}{s}f32"),
            Self::f64(v) => write!(f, "{v}{s}f64"),
        }
    }
}

impl PValue {
    pub fn typename(&self) -> &'static str {
        match self {
            Self::bool(..) => "bool",
            Self::char(..) => "char",
            Self::u8(..) => "u8",
            Self::i8(..) => "i8",
            Self::u16(..) => "u16",
            Self::i16(..) => "i16",
            Self::u32(..) => "u32",
            Self::i32(..) => "i32",
            Self::u64(..) => "u64",
            Self::i64(..) => "i64",
            Self::usize(..) => "usize",
            Self::isize(..) => "isize",
            Self::u128(..) => "u128",
            Self::i128(..) => "i128",
            Self::f32(..) => "f32",
            Self::f64(..) => "f64",
        }
    }
}

impl Addr {
    pub fn new(b: &[u8]) -> Self {
        match b.len() {
            4 => Self([0, 0, 0, 0, b[0], b[1], b[2], b[3]]),
            8 => Self([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            _ => panic!("Not an address: {b:?}"),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl From<u64> for Addr {
    fn from(v: u64) -> Self {
        Self::new(&v.to_ne_bytes())
    }
}

impl Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        // to_ne_bytes (endianness)
        for b in self.0.iter().rev() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl FromStr for Addr {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut addr = [0u8; 8];
        let bytes = s.as_bytes();
        if bytes.len() != 18 {
            return Err("Not 8 bytes?");
        }
        let mut i = 2;
        while i < 18 {
            let str = std::str::from_utf8(&bytes[i..i + 2]).unwrap();
            let b = u8::from_str_radix(str, 16).map_err(|_| "Invalid byte")?;
            addr[7 - (i - 2) / 2] = b;
            i += 2;
        }
        Ok(Addr(addr))
    }
}

impl Display for RefAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Redacted => write!(f, "<redacted>"),
            Self::Addr(addr) => write!(f, "{addr}"),
        }
    }
}

impl FromStr for RefAddr {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "<redacted>" {
            Ok(Self::Redacted)
        } else {
            Ok(Self::Addr(s.parse()?))
        }
    }
}

impl_serde_with_str!(RefAddr);
impl_serde_with_str!(RefType);
impl_serde_with_str!(RefCountedType);
impl_serde_with_str!(StringType);
impl_serde_with_str!(ArrayType);

fn serialize_as_str<S, T>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: std::fmt::Display,
{
    serializer.serialize_str(&v.to_string())
}

fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = <&str>::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serde_addr() {
        let addr = Addr([0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0x00]);
        let ref_addr = RefAddr::Addr(addr);
        assert_eq!(addr.to_string().as_str(), "0x00efcdab78563412");
        assert_eq!(ref_addr.to_string().as_str(), "0x00efcdab78563412");
        assert_eq!(addr, "0x00efcdab78563412".parse().unwrap());
        assert_eq!(ref_addr, "0x00efcdab78563412".parse().unwrap());
        assert_eq!(RefAddr::Redacted, "<redacted>".parse().unwrap());
    }
}
