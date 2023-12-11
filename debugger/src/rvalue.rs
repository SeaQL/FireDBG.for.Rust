use crate::{Addr, PValue, RValue, RefAddr, RefCountedType};

/// This trait only exists to workaround the orphan rule.
pub trait RValueLift {
    /// Lift the type to a higher level, in place
    fn lift(&mut self) -> Option<()>;
}

impl RValueLift for RValue {
    /// Lift the `RValue` to a higher level. It removes some implementation details of standard library types.
    fn lift(&mut self) -> Option<()> {
        match self {
            Self::Unit => (),
            Self::Prim(_) => (),
            Self::Bytes { .. } => (),
            Self::Ref { value, .. } => {
                value.lift();
            }
            Self::DynRef { .. } | Self::RefCounted { .. } | Self::DynRefCounted { .. } => {
                // already lifted
            }
            Self::UnresolvedRef { .. } => (),
            Self::Struct { typename, fields } => {
                for value in fields.values_mut() {
                    value.lift();
                }
                if typename.starts_with("(") && typename.ends_with(")") {
                    let mut items = Vec::new();
                    for (i, (t, item)) in fields.drain(..).enumerate() {
                        let t: usize = t.parse().ok()?;
                        if i != t {
                            return None;
                        }
                        items.push(item);
                    }
                    *self = Self::Tuple {
                        typename: std::mem::take(typename),
                        items,
                    };
                } else if typename.starts_with("core::sync::atomic::Atomic")
                    || typename.starts_with("core::cell::Cell<")
                {
                    if fields.len() != 1 {
                        return None;
                    }
                    let mut value = fields.get("value");
                    if value.is_none() {
                        value = fields.get("v");
                    }
                    let value = value?;
                    let prim = value.struct_field("value")?.prim()?;
                    fields.clear();
                    fields.insert("value".to_owned(), RValue::Prim(prim));
                } else if typename.starts_with("&dyn ")
                    || typename.starts_with("alloc::boxed::Box<dyn ")
                {
                    if fields.get("pointer").is_some() && fields.get("vtable").is_some() {
                        let pointer = fields.remove("pointer");
                        let vtable = fields.remove("vtable");
                        if let (Some(pointer), Some(vtable)) = (pointer, vtable) {
                            *self = Self::DynRef {
                                typename: std::mem::take(typename),
                                addr: get_addr(&pointer),
                                vtable: get_addr(&vtable),
                                value: match pointer {
                                    RValue::Ref { value, .. } => value,
                                    _ => Box::new(RValue::Opaque),
                                },
                            };
                        }
                    }
                } else if typename.starts_with("alloc::sync::Arc<dyn ")
                    || typename.starts_with("alloc::rc::Rc<dyn ")
                {
                    if let Some(value) = fields.remove("ptr") {
                        if let Some(value) = take_struct_field(value, "pointer") {
                            if let Self::Struct { mut fields, .. } = value {
                                let pointer = fields.remove("pointer");
                                let vtable = fields.remove("vtable");
                                if let (Some(pointer), Some(vtable)) = (pointer, vtable) {
                                    if let Self::Ref { addr, value, .. } = pointer {
                                        let extract = |name: &str| -> Option<u64> {
                                            take_usize(
                                                value
                                                    .struct_field(name)?
                                                    .struct_field("value")?
                                                    .prim()?,
                                            )
                                        };
                                        let strong = extract("strong").unwrap_or_default();
                                        let weak = extract("weak").unwrap_or_default();
                                        let field = if typename.starts_with("alloc::sync::Arc<") {
                                            "data"
                                        } else {
                                            "value"
                                        };
                                        *self = Self::DynRefCounted {
                                            typename: std::mem::take(typename),
                                            addr,
                                            strong,
                                            weak,
                                            vtable: get_addr(&vtable),
                                            value: Box::new(take_struct_field(
                                                unbox(value),
                                                field,
                                            )?),
                                        };
                                    } else {
                                        *self = Self::Struct {
                                            typename: std::mem::take(typename),
                                            fields: Default::default(),
                                        };
                                    }
                                } else {
                                    *self = Self::Struct {
                                        typename: std::mem::take(typename),
                                        fields: Default::default(),
                                    };
                                }
                            } else {
                                *self = Self::Struct {
                                    typename: std::mem::take(typename),
                                    fields: Default::default(),
                                };
                            }
                        } else {
                            *self = Self::Struct {
                                typename: std::mem::take(typename),
                                fields: Default::default(),
                            };
                        }
                    }
                } else if typename.starts_with("alloc::sync::Arc<")
                    || typename.starts_with("alloc::rc::Rc<")
                {
                    if let Some(value) = fields.remove("ptr") {
                        let value = take_struct_field(value, "pointer")?;
                        if let Self::Ref { addr, value, .. } = value {
                            let extract = |name: &str| -> Option<u64> {
                                take_usize(value.struct_field(name)?.struct_field("value")?.prim()?)
                            };
                            let strong = extract("strong").unwrap_or_default();
                            let weak = extract("weak").unwrap_or_default();
                            let (typename, field) = if typename.starts_with("alloc::sync::Arc<") {
                                (RefCountedType::Arc, "data")
                            } else if typename.starts_with("alloc::rc::Rc<") {
                                (RefCountedType::Rc, "value")
                            } else {
                                *self = Self::Struct {
                                    typename: std::mem::take(typename),
                                    fields: Default::default(),
                                };
                                return None;
                            };
                            *self = Self::RefCounted {
                                typename,
                                addr,
                                strong,
                                weak,
                                value: Box::new(
                                    take_struct_field(unbox(value), field).unwrap_or(Self::Opaque),
                                ),
                            };
                        } else {
                            *self = Self::Struct {
                                typename: std::mem::take(typename),
                                fields: Default::default(),
                            };
                        }
                    }
                }
            }
            Self::Tuple { .. } => {
                // already lifted
            }
            Self::Enum { .. } => (),
            Self::String { .. } => (),
            Self::Union {
                typeinfo,
                variant,
                fields,
            } => {
                for value in fields.values_mut() {
                    value.lift();
                }
                if typeinfo.name.starts_with("core::option::Option") {
                    *self = Self::Option {
                        typename: typeinfo.name.to_owned(),
                        variant: std::mem::take(variant),
                        value: if let Some(field) = fields.remove("0") {
                            Some(Box::new(field))
                        } else if fields.contains_key("<>") {
                            Some(Box::new(Self::Opaque))
                        } else {
                            None
                        },
                    }
                } else if typeinfo.name.starts_with("core::result::Result") {
                    *self = Self::Result {
                        typename: typeinfo.name.to_owned(),
                        variant: std::mem::take(variant),
                        value: Box::new(fields.remove("0").unwrap_or(Self::Opaque)),
                    }
                }
            }
            Self::Option { .. } | Self::Result { .. } => {
                // already lifted
            }
            Self::Array { data, .. } => {
                for value in data.iter_mut() {
                    value.lift();
                }
            }
            Self::Opaque => (),
        }
        Some(())
    }
}

fn take_struct_field(value: RValue, field: &str) -> Option<RValue> {
    if let RValue::Struct { mut fields, .. } = value {
        fields.remove(field)
    } else {
        None
    }
}

fn get_addr(value: &RValue) -> RefAddr {
    if let RValue::Ref { addr, .. } = value {
        *addr
    } else {
        RefAddr::Addr(Addr::from(0))
    }
}

fn take_usize(value: PValue) -> Option<u64> {
    match value {
        PValue::usize(value) => Some(value),
        _ => None,
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}
