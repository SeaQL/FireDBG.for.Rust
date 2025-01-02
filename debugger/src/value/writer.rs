use super::WriteErr;
use crate::{
    alignment_of, cache_sb_type, condense, format_value_type_as_tuple, get_layout_of, get_sb_type,
    get_union_type, parse_pair, read_process_memory, sb_value_from_addr, sb_value_from_data, Addr,
    Bytes, RVal, RValueWriter, SizeOfType, Val, ValueType, KEEP_HASH_ORDER, MAX_ARRAY_SIZE,
    RECURSIVE_DEREF_LIMIT, STD_HASH_MAP, STD_HASH_SET, STD_HASH_STATE,
};
use lldb::{IsValid, SBType, SBValue, TypeClass};
use std::ops::IndexMut;

type Result<T> = std::result::Result<T, WriteErr>;

const OPTION_BOX: &str = "core::option::Option<alloc::boxed::Box<";
const RESULT_BOX: &str = "core::result::Result<alloc::boxed::Box<";
const RESULT_T: &str = "core::result::Result<";
const OPTION_T: &str = "core::option::Option<";
const OPTION_RC: &str = "core::option::Option<alloc::rc::Rc<";
const OPTION_ARC: &str = "core::option::Option<alloc::sync::Arc<";
const BOX_DYN: &str = "alloc::boxed::Box<dyn ";
const RC_BOX: &str = "alloc::rc::RcBox<";
const RC_BOX_DYN: &str = "alloc::rc::RcBox<dyn ";
const ARC_INNER: &str = "alloc::sync::ArcInner<";
const ARC_INNER_DYN: &str = "alloc::sync::ArcInner<dyn ";
const STD_IO_ERROR: &str = "std::io::error::Error";
const STD_THREAD_JOIN_HANDLE: &str = "std::thread::JoinHandle<";

macro_rules! bail {
    () => {
        log::trace!("[writer.rs:{}] WriteErr", line!());
        return Err(WriteErr);
    };
}

/// r is recursive limit
pub(crate) fn write_value(t: &mut RValueWriter, v: &SBValue, mut r: usize) -> Result<Bytes> {
    let vtype = v.type_();
    let typename = vtype.name();
    if r == 0 {
        log::trace!("Recursive limit reached for {}", typename);
        return Ok(t.opaque_v());
    }
    r -= 1;
    if typename == "&str" {
        return Ok(t.strlit_v(&read_str(v)?));
    }
    if let Some(ty) = get_union_type(&vtype) {
        if (ty.name.starts_with(OPTION_BOX)
            && ty.name.ends_with(">>")
            && &ty.name[OPTION_BOX.len()..OPTION_BOX.len() + 4] != "dyn ")
            || (ty.name.starts_with(RESULT_BOX)
                && ty.name.ends_with(">, ()>") // Result<Box<T>, ()> is equivalent to Option<Box<T>>
                && &ty.name[RESULT_BOX.len()..RESULT_BOX.len() + 4] != "dyn ")
        {
            let (none, none_inner, pointee) = if ty.name.starts_with(OPTION_BOX) {
                (false, None, &ty.name[OPTION_BOX.len()..ty.name.len() - 2])
            } else {
                (
                    true,
                    Some(("0".to_owned(), t.unit_v())),
                    &ty.name[RESULT_BOX.len()..ty.name.len() - ">, ()>".len()],
                )
            };
            let pointee = get_sb_type(&pointee).ok_or(WriteErr)?;
            let addr = value_to_bytes::<8>(v)?;
            let addr = u64::from_ne_bytes(addr);
            return if addr == 0 {
                Ok(t.union_v(&ty, none as usize, none_inner.into_iter()))
            } else {
                let value = t.pointer_to("Box", addr, &pointee, r)?;
                Ok(t.union_v(&ty, !none as usize, [("0".to_owned(), value)].into_iter()))
            };
        } else if (ty.name.starts_with(OPTION_RC)
            && ty.name.ends_with(">>")
            && &ty.name[OPTION_RC.len()..OPTION_RC.len() + 4] != "dyn ")
            || (ty.name.starts_with(OPTION_ARC)
                && ty.name.ends_with(">>")
                && &ty.name[OPTION_ARC.len()..OPTION_ARC.len() + 4] != "dyn ")
        {
            let addr = value_to_bytes::<8>(v)?;
            let addr = u64::from_ne_bytes(addr);
            return if addr == 0 {
                Ok(t.union_v(&ty, 0, [].into_iter()))
            } else {
                // shockingly we'd have to re-construct the layout manually but Option<Rc / Arc> breaks lldb
                let (ptr_ty, pointee_name) = if ty.name.starts_with(OPTION_RC) {
                    ("rc", &ty.name[OPTION_RC.len()..ty.name.len() - 2])
                } else {
                    ("arc", &ty.name[OPTION_ARC.len()..ty.name.len() - 2])
                };
                let value = t.ref_counted_to(ptr_ty, addr, pointee_name, r)?;
                Ok(t.union_v(&ty, 1, [("0".to_owned(), value)].into_iter()))
            };
        } else {
            let mut fields = Vec::new();
            for c in v.children() {
                if let Some(name) = c.name() {
                    fields.push((name.to_string(), write_value(t, &c, r)?));
                }
            }
            let tag = typename.rsplit_once("::").ok_or(WriteErr)?.1;
            return if let Some(variant) = ty.variants.iter().position(|v| v == tag) {
                Ok(t.union_v(&ty, variant, fields.into_iter()))
            } else if ty.name.starts_with(OPTION_T) && ty.name.ends_with(">") {
                // [aarch64] std::mem::size_of::<i128>() = 16
                // [aarch64] std::mem::size_of::<Option<i128>>() = 32
                // [x64] std::mem::size_of::<i128>() = 16
                // [x64] std::mem::size_of::<Option<i128>>() = 24
                //
                // if for some reason lldb failed to determine the dynamic type
                // and we might be able to in some special cases
                let inner_type_name = &ty.name[OPTION_T.len()..ty.name.len() - 1];
                if let Some(inner) = get_sb_type(inner_type_name) {
                    if inner.byte_size() < vtype.byte_size() {
                        // there are extra bytes, must be the determinant
                        if let Ok([det]) = value_to_bytes::<1>(v) {
                            if matches!(det, 0 | 1) {
                                if let Ok(val) = write_union_with(t, &v, det as u32) {
                                    return Ok(val);
                                }
                            }
                        }
                    }
                }
                Ok(t.opaque_v())
            } else if ty.name.starts_with(RESULT_T) && ty.name.ends_with(">") {
                if let Ok(ValueType::Result(left, right)) = ty.name.parse() {
                    let mut size = 0;
                    if let SizeOfType::Sized(s) = left.size_of() {
                        size = size.max(s);
                    } else if let Some(left) = get_sb_type(&left.to_string()) {
                        size = size.max(left.byte_size() as usize);
                    }
                    if let SizeOfType::Sized(s) = right.size_of() {
                        size = size.max(s);
                    } else if let Some(right) = get_sb_type(&right.to_string()) {
                        size = size.max(right.byte_size() as usize);
                    }
                    if 0 < size && size < vtype.byte_size() as usize {
                        if let Ok([det]) = value_to_bytes::<1>(v) {
                            if matches!(det, 0 | 1) {
                                if let Ok(val) = write_union_with(t, &v, det as u32) {
                                    return Ok(val);
                                }
                            }
                        }
                    }
                }
                Ok(t.opaque_v())
            } else {
                log::trace!("Failed to write enum {}", typename);
                Ok(t.opaque_v())
            };
        }
    }
    if typename.starts_with("alloc::vec::Vec<") {
        let ptr = v
            .child_at_index(0)
            .child_at_index(0)
            .child_at_index(0)
            .child_at_index(0);
        let len = v.child_at_index(1);
        if !len.is_valid() {
            return Err(WriteErr);
        }
        let len = len.value_as_unsigned(0);
        let len = (len as usize).min(*MAX_ARRAY_SIZE);

        return Ok(if typename.ends_with("Vec<u8, alloc::alloc::Global>") {
            t.bytes_v("Vec<u8>", Bytes::from(read_bytes(&ptr, len)?))
        } else if len == 0 {
            t.vector_v([].into_iter())
        } else if let Ok(arr) = read_array(&ptr, len as u64) {
            let mut elems = Vec::with_capacity(len);
            for c in arr.children().take(*MAX_ARRAY_SIZE) {
                elems.push(write_value(t, &c, r)?);
            }
            t.vector_v(elems.into_iter())
        } else {
            t.opaque_v()
        });
    }
    if (typename.starts_with("&[") || typename.starts_with("&mut [")) && !typename.contains(';') {
        let ptr = v.child_at_index(0);
        let len = v.child_at_index(1);
        if !len.is_valid() {
            return Err(WriteErr);
        }
        let len = len.value_as_unsigned(0);
        let len = (len as usize).min(*MAX_ARRAY_SIZE);

        return Ok(if typename == "&[u8]" || typename == "&mut [u8]" {
            t.bytes_v(typename, Bytes::from(read_bytes(&ptr, len)?))
        } else if len == 0 {
            t.slice_v([].into_iter())
        } else if let Ok(arr) = read_array(&ptr, len as u64) {
            let mut elems = Vec::with_capacity(len);
            for c in arr.children().take(*MAX_ARRAY_SIZE) {
                elems.push(write_value(t, &c, r)?);
            }
            t.slice_v(elems.into_iter())
        } else {
            t.opaque_v()
        });
    }
    cache_sb_type(vtype);
    write_base_value(t, v, r + 1)
}

/// This should not call methods of `RVal`
fn write_base_value(t: &mut RValueWriter, v: &SBValue, mut r: usize) -> Result<Bytes> {
    if r == 0 {
        log::trace!("Recursive limit reached for {:?}", v.type_name());
        return Ok(t.opaque_v());
    }
    r -= 1;
    let vtype = v.type_();
    let type_class = vtype.type_class();
    let write_primitive = |ty: &str| -> Result<Bytes> {
        Ok(match v.byte_size() {
            1 => t.prim_v(ty, &value_to_bytes::<1>(v)?),
            2 => t.prim_v(ty, &value_to_bytes::<2>(v)?),
            4 => t.prim_v(ty, &value_to_bytes::<4>(v)?),
            8 => t.prim_v(ty, &value_to_bytes::<8>(v)?),
            16 => t.prim_v(ty, &value_to_bytes::<16>(v)?),
            _ => panic!("Not a primitive"),
        })
    };
    if type_class.contains(TypeClass::Builtin) {
        let ty = vtype.name();
        return Ok(if !ty.is_empty() {
            write_primitive(ty)?
        } else {
            t.opaque_v()
        });
    }
    if type_class.contains(TypeClass::Array) {
        let vtype = v.type_();
        let typename = vtype.name();
        return Ok(if typename.starts_with("[u8;") {
            let len = v.byte_size().min(*MAX_ARRAY_SIZE);
            let mut bytes = vec![0; len];
            v.data()
                .read_raw_data(0, &mut bytes)
                .map_err(|_| WriteErr)?;
            t.bytes_v(typename, Bytes::from(bytes))
        } else {
            let mut elems = Vec::with_capacity((v.num_children() as usize).min(*MAX_ARRAY_SIZE));
            for c in v.children().take(*MAX_ARRAY_SIZE) {
                elems.push(write_value(t, &c, r)?);
            }
            t.arr_v(elems.into_iter())
        });
    }
    if type_class.intersects(TypeClass::Pointer | TypeClass::Reference) {
        let raw_addr = value_to_bytes::<8>(v)?;
        let addr = Addr::new(&raw_addr);
        let raw_addr = u64::from_ne_bytes(raw_addr);
        if t.alloc_env(addr) {
            let pointee = vtype.pointee_type();
            if !pointee.is_valid() {
                bail!();
            }
            let pointee = pointee.name();
            let value = if pointee.starts_with(RC_BOX_DYN) || pointee.starts_with(ARC_INNER_DYN) {
                let ty = if pointee.starts_with(RC_BOX_DYN) {
                    "rc"
                } else {
                    "arc"
                };
                if let Some(pointee) = t.allocated_at(raw_addr) {
                    let pointee = if pointee.starts_with(RC_BOX) {
                        &pointee[RC_BOX.len()..pointee.len() - 1]
                    } else if pointee.starts_with(ARC_INNER) {
                        &pointee[ARC_INNER.len()..pointee.len() - 1]
                    } else {
                        bail!();
                    }
                    .to_owned();
                    t.ref_counted_inner(ty, raw_addr, &pointee, r)?
                } else {
                    write_value(t, &v.dereference(), r)?
                }
            } else {
                write_value(t, &v.dereference(), r)?
            };
            t.set_env(addr, value); // side effects! should never bail below this line
        }
        let tname = vtype.name();
        let ty = if tname.starts_with("&") {
            "ref"
        } else if tname.starts_with("alloc::boxed::Box<") {
            "Box"
        } else {
            "ptr"
        };
        return Ok(t.ref_v(ty, addr));
    }
    if type_class.contains(TypeClass::Struct) {
        let typename = vtype.name();
        if (typename.starts_with("&dyn ") || typename.starts_with(BOX_DYN)) && v.num_children() == 2
        {
            let mut fields = Vec::new();
            for c in v.children() {
                if let Some(name) = c.name() {
                    if name == "vtable" {
                        fields.push(("vtable".to_owned(), write_value(t, &c, r)?));
                    } else if name == "pointer" {
                        let addr = value_to_bytes::<8>(&c)?;
                        let addr = u64::from_ne_bytes(addr);
                        if let Some(pointee) = t.allocated_at(addr) {
                            if let Some(pointee) = get_sb_type(pointee) {
                                let value = t.pointer_to("ptr", addr, &pointee, r)?;
                                fields.push(("pointer".to_owned(), value));
                            }
                        }
                    }
                    if fields.len() == 2 {
                        return Ok(t.struct_v(typename, fields.into_iter()));
                    }
                }
            }
        } else if (typename.starts_with(STD_HASH_MAP) || typename.starts_with(STD_HASH_SET))
            && typename.ends_with(STD_HASH_STATE)
        {
            // specifically handle std HashMap & HashSet
            let (mut left, mut right) = (None, None);
            let bucket_type;
            let bucket_typename;
            let bucket_size;
            let bucket_align;
            if typename.starts_with(STD_HASH_MAP) {
                let pair = &typename[STD_HASH_MAP.len()..typename.len() - STD_HASH_STATE.len()];
                let pv = parse_pair(pair).map_err(|_| WriteErr)?;
                let left_ = pair[..pv].trim();
                let right_ = pair[pv + 1..].trim();
                bucket_typename = format!("({left_}, {right_})");
                bucket_type = get_sb_type(&bucket_typename);
                if let Some(bucket_type) = &bucket_type {
                    bucket_size = bucket_type.byte_size() as usize;
                    bucket_align = alignment_of(bucket_type.clone()).map_err(|_| WriteErr)?;
                } else {
                    left = Some(get_sb_type(left_).ok_or(WriteErr)?);
                    right = Some(get_sb_type(right_).ok_or(WriteErr)?);
                    let left = condense(left.clone().expect("Some")).map_err(|_| WriteErr)?;
                    let right = condense(right.clone().expect("Some")).map_err(|_| WriteErr)?;
                    let typedef = format!(
                        "type T<'a> = ({}, {});",
                        format_value_type_as_tuple(&left).replace('&', "&'a "),
                        format_value_type_as_tuple(&right).replace('&', "&'a "),
                    );
                    let layout = get_layout_of(&bucket_typename, &typedef).map_err(|_| WriteErr)?;
                    bucket_size = layout.size();
                    bucket_align = layout.align();
                };
            } else {
                bucket_typename =
                    typename[STD_HASH_SET.len()..typename.len() - STD_HASH_STATE.len()].to_owned();
                if let Some(ty) = get_sb_type(&bucket_typename) {
                    bucket_type = Some(ty.clone());
                    bucket_size = ty.byte_size() as usize;
                    bucket_align = alignment_of(ty).map_err(|_| WriteErr)?;
                } else {
                    log::trace!("Failed to write hashmap for {}", bucket_typename);
                    return Ok(t.opaque_v());
                }
            }
            let mut bytes = vec![0u8; vtype.byte_size() as usize];
            if bytes.len() != std::mem::size_of::<frozen_hashbrown::HashMap>() {
                log::trace!("frozen_hashbrown size mismatch");
                return Ok(t.opaque_v());
            }
            v.data()
                .read_raw_data(0, &mut bytes)
                .map_err(|_| WriteErr)?;
            // this assumes that the hashbrown we are using has the exact same layout as user's code
            let hashmap: &frozen_hashbrown::HashMap =
                unsafe { core::mem::transmute(bytes.as_ptr() as *const _) };
            let table_layout = frozen_hashbrown::TableLayout::new(
                core::alloc::Layout::from_size_align(bucket_size, bucket_align)
                    .map_err(|_| WriteErr)?,
            );
            let allocation = hashmap.table.table.reallocation(&table_layout);
            if allocation.is_none() {
                // if for some reason we are unable to reconstruct
                return Ok(t.struct_v(
                    typename,
                    [
                        ("items".to_owned(), t.opaque_v()),
                        ("len".to_owned(), t.opaque_v()),
                    ]
                    .into_iter(),
                ));
            }
            if hashmap.len() == 0 {
                return Ok(t.struct_v(
                    typename,
                    [
                        ("items".to_owned(), t.vector_v([].into_iter())),
                        (
                            "len".to_owned(),
                            t.prim_v("usize", &hashmap.len().to_ne_bytes()),
                        ),
                    ]
                    .into_iter(),
                ));
            }
            let (offset, layout) = allocation.expect("Already checked");
            let base_ptr = hashmap.table.table.ctrl.as_ptr() as u64;
            // read the control bytes
            let control = read_process_memory(base_ptr, layout.size() - offset)?;
            let mut items = hashmap.len().min(*MAX_ARRAY_SIZE);
            let mut elems = Vec::with_capacity(items);

            let right_at = if let Some(left) = &left {
                let left_size = left.byte_size() as usize;
                // round up to the next alignment
                left_size
                    + if left_size % bucket_align == 0 {
                        0
                    } else {
                        bucket_align - (left_size % bucket_align)
                    }
            } else {
                0
            } as u64;

            for (i, ctrl) in control.into_iter().enumerate() {
                // most significant bit = 0 means bucket is full
                if (ctrl & 0x80) == 0 {
                    // [Padding], Tlast, ..., T1, T0, C0, C1, ..., Clast
                    let bucket_addr = base_ptr - (i as u64 + 1) * table_layout.size as u64;
                    if let Some(bucket_type) = &bucket_type {
                        // this should be safer
                        let sb_value = sb_value_from_addr("i", bucket_addr, bucket_type)?;
                        elems.push(write_value(t, &sb_value, r)?);
                    } else if let (Some(left), Some(right)) = (&left, &right) {
                        // we figure out value alignment on our own; can be wrong
                        let sb_value = sb_value_from_addr("i", bucket_addr, left)?;
                        let left_val = write_value(t, &sb_value, r)?;
                        let sb_value = sb_value_from_addr("j", bucket_addr + right_at, right)?;
                        let right_val = write_value(t, &sb_value, r)?;
                        elems.push(t.struct_v(
                            &bucket_typename,
                            [("0".to_owned(), left_val), ("1".to_owned(), right_val)].into_iter(),
                        ));
                    } else {
                        break;
                    }
                    items -= 1;
                    if items == 0 {
                        break;
                    }
                }
            }
            if !*KEEP_HASH_ORDER {
                elems.sort();
            }
            return Ok(t.struct_v(
                typename,
                [
                    ("items".to_owned(), t.vector_v(elems.into_iter())),
                    (
                        "len".to_owned(),
                        t.prim_v("usize", &hashmap.len().to_ne_bytes()),
                    ),
                ]
                .into_iter(),
            ));
        } else if typename.starts_with(STD_THREAD_JOIN_HANDLE) {
            // TODO what useful info can we capture?
            return Ok(t.struct_v(typename, [].into_iter()));
        } else if typename == STD_IO_ERROR {
            // Capture std::io::Error according to https://doc.rust-lang.org/src/std/io/error/repr_bitpacked.rs.html
            let addr = value_to_bytes::<8>(v)?;
            let addr = u64::from_ne_bytes(addr);
            const TAG_MASK: u64 = 0b11;
            const TAG_SIMPLE_MESSAGE: u64 = 0b00;
            const TAG_CUSTOM: u64 = 0b01;
            const TAG_OS: u64 = 0b10;
            const TAG_SIMPLE: u64 = 0b11;
            let ekind = get_sb_type("std::io::error::ErrorKind").ok_or(WriteErr)?;
            let error_kind = |kind: u64| {
                let kind = sb_value_from_data("kind", &[kind], &ekind)?;
                let (etype, variant) = enumerate_value(&ekind, &kind)?;
                Ok(t.enumerate_v(etype, variant))
            };
            return match addr & TAG_MASK {
                TAG_OS => {
                    // the high 32 bit is the OS Error code
                    let code = ((addr >> 32) as u32) as i32;
                    // this depends on the platform where debugger is compiled
                    let err = std::io::Error::from_raw_os_error(code);
                    let kind = err.kind() as u64;
                    let kind = error_kind(kind)?;
                    let mess = err.kind().to_string();
                    let mess = t.strlit_v(mess.as_bytes());
                    Ok(t.struct_v(
                        typename,
                        [
                            ("code".to_owned(), t.prim_v("i32", &code.to_ne_bytes())),
                            ("kind".to_owned(), kind),
                            ("message".to_owned(), mess),
                        ]
                        .into_iter(),
                    ))
                }
                TAG_SIMPLE_MESSAGE => {
                    // #[repr(align(4))]
                    // struct SimpleMessage {
                    //     kind: ErrorKind,
                    //     message: &'static str,
                    // }
                    let kind = sb_value_from_addr("0", addr, &ekind)?;
                    let (etype, variant) = enumerate_value(&ekind, &kind)?;
                    let kind = t.enumerate_v(etype, variant);
                    let sstr = get_sb_type("&str").ok_or(WriteErr)?;
                    let mess = sb_value_from_addr("1", addr + 4, &sstr)?;
                    let mess = t.strlit_v(&read_str(&mess)?);
                    Ok(t.struct_v(
                        typename,
                        [("kind".to_owned(), kind), ("message".to_owned(), mess)].into_iter(),
                    ))
                }
                TAG_SIMPLE | TAG_CUSTOM => {
                    // #[repr(align(4))]
                    // struct Custom {
                    //     kind: ErrorKind,
                    //     error: Box<dyn error::Error + Send + Sync>,
                    // }
                    let kind = ((addr >> 32) as u32) as u64;
                    let kind = error_kind(kind)?;
                    Ok(t.struct_v(typename, [("kind".to_owned(), kind)].into_iter()))
                }
                _ => {
                    // Shouldn't happen; but just in case
                    Ok(t.opaque_v())
                }
            };
        }
        let mut fields = Vec::new();
        for c in v.children() {
            if let Some(name) = c.name() {
                fields.push((name.to_string(), write_value(t, &c, r)?));
            }
        }
        return Ok(t.struct_v(typename, fields.into_iter()));
    }
    if type_class.contains(TypeClass::Enumeration) {
        let (typename, variant) = enumerate_value(&vtype, v)?;
        return Ok(t.enumerate_v(typename, variant));
    }
    let ty = vtype.name();
    Ok(if !ty.is_empty() {
        write_primitive(ty)?
    } else {
        t.opaque_v()
    })
}

pub(crate) fn write_union_with(
    wt: &mut RValueWriter,
    v: &SBValue,
    discriminant: u32,
) -> Result<Bytes> {
    let r = *RECURSIVE_DEREF_LIMIT;
    let vtype = v.type_();
    let typename = vtype.name();
    let ty = get_union_type(&vtype).unwrap_or_else(|| panic!("type {vtype:?} is not Union"));
    let tag = typename.rsplit_once("::").ok_or(WriteErr)?.1;
    return if let Some(variant) = ty.variants.iter().position(|v| v == tag) {
        let mut fields = Vec::new();
        for c in v.children() {
            if let Some(name) = c.name() {
                fields.push((name.to_string(), write_value(wt, &c, r)?));
            }
        }
        Ok(wt.union_v(&ty, variant, fields.into_iter()))
    } else {
        // this is the case where sb_value for some reason cannot determine the variant
        // so we have to fallback on what the caller thinks is the variant
        let v = v.child_at_index(discriminant);
        if !v.is_valid() {
            bail!();
        }
        let mut fields = Vec::new();
        for c in v.children() {
            if let Some(name) = c.name() {
                fields.push((name.to_string(), write_value(wt, &c, r)?));
            }
        }
        Ok(wt.union_v(&ty, discriminant as usize, fields.into_iter()))
    };
}

fn read_array(ptr: &SBValue, len: u64) -> Result<SBValue> {
    if !ptr.is_valid() {
        return Err(WriteErr);
    }
    let arr_t = ptr.type_().pointee_type().array_type(len);
    let addr = ptr.dereference().address().ok_or(WriteErr)?;
    let value = ptr
        .target()
        .create_value_from_address("<arr>", &addr, &arr_t);
    if value.is_valid() {
        Ok(value)
    } else {
        Err(WriteErr)
    }
}

pub(crate) fn read_str(v: &SBValue) -> Result<Vec<u8>> {
    let ptr = v.child_at_index(0);
    let len = v.child_at_index(1);
    if !len.is_valid() {
        return Err(WriteErr);
    }
    let len = len.value_as_unsigned(0);
    let len = (len as usize).min(*MAX_ARRAY_SIZE);
    read_bytes(&ptr, len)
}

fn read_bytes(ptr: &SBValue, len: usize) -> Result<Vec<u8>> {
    if len == 0 {
        Ok(Vec::new())
    } else {
        if !ptr.is_valid() {
            return Err(WriteErr);
        }
        let addr = ptr.value_as_unsigned(0);
        read_process_memory(addr, len)
    }
}

pub(crate) fn enumerate_value<'a>(
    vtype: &'a SBType,
    value: &'a SBValue,
) -> Result<(&'a str, &'a str)> {
    let parent = vtype.name();
    let variant = value_to_str(value)?;
    if !variant.starts_with(parent) {
        // (invalid enum value) 3
        bail!();
    }
    if parent.len() + 2 >= variant.len() {
        bail!();
    }
    let variant = &variant[parent.len() + 2..];
    Ok((parent, variant))
}

fn value_to_str(v: &SBValue) -> Result<&str> {
    match v.value() {
        Some(s) => s.to_str().map_err(|_| WriteErr),
        None => Err(WriteErr),
    }
}

pub(crate) fn value_to_bytes<const N: usize>(v: &SBValue) -> Result<[u8; N]> {
    let mut bytes = [0; N];
    v.data()
        .read_raw_data(0, bytes.as_mut())
        .map_err(|_| WriteErr)?;
    Ok(bytes)
}

pub(crate) fn values_to_bytes<const N: usize, I>(mut vals: I, i: usize) -> Result<[u8; N]>
where
    I: Iterator<Item = SBValue>,
{
    let mut bytes = [0; N];
    let offset = N / i;
    for i in 0..i {
        let v = vals.next().ok_or(WriteErr)?;
        let l = offset * i;
        let h = l + offset;
        v.data()
            .read_raw_data(0, bytes.index_mut(l..h))
            .map_err(|_| WriteErr)?;
    }
    Ok(bytes)
}

impl RValueWriter<'_> {
    fn pointer_to(&mut self, ty: &str, addr: u64, pointee: &SBType, r: usize) -> Result<Bytes> {
        let sb_value = sb_value_from_addr("0", addr, pointee)?;
        let addr = Addr::from(addr);
        self.alloc_env(addr); // it's very important to alloc first before writing value
        let value = write_value(self, &sb_value, r)?;
        self.set_env(addr, value); // put this value on the env
        Ok(self.ref_v(ty, addr))
    }

    fn ref_counted_to(
        &mut self,
        ty: &str,
        addr: u64,
        pointee_name: &str,
        r: usize,
    ) -> Result<Bytes> {
        let (outer_type, inner_type) = match ty {
            "rc" => ("alloc::rc::Rc", "alloc::rc::RcBox"),
            "arc" => ("alloc::sync::Arc", "alloc::sync::ArcInner"),
            _ => panic!("Unexpected {ty}"),
        };
        let inner = self.ref_counted_inner(ty, addr, pointee_name, r)?;
        // put this value on the env
        let addr = Addr::from(addr);
        self.set_env(addr, inner);
        // see ref.js on what we're trying to construct
        let value = self.ref_v("ptr", addr);
        let value = self.struct_v(
            &format!(
                "core::ptr::non_null::NonNull<{}<{}>>",
                inner_type, pointee_name
            ),
            [("pointer".to_owned(), value)].into_iter(),
        );
        Ok(self.struct_v(
            &format!("{}<{}>", outer_type, pointee_name),
            [("ptr".to_owned(), value)].into_iter(),
        ))
    }

    fn ref_counted_inner(
        &mut self,
        ty: &str,
        addr: u64,
        pointee_name: &str,
        r: usize,
    ) -> Result<Bytes> {
        let (counter_type, inner_type, data_field) = match ty {
            "rc" => ("core::cell::Cell<usize>", "alloc::rc::RcBox", "value"),
            "arc" => (
                "core::sync::atomic::AtomicUsize",
                "alloc::sync::ArcInner",
                "data",
            ),
            _ => panic!("Unexpected {ty}"),
        };
        let pointee = get_sb_type(pointee_name).ok_or(WriteErr)?;
        let counter = get_sb_type(counter_type).ok_or(WriteErr)?;
        let strong = sb_value_from_addr("0", addr, &counter)?;
        let ptr_size = core::mem::size_of::<usize>() as u64;
        let weak = sb_value_from_addr("0", addr + ptr_size, &counter)?;
        let sb_value = sb_value_from_addr("0", addr + 2 * ptr_size, &pointee)?;
        let addr = Addr::from(addr);
        self.alloc_env(addr); // it's very important to alloc first before writing value
        let strong = write_value(self, &strong, 3)?;
        let weak = write_value(self, &weak, 3)?;
        let value = write_value(self, &sb_value, r)?;
        Ok(self.struct_v(
            &format!("{}<{}>", inner_type, pointee_name),
            [
                ("strong".to_owned(), strong),
                ("weak".to_owned(), weak),
                (data_field.to_owned(), value),
            ]
            .into_iter(),
        ))
    }
}
