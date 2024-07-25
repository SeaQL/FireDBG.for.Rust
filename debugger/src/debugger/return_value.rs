use super::{get_union_type, read_u64};
use crate::{
    boildown, enumerate_value, get_ref_counted_pointee,
    value::{value_to_bytes, values_to_bytes, write_union_with, RVal, Val},
    Addr, Bytes, RValueWriter, SizeOfType, ValueType, WriteErr, MAX_ARRAY_SIZE,
};
use lldb::{IsValid, SBAddress, SBData, SBFrame, SBProcess, SBTarget, SBType, SBValue, TypeClass};
use std::{ops::Deref, rc::Rc};

pub const RETVAL: &'static str = "return_value";

pub(super) fn write_return_value(
    event: &mut Bytes,
    rwriter: &mut RValueWriter,
    sb_target: &SBTarget,
    sb_process: &SBProcess,
    sb_frame: &SBFrame,
    return_type: &SBType,
) -> Result<(), WriteErr> {
    let rax = || {
        sb_frame.find_register({
            #[cfg(target_arch = "x86_64")]
            {
                "rax"
            }
            #[cfg(target_arch = "aarch64")]
            {
                "x0"
            }
        })
    };
    let rdx = || {
        sb_frame.find_register({
            #[cfg(target_arch = "x86_64")]
            {
                "rdx"
            }
            #[cfg(target_arch = "aarch64")]
            {
                "x1"
            }
        })
    };
    #[cfg(target_arch = "x86_64")]
    let rcx = || sb_frame.find_register("rcx");
    let reg = |reg: &str| sb_frame.find_register(reg);
    let xmm0 = || {
        sb_frame.find_register({
            #[cfg(target_arch = "x86_64")]
            {
                "xmm0"
            }
            #[cfg(target_arch = "aarch64")]
            {
                "v0"
            }
        })
    };
    let xmm1 = || {
        sb_frame.find_register({
            #[cfg(target_arch = "x86_64")]
            {
                "xmm1"
            }
            #[cfg(target_arch = "aarch64")]
            {
                "v1"
            }
        })
    };

    let get_prim_from_rax_rdx =
        |rwriter: &RValueWriter, value_type: &ValueType| -> Result<Bytes, WriteErr> {
            Ok(match value_type {
                ValueType::Unit => rwriter.unit_v(),
                ValueType::bool => {
                    let v = read_u64(&rax())?;
                    rwriter.prim_v("bool", &[if v != 0 { 1 } else { 0 }])
                }
                ValueType::char => {
                    let v = &value_to_bytes::<4>(&rax())?;
                    rwriter.prim_v("char", v)
                }
                ValueType::u8 => {
                    let v = &value_to_bytes::<1>(&rax())?;
                    rwriter.prim_v("u8", v)
                }
                ValueType::i8 => {
                    let v = &value_to_bytes::<1>(&rax())?;
                    rwriter.prim_v("i8", v)
                }
                ValueType::u16 => {
                    let v = &value_to_bytes::<2>(&rax())?;
                    rwriter.prim_v("u16", v)
                }
                ValueType::i16 => {
                    let v = &value_to_bytes::<2>(&rax())?;
                    rwriter.prim_v("i16", v)
                }
                ValueType::u32 => {
                    let v = &value_to_bytes::<4>(&rax())?;
                    rwriter.prim_v("u32", v)
                }
                ValueType::i32 => {
                    let v = &value_to_bytes::<4>(&rax())?;
                    rwriter.prim_v("i32", v)
                }
                ValueType::u64 => {
                    let v = &value_to_bytes::<8>(&rax())?;
                    rwriter.prim_v("u64", v)
                }
                ValueType::i64 => {
                    let v = &value_to_bytes::<8>(&rax())?;
                    rwriter.prim_v("i64", v)
                }
                ValueType::u128 => {
                    let v = &values_to_bytes::<16, _>([rax(), rdx()].into_iter(), 2)?;
                    rwriter.prim_v("u128", v)
                }
                ValueType::i128 => {
                    let v = &values_to_bytes::<16, _>([rax(), rdx()].into_iter(), 2)?;
                    rwriter.prim_v("i128", v)
                }
                ValueType::usize => {
                    let v = &value_to_bytes::<8>(&rax())?;
                    rwriter.prim_v("usize", v)
                }
                ValueType::isize => {
                    let v = &value_to_bytes::<8>(&rax())?;
                    rwriter.prim_v("isize", v)
                }
                ValueType::f32 => {
                    let v = &values_to_bytes::<4, _>(xmm0().children(), 4)?;
                    rwriter.prim_v("f32", v)
                }
                ValueType::f64 => {
                    let v = &values_to_bytes::<8, _>(xmm0().children(), 8)?;
                    rwriter.prim_v("f64", v)
                }
                _ => panic!("Not primitive"),
            })
        };

    let get_prim_from_rdx =
        |rwriter: &RValueWriter, value_type: &ValueType| -> Result<Bytes, WriteErr> {
            Ok(match value_type {
                ValueType::Unit => rwriter.unit_v(),
                ValueType::bool => {
                    let v = read_u64(&rdx())?;
                    rwriter.prim_v("bool", &[if v != 0 { 1 } else { 0 }])
                }
                ValueType::char => {
                    let v = &value_to_bytes::<4>(&rdx())?;
                    rwriter.prim_v("char", v)
                }
                ValueType::u8 => {
                    let v = &value_to_bytes::<1>(&rdx())?;
                    rwriter.prim_v("u8", v)
                }
                ValueType::i8 => {
                    let v = &value_to_bytes::<1>(&rdx())?;
                    rwriter.prim_v("i8", v)
                }
                ValueType::u16 => {
                    let v = &value_to_bytes::<2>(&rdx())?;
                    rwriter.prim_v("u16", v)
                }
                ValueType::i16 => {
                    let v = &value_to_bytes::<2>(&rdx())?;
                    rwriter.prim_v("i16", v)
                }
                ValueType::u32 => {
                    let v = &value_to_bytes::<4>(&rdx())?;
                    rwriter.prim_v("u32", v)
                }
                ValueType::i32 => {
                    let v = &value_to_bytes::<4>(&rdx())?;
                    rwriter.prim_v("i32", v)
                }
                ValueType::u64 => {
                    let v = &value_to_bytes::<8>(&rdx())?;
                    rwriter.prim_v("u64", v)
                }
                ValueType::i64 => {
                    let v = &value_to_bytes::<8>(&rdx())?;
                    rwriter.prim_v("i64", v)
                }
                ValueType::usize => {
                    let v = &value_to_bytes::<8>(&rdx())?;
                    rwriter.prim_v("usize", v)
                }
                ValueType::isize => {
                    let v = &value_to_bytes::<8>(&rdx())?;
                    rwriter.prim_v("isize", v)
                }
                ValueType::f32 => {
                    let v = &values_to_bytes::<4, _>(xmm0().children(), 4)?;
                    rwriter.prim_v("f32", v)
                }
                ValueType::f64 => {
                    let v = &values_to_bytes::<8, _>(xmm0().children(), 8)?;
                    rwriter.prim_v("f64", v)
                }
                _ => panic!("Not primitive"),
            })
        };

    let get_float_from_xmm0 =
        |rwriter: &RValueWriter, value_type: &ValueType| -> Result<Bytes, WriteErr> {
            Ok(match value_type {
                ValueType::f32 => {
                    let v = &values_to_bytes::<4, _>(xmm0().children(), 4)?;
                    rwriter.prim_v("f32", v)
                }
                ValueType::f64 => {
                    let v = &values_to_bytes::<8, _>(xmm0().children(), 8)?;
                    rwriter.prim_v("f64", v)
                }
                _ => panic!("Not primitive"),
            })
        };

    let get_float_from_xmm1 =
        |rwriter: &RValueWriter, value_type: &ValueType| -> Result<Bytes, WriteErr> {
            Ok(match value_type {
                ValueType::f32 => {
                    let v = &values_to_bytes::<4, _>(xmm1().children(), 4)?;
                    rwriter.prim_v("f32", v)
                }
                ValueType::f64 => {
                    let v = &values_to_bytes::<8, _>(xmm1().children(), 8)?;
                    rwriter.prim_v("f64", v)
                }
                _ => panic!("Not primitive"),
            })
        };

    let ref_value_from_addr =
        |rwriter: &mut RValueWriter, addr: u64, pointer: &SBType| -> Result<Bytes, WriteErr> {
            let ty = pointer_typename(pointer.name());

            let (ptr, pointee) = match ty {
                "ref" | "Box" | "ptr" => (None, pointer.pointee_type()),
                "Rc" | "Arc" => {
                    let (ptr, pointee) = get_ref_counted_pointee(pointer)?;
                    (Some(ptr), pointee)
                }
                _ => unreachable!(),
            };

            if !pointee.is_valid() {
                return Err(WriteErr);
            }
            let sb_addr = SBAddress::from_load_address(addr, sb_target);
            let sb_value = sb_target.create_value_from_address(RETVAL, &sb_addr, &pointee);
            let addr = Addr::from(addr);
            rwriter.alloc_env(addr); // it's very important to alloc first before writing value
                                     // this ensures the referential order if the value itself is a reference
            let value = rwriter.write_value(&sb_value)?;
            rwriter.set_env(addr, value);
            if let Some(ptr) = ptr {
                let val = rwriter.ref_v("ptr", addr);
                let val = rwriter.struct_v(ptr.name(), [("pointer".to_owned(), val)].into_iter());
                Ok(rwriter.struct_v(pointer.name(), [("ptr".to_owned(), val)].into_iter()))
            } else {
                Ok(rwriter.ref_v(ty, addr))
            }
        };

    let read_str_from_addr = |addr: u64, len: usize| -> Result<Vec<u8>, WriteErr> {
        let mut buf = vec![0u8; len.min(*MAX_ARRAY_SIZE)];
        sb_process
            .read_memory(addr, &mut buf)
            .map_err(|_| WriteErr)?;
        Ok(buf)
    };

    #[cfg(target_arch = "aarch64")]
    let read_byte_from_addr = |addr: u64| -> Result<u8, WriteErr> {
        let mut buf = [0u8; 1];
        sb_process
            .read_memory(addr, &mut buf)
            .map_err(|_| WriteErr)?;
        Ok(buf[0])
    };

    let get_reference_from_rax_rdx = |rwriter: &mut RValueWriter,
                                      value_type: &ValueType,
                                      pointer_type: &SBType|
     -> Result<Bytes, WriteErr> {
        match value_type {
            ValueType::Reference(r) => match r.deref() {
                ValueType::str => {
                    // &str is a fat pointer
                    let addr = read_u64(&rax())?; // one half is the address
                    let len = read_u64(&rdx())? as usize; // one half is the length
                    let bytes = read_str_from_addr(addr, len)?;
                    Ok(rwriter.strlit_v(&bytes))
                }
                _ => {
                    let addr = read_u64(&rax())?;
                    ref_value_from_addr(rwriter, addr, pointer_type)
                }
            },
            _ => panic!("Not reference"),
        }
    };

    let get_slice_from_rax_rdx = |rwriter: &mut RValueWriter,
                                  value_type: &ValueType,
                                  pointer_type: &SBType|
     -> Result<Bytes, WriteErr> {
        match value_type {
            ValueType::Slice(_) => {
                let addr = read_u64(&rax())?; // one half is the address
                let len = read_u64(&rdx())?; // one half is the length
                let sb_data = SBData::from_u64(
                    &[addr, len],
                    sb_target.byte_order(),
                    sb_target.address_byte_size() as u32,
                );
                if !sb_data.is_valid() {
                    return Err(WriteErr);
                }
                let sb_value = sb_target.create_value_from_data(RETVAL, &sb_data, pointer_type);
                if !sb_value.is_valid() {
                    return Err(WriteErr);
                }
                rwriter.write_value(&sb_value)
            }
            _ => panic!("Not slice"),
        }
    };

    let get_reference_from_rdx = |rwriter: &mut RValueWriter,
                                  value_type: &ValueType,
                                  pointer_type: &SBType|
     -> Result<Bytes, WriteErr> {
        match value_type {
            ValueType::Reference(r) => match r.deref() {
                ValueType::str => {
                    panic!("Cannot get &str from rdx");
                }
                ValueType::DynRef(_) => {
                    panic!("Cannot get &dyn from rdx");
                }
                _ => {
                    let addr = read_u64(&rdx())?;
                    ref_value_from_addr(rwriter, addr, pointer_type)
                }
            },
            _ => panic!("Not reference"),
        }
    };

    let sb_value_from_addr = |addr: u64, sb_type: &SBType| -> Result<SBValue, WriteErr> {
        super::sb_value_from_addr(RETVAL, addr, sb_type)
    };

    let sb_value_from_rax = |sb_type: &SBType| -> Result<SBValue, WriteErr> {
        sb_value_from_addr(read_u64(&rax())?, sb_type)
    };

    let create_data_from_u64 = |data: u64| -> Result<SBData, WriteErr> {
        let data = SBData::from_u64(
            &[data],
            sb_target.byte_order(),
            sb_target.address_byte_size() as u32,
        );
        if !data.is_valid() {
            return Err(WriteErr);
        }
        Ok(data)
    };

    let dyn_ref_value_from_rax_rdx = |rwriter: &mut RValueWriter,
                                      value_type: &ValueType,
                                      sb_type: &SBType|
     -> Result<Bytes, WriteErr> {
        match value_type {
            ValueType::DynRef(_) => {
                let addr = Addr::from(read_u64(&rax())?);
                rwriter.alloc_env(addr);
                rwriter.set_env(addr, rwriter.opaque_v());
                let pointer = rwriter.ref_v("ptr", addr);

                let vtable = Addr::from(read_u64(&rdx())?);
                rwriter.alloc_env(vtable);
                rwriter.set_env(vtable, rwriter.opaque_v());
                let vtable = rwriter.ref_v("ref", vtable);

                let typename = sb_type.name();
                if typename.starts_with("alloc::rc::Rc<")
                    || typename.starts_with("alloc::sync::Arc<")
                {
                    // ptr -> pointer -> pointer
                    let field = sb_type.field_at_index(0);
                    assert_eq!(field.name(), "ptr");
                    let ptr = field.type_();
                    if !ptr.is_valid() {
                        return Err(WriteErr);
                    }
                    let field = ptr.field_at_index(0);
                    assert_eq!(field.name(), "pointer");
                    let pointer_ty = field.type_();
                    if !pointer_ty.is_valid() {
                        return Err(WriteErr);
                    }
                    let field = pointer_ty.field_at_index(0);
                    assert_eq!(field.name(), "pointer");
                    let inner = field.type_();
                    if !inner.is_valid() {
                        return Err(WriteErr);
                    }
                    let pointee = inner.pointee_type();
                    // alloc::rc::RcBox / alloc::sync::ArcInner
                    if !pointee.is_valid() {
                        return Err(WriteErr);
                    }
                    let value = sb_value_from_rax(&pointee)?;
                    let value = rwriter.write_value(&value)?;
                    rwriter.set_env(addr, value);
                    let pointer = rwriter.struct_v(
                        pointer_ty.name(),
                        [
                            ("pointer".to_owned(), pointer),
                            ("vtable".to_owned(), vtable),
                        ]
                        .into_iter(),
                    );
                    let ptr =
                        rwriter.struct_v(ptr.name(), [("pointer".to_owned(), pointer)].into_iter());
                    // see dyn_ref.js for what we are re-constructing
                    Ok(rwriter.struct_v(typename, [("ptr".to_owned(), ptr)].into_iter()))
                } else {
                    // Box or &dyn
                    Ok(rwriter.struct_v(
                        typename,
                        [
                            ("pointer".to_owned(), pointer),
                            ("vtable".to_owned(), vtable),
                        ]
                        .into_iter(),
                    ))
                }
            }
            _ => panic!("Not DynRef"),
        }
    };

    let enumerate_from_rax = |sb_type: &SBType| -> Result<Bytes, WriteErr> {
        if sb_type.byte_size() <= 8 {
            let data = create_data_from_u64(read_u64(&rax())?)?;
            let value = sb_target.create_value_from_data(RETVAL, &data, &sb_type);
            if !value.is_valid() {
                return Err(WriteErr);
            }
            let (typename, variant) = enumerate_value(&sb_type, &value)?;
            Ok(rwriter.enumerate_v(typename, variant))
        } else {
            Err(WriteErr)
        }
    };

    let final_pass_by_stack = |return_type: &SBType| -> Result<SBValue, WriteErr> {
        #[cfg(target_arch = "x86_64")]
        {
            log::trace!("{} pass by stack (rax)", return_type.name());
            sb_value_from_rax(return_type)
        }
        #[cfg(target_arch = "aarch64")]
        {
            // it is unwise to assume valid address is above certain range but this is only MacOS for now
            let mut sb_value = Err(WriteErr);
            let x8 = read_u64(&reg("x8"))?;
            if x8 > 0xffff && sb_value.is_err() {
                sb_value = sb_value_from_addr(x8, return_type);
                if sb_value.is_ok() {
                    log::trace!("{} pass by stack (x8)", return_type.name());
                }
            }
            let x9 = read_u64(&reg("x9"))?;
            if x9 > 0xffff && sb_value.is_err() {
                sb_value = sb_value_from_addr(x9, return_type);
                if sb_value.is_ok() {
                    log::trace!("{} pass by stack (x9)", return_type.name());
                }
            }
            let x0 = read_u64(&reg("x0"))?;
            if x0 > 0xffff && sb_value.is_err() {
                sb_value = sb_value_from_addr(x0, return_type);
                if sb_value.is_ok() {
                    log::trace!("{} pass by stack (x0)", return_type.name());
                }
            }
            sb_value
        }
    };

    let write_array = |rwriter: &mut RValueWriter,
                       event: &mut Bytes,
                       value_type: &ValueType,
                       sb_type: &SBType|
     -> Result<(), WriteErr> {
        let sb_value = match value_type.size_of() {
            SizeOfType::Sized(size) if size <= 8 => {
                log::trace!("{} pass by register (rax)", sb_type.name());
                let data = value_to_bytes::<8>(&rax())?;
                let sb_data = SBData::borrow_bytes(
                    &data,
                    sb_target.byte_order(),
                    sb_target.address_byte_size(),
                );
                if !sb_data.is_valid() {
                    return Err(WriteErr);
                }
                let sb_value = sb_target.create_value_from_data(RETVAL, &sb_data, sb_type);
                if !sb_value.is_valid() {
                    return Err(WriteErr);
                }
                sb_value
            }
            _ => final_pass_by_stack(sb_type)?,
        };
        event.write_sb_value(rwriter, &sb_value);
        Ok(())
    };

    let value_type = return_type.name().parse().map_err(|_| WriteErr)?;
    match value_type {
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
        | ValueType::usize
        | ValueType::isize
        | ValueType::f32
        | ValueType::f64 => {
            let val = get_prim_from_rax_rdx(rwriter, &value_type)?;
            event.write_value(rwriter, RETVAL, val.as_bytes());
            Ok(())
        }
        ValueType::str => {
            panic!("There shouldn't be any naked str");
        }
        ValueType::Reference(_) => {
            let value = get_reference_from_rax_rdx(rwriter, &value_type, &return_type)?;
            event.write_value(rwriter, RETVAL, value.as_bytes());
            Ok(())
        }
        ValueType::DynRef(_) => {
            let value = dyn_ref_value_from_rax_rdx(rwriter, &value_type, &return_type)?;
            event.write_value(rwriter, RETVAL, value.as_bytes());
            Ok(())
        }
        ValueType::Option(left) => {
            assert_eq!(return_type.number_of_fields(), 2);
            let left_type = return_type
                .field_at_index(1)
                .type_()
                .field_at_index(0)
                .type_();
            let left = boildown(rc_into_inner(left)?, left_type.clone())?;
            match left.size_of() {
                SizeOfType::Sized(left_size) if left_size <= 16 => {
                    // through register
                    let opt = read_u64(&rax())?;
                    let opt_lsb = opt & 0xFF; // Least significant bytes
                    if matches!(left, ValueType::bool) {
                        // this is a Rust trick for Option<bool>
                        let val = match opt_lsb {
                            0 => Some(rwriter.prim_v("bool", &[0])),
                            1 => Some(rwriter.prim_v("bool", &[1])),
                            2 => None,
                            _ => return Err(WriteErr),
                        };
                        event.write_option(rwriter, RETVAL, return_type.name(), val);
                    } else if matches!(left, ValueType::char) {
                        // this is a Rust trick for Option<char>
                        let val = if opt <= (char::MAX as u64) {
                            Some(rwriter.prim_v("char", &opt.to_ne_bytes()[..4]))
                        } else {
                            None
                        };
                        event.write_option(rwriter, RETVAL, return_type.name(), val);
                    } else if left.is_primitive() && left_size <= 8 {
                        let value = if opt_lsb != 0 {
                            Some(get_prim_from_rdx(rwriter, &left)?)
                        } else {
                            None
                        };
                        event.write_option(rwriter, RETVAL, return_type.name(), value);
                    } else if matches!(left, ValueType::i128 | ValueType::u128) {
                        let val = {
                            #[cfg(target_arch = "x86_64")]
                            {
                                let rdx_u64 = read_u64(&rdx())?;
                                let rcx_u64 = read_u64(&rcx())?;
                                let res_is_rcx = (rdx_u64 == 0 && rcx_u64 == 1)
                                    || (rdx_u64 == 0 && rcx_u64 == 0);
                                if res_is_rcx && (read_u64(&rcx())? & 0xF) != 0 {
                                    let v = {
                                        let v = [reg("rsi"), reg("r8")];
                                        &values_to_bytes::<16, _>(v.into_iter(), 2)?
                                    };
                                    Some(rwriter.prim_v(left.primitive_name(), v))
                                } else if !res_is_rcx && (read_u64(&rdx())? & 0xF) != 0 {
                                    let v = {
                                        let v = [reg("rdi"), reg("r8")];
                                        &values_to_bytes::<16, _>(v.into_iter(), 2)?
                                    };
                                    Some(rwriter.prim_v(left.primitive_name(), v))
                                } else {
                                    None
                                }
                            }
                            #[cfg(target_arch = "aarch64")]
                            {
                                if opt != 0 {
                                    let v = {
                                        let v = [reg("x2"), reg("x3")];
                                        &values_to_bytes::<16, _>(v.into_iter(), 2)?
                                    };
                                    Some(rwriter.prim_v(left.primitive_name(), v))
                                } else {
                                    None
                                }
                            }
                        };
                        event.write_option(rwriter, RETVAL, return_type.name(), val);
                    } else if opt != 0 {
                        let val = if left_size == 0 {
                            rwriter.unit_v()
                        } else if matches!(left, ValueType::Slice(_)) {
                            get_slice_from_rax_rdx(rwriter, &left, &left_type)?
                        } else if left.is_thin_ptr() {
                            // pointer is non-zero, so discriminant and addr is packed together
                            let addr = opt;
                            ref_value_from_addr(rwriter, addr, &left_type)?
                        } else if left.is_str() {
                            // pointer is non-zero, so discriminant and addr is packed together
                            let addr = opt;
                            let len = read_u64(&rdx())? as usize;
                            let bytes = read_str_from_addr(addr, len)?;
                            rwriter.strlit_v(&bytes)
                        } else if matches!(left, ValueType::DynRef(_)) {
                            dyn_ref_value_from_rax_rdx(rwriter, &left, &left_type)?
                        } else {
                            rwriter.opaque_v()
                        };
                        event.write_option(rwriter, RETVAL, return_type.name(), Some(val));
                    } else {
                        event.write_option(rwriter, RETVAL, return_type.name(), None);
                    }
                }
                _ => {
                    let type_class = left_type.type_class();
                    if type_class.contains(TypeClass::Enumeration) {
                        log::trace!("{} enumerate (rax)", return_type.name());
                        if let Ok(val) = enumerate_from_rax(&left_type) {
                            event.write_option(rwriter, RETVAL, return_type.name(), Some(val));
                        } else {
                            event.write_option(rwriter, RETVAL, return_type.name(), None);
                        }
                        return Ok(());
                    }
                    // size is known; pass by stack
                    #[cfg(target_arch = "x86_64")]
                    {
                        log::trace!("{} pass by stack (rax)", return_type.name());
                        let sb_value = sb_value_from_rax(return_type)?;
                        event.write_sb_value(rwriter, &sb_value);
                    }
                    #[cfg(target_arch = "aarch64")]
                    {
                        let mut sb_value = Err(WriteErr);
                        let x0 = read_u64(&reg("x0"))?;
                        if x0 == 0 {
                            log::trace!("{} pass by stack (x0) is None", return_type.name());
                            event.write_option(rwriter, RETVAL, return_type.name(), None);
                            return Ok(());
                        }
                        if x0 > 0xffff && sb_value.is_err() {
                            sb_value = sb_value_from_addr(x0, return_type);
                            if sb_value.is_ok() {
                                log::trace!("{} pass by stack (x0)", return_type.name());
                            }
                        }
                        let x8 = read_u64(&reg("x8"))?;
                        if x8 > 0xffff && sb_value.is_err() {
                            sb_value = sb_value_from_addr(x8, return_type);
                            if sb_value.is_ok() {
                                log::trace!("{} pass by stack (x8)", return_type.name());
                            }
                        }
                        let x9 = read_u64(&reg("x9"))?;
                        if x9 > 0xffff && sb_value.is_err() {
                            sb_value = sb_value_from_addr(x9, return_type);
                            if sb_value.is_ok() {
                                log::trace!("{} pass by stack (x9)", return_type.name());
                            }
                        }
                        event.write_sb_value(rwriter, &sb_value?);
                    }
                }
            }
            Ok(())
        }
        ValueType::Result(left, right) => {
            assert_eq!(return_type.number_of_fields(), 2);
            let left_type = return_type
                .field_at_index(0)
                .type_()
                .field_at_index(0)
                .type_();
            let right_type = return_type
                .field_at_index(1)
                .type_()
                .field_at_index(0)
                .type_();
            if !left_type.is_valid() || !right_type.is_valid() {
                event.write_value(rwriter, RETVAL, rwriter.opaque_v().as_bytes());
                return Ok(());
            }
            let left = boildown(rc_into_inner(left)?, left_type.clone())?;
            let right = boildown(rc_into_inner(right)?, right_type.clone())?;
            match (left.size_of(), right.size_of()) {
                (SizeOfType::Sized(left_size), SizeOfType::Sized(right_size)) => {
                    if left_size == 0 && right_size == 0 {
                        // Result<(), ()>
                        log::trace!("{} rax only", return_type.name());
                        let res = read_u64(&rax())?;
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            res == 0,
                            rwriter.unit_v(),
                        );
                        return Ok(());
                    } else if (matches!(left, ValueType::bool) && right_size == 0)
                        || (matches!(right, ValueType::bool) && left_size == 0)
                    {
                        // this is a Rust trick for Result<bool, ()> or Result<(), bool>
                        log::trace!("{} rax only", return_type.name());
                        let res = read_u64(&rax())? & 0b11;
                        let val = match res {
                            0 => rwriter.prim_v("bool", &[0]),
                            1 => rwriter.prim_v("bool", &[1]),
                            2 => rwriter.unit_v(),
                            _ => return Err(WriteErr),
                        };
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            (res == 2) == (left_size == 0),
                            val,
                        );
                        return Ok(());
                    } else if (matches!(left, ValueType::char) && right_size == 0)
                        || (matches!(right, ValueType::char) && left_size == 0)
                    {
                        // this is a Rust trick for Result<char, ()> or Result<(), char>
                        log::trace!("{} rax only", return_type.name());
                        let res = read_u64(&rax())?;
                        let is_char = res <= (char::MAX as u64);
                        let val = if is_char {
                            rwriter.prim_v("char", &res.to_ne_bytes()[..4])
                        } else {
                            rwriter.unit_v()
                        };
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            is_char == (right_size == 0),
                            val,
                        );
                        return Ok(());
                    } else if (matches!(left, ValueType::Slice(_)) && right_size == 0)
                        || (matches!(right, ValueType::Slice(_)) && left_size == 0)
                    {
                        // this is a Rust trick for Result<&[T], ()> or Result<(), &[T]>
                        log::trace!("{} rax, rdx", return_type.name());
                        let rax = read_u64(&rax())?;
                        let val = if rax != 0 {
                            if left_size != 0 {
                                get_slice_from_rax_rdx(rwriter, &left, &left_type)?
                            } else {
                                get_slice_from_rax_rdx(rwriter, &right, &right_type)?
                            }
                        } else {
                            rwriter.unit_v()
                        };
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            (rax == 0) == (left_size == 0),
                            val,
                        );
                        return Ok(());
                    } else if (matches!(left, ValueType::i128 | ValueType::u128) && left == right)
                        || (matches!(left, ValueType::i128 | ValueType::u128) && right_size == 0)
                        || (left_size == 0 && matches!(right, ValueType::i128 | ValueType::u128))
                    // but not Result<i128, u128>
                    {
                        // Result<i128, i128>, Result<u128, u128>, Result<i128, ()>, Result<(), u128>
                        log::trace!("{} rcx, rdx", return_type.name());
                        let (res, v) = {
                            #[cfg(target_arch = "x86_64")]
                            {
                                let rdx_u64 = read_u64(&rdx())?;
                                let rcx_u64 = read_u64(&rcx())?;
                                let res_is_rcx = (rdx_u64 == 0 && rcx_u64 == 1)
                                    || (rdx_u64 == 0 && rcx_u64 == 0);
                                let res = if res_is_rcx { rcx_u64 } else { rdx_u64 };
                                let v = if res_is_rcx {
                                    [reg("rsi"), reg("r8")]
                                } else {
                                    [reg("rdi"), reg("r8")]
                                };
                                (res, &values_to_bytes::<16, _>(v.into_iter(), 2)?)
                            }
                            #[cfg(target_arch = "aarch64")]
                            {
                                let res = read_u64(&rax())?;
                                let v = [reg("x2"), reg("x3")];
                                (res, &values_to_bytes::<16, _>(v.into_iter(), 2)?)
                            }
                        };
                        let val = if left_size == 0 && res == 0 {
                            rwriter.unit_v()
                        } else if right_size == 0 && res == 1 {
                            rwriter.unit_v()
                        } else {
                            let ty = if left.is_integer() {
                                left.primitive_name()
                            } else {
                                right.primitive_name()
                            };
                            rwriter.prim_v(ty, v)
                        };
                        event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        return Ok(());
                    } else if (left_size == 0 && right.is_str())
                        || (left.is_str() && right_size == 0)
                    {
                        // Result<(), &str>, Result<&str, ()>
                        // pointer is non-zero, so discriminant and data is packed together
                        log::trace!("{} rax, rdx", return_type.name());
                        let addr = read_u64(&rax())?;
                        let val = if addr != 0 {
                            let len = read_u64(&rdx())? as usize;
                            let bytes = read_str_from_addr(addr, len)?;
                            rwriter.strlit_v(&bytes)
                        } else {
                            rwriter.unit_v()
                        };
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            (addr == 0) == (left_size == 0),
                            val,
                        );
                        return Ok(());
                    } else if (matches!(left, ValueType::DynRef(_)) && right_size == 0)
                        || (left_size == 0 && matches!(right, ValueType::DynRef(_)))
                    {
                        log::trace!("{} rax, rdx", return_type.name());
                        let addr = read_u64(&rax())?;
                        let val = if addr != 0 {
                            if left_size != 0 {
                                dyn_ref_value_from_rax_rdx(rwriter, &left, &left_type)?
                            } else {
                                dyn_ref_value_from_rax_rdx(rwriter, &right, &right_type)?
                            }
                        } else {
                            rwriter.unit_v()
                        };
                        event.write_result(
                            rwriter,
                            RETVAL,
                            return_type.name(),
                            (addr == 0) == (left_size == 0),
                            val,
                        );
                        return Ok(());
                    } else if cfg!(target_arch = "aarch64")
                        && (left_size <= 8 && right_size <= 8)
                        && (left.is_integer() && right.is_integer())
                        && left.is_signed_integer() != right.is_signed_integer()
                    {
                        // since llvm 18; Result<u32, i32> Result<u64, i64>
                        log::trace!("{} x0, x1", return_type.name());
                        let res = read_u64(&reg("x0"))?;
                        let left_or_right = if res == 0 { &left } else { &right };
                        let val = get_prim_from_rdx(rwriter, left_or_right)?;
                        event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        return Ok(());
                    } else if cfg!(target_arch = "aarch64")
                        && matches!(
                            (&left, &right),
                            (ValueType::u128, ValueType::i128) | (ValueType::i128, ValueType::u128)
                        )
                    {
                        // since llvm 18; Result<u128, i128>
                        log::trace!("{} x0, x2, x3", return_type.name());
                        let res = read_u64(&reg("x0"))?;
                        let left_or_right = if res == 0 { &left } else { &right };
                        let val = rwriter.prim_v(
                            match left_or_right {
                                ValueType::u128 => "u128",
                                ValueType::i128 => "i128",
                                _ => unreachable!(),
                            },
                            &values_to_bytes::<16, _>([reg("x2"), reg("x3")].into_iter(), 2)?,
                        );
                        event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        return Ok(());
                    } else if (left_size <= 8 && right_size <= 8) // must fit in register
                        && (left == right // same type, easy case
                        || (left.is_thin_ptr() && right.is_thin_ptr()) // both are pointers
                        // small integers, complex case
                        || (left.is_integer() && right.is_integer() && left_size <= 4 && right_size <= 4)
                        // if any side is (), result would be passed through registers
                        || (left == ValueType::Unit || right == ValueType::Unit))
                    {
                        // pass through register
                        if (left.is_integer() && right.is_integer())
                            && (left_size <= 4 && right_size <= 4) // must fit together
                            && (
                                left_size != right_size || // size mismatch
                                (left.is_signed_integer() != right.is_signed_integer()) // signed mismatch
                            )
                        {
                            log::trace!("{} rax only", return_type.name());
                            // Result<i32, u32>, Result<i8, u8>, Result<i16, i32>
                            // this is the complex case, where the discriminant and value are both squashed into the same register
                            let data = read_u64(&rax())?;
                            let res = data & 0x1; // select the least significant bit
                            let left_or_right = if res == 0 { &left } else { &right };
                            let left_or_right_size = if res == 0 { left_size } else { right_size };
                            let ty = left_or_right.primitive_name();

                            let val = if data == 0 || data == 1 {
                                let b = read_u64(&rdx())?.to_ne_bytes();
                                rwriter.prim_v(ty, &b[..left_or_right_size])
                            } else {
                                let b = data.to_ne_bytes();
                                match left_or_right_size {
                                    // the value is in the 2nd lower word
                                    1 => rwriter.prim_v(ty, &b[1..2]),
                                    2 => rwriter.prim_v(ty, &b[2..4]),
                                    4 => rwriter.prim_v(ty, &b[4..8]),
                                    _ => unreachable!(),
                                }
                            };
                            event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        } else if (left == ValueType::Unit || right == ValueType::Unit)
                            && (left.is_thin_ptr() || right.is_thin_ptr())
                        {
                            // Result<(), &T>, Result<&T, ()>
                            // ptr is always non-zero, so the whole struct fits in 64 bits
                            log::trace!("{} rax only", return_type.name());
                            let data = read_u64(&rax())?;
                            let res = (data == 0) == (left_size == 0);
                            let data = create_data_from_u64(data)?;
                            let sb_value =
                                sb_target.create_value_from_data(RETVAL, &data, return_type);
                            let val =
                                write_union_with(rwriter, &sb_value, if res { 0 } else { 1 })?;
                            event.write_value(rwriter, RETVAL, val.as_bytes());
                        } else {
                            log::trace!("{} rax, rdx", return_type.name());
                            // Result<i32, i32>, Result<i8, i8>
                            // this is the easy case, where discriminant is in rax , data in rdx
                            let res = read_u64(&rax())? & 0x1; // select the least significant bit
                            let left_or_right = if res == 0 { &left } else { &right };
                            let left_or_right_type =
                                if res == 0 { &left_type } else { &right_type };
                            let val = if left_or_right.is_primitive() {
                                // here we only use the boiled down type
                                // may be we can recreate the full type using create_value_from_data()?
                                get_prim_from_rdx(rwriter, left_or_right)?
                            } else if left_or_right.is_thin_ptr() {
                                let addr = read_u64(&rdx())?;
                                ref_value_from_addr(rwriter, addr, left_or_right_type)?
                            } else {
                                rwriter.opaque_v()
                            };
                            event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        }
                        return Ok(());
                    } else if cfg!(target_arch = "aarch64")
                        && (left_size <= 8 && right_size <= 8)
                        && (left.is_thin_ptr() || right.is_thin_ptr())
                    {
                        log::trace!("{} x8, x9", return_type.name());
                        let res = read_u64(&reg("x8"))?;
                        let data = read_u64(&reg("x9"))?;
                        let left_or_right_type = if res == 0 { &left_type } else { &right_type };
                        let data = create_data_from_u64(data)?;
                        let value =
                            sb_target.create_value_from_data(RETVAL, &data, &left_or_right_type);
                        if !value.is_valid() {
                            return Err(WriteErr);
                        }
                        let mut val = Bytes::new();
                        val.write_inner_value(rwriter, &value);
                        event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                        return Ok(());
                    }
                }
                _ => (),
            }
            // Result<i32, i64>, Result<u64, i64>
            #[cfg(target_arch = "x86_64")]
            {
                let data = read_u64(&rax())?;
                if data == 0 || data == 1 {
                    let res = data & 0x1; // select the least significant bit
                    let left_or_right = if res == 0 { &left } else { &right };
                    let left_or_right_size = match (left.size_of(), right.size_of()) {
                        (SizeOfType::Sized(left_size), SizeOfType::Sized(right_size)) => {
                            if res == 0 {
                                left_size
                            } else {
                                right_size
                            }
                        }
                        _ => unreachable!(),
                    };
                    let ty = left_or_right.primitive_name();
                    let b = read_u64(&rdx())?.to_ne_bytes();
                    let val = rwriter.prim_v(ty, &b[..left_or_right_size]);
                    event.write_result(rwriter, RETVAL, return_type.name(), res == 0, val);
                } else {
                    log::trace!("{} pass by stack (rax)", return_type.name());
                    let sb_value = sb_value_from_rax(return_type)?;
                    event.write_sb_value(rwriter, &sb_value);
                }
            }
            #[cfg(target_arch = "aarch64")]
            {
                // here we assume the layout on stack where the first byte is the discriminant
                let mut res = 0;
                let mut sb_value = Err(WriteErr);
                let x8 = read_u64(&reg("x8"))?;
                if x8 > 0xffff && sb_value.is_err() {
                    res = read_byte_from_addr(x8)?;
                    sb_value = sb_value_from_addr(x8, return_type);
                    if sb_value.is_ok() {
                        log::trace!("{} pass by stack (x8)", return_type.name());
                    }
                }
                let x9 = read_u64(&reg("x9"))?;
                if x9 > 0xffff && sb_value.is_err() {
                    res = read_byte_from_addr(x9)?;
                    sb_value = sb_value_from_addr(x9, return_type);
                    if sb_value.is_ok() {
                        log::trace!("{} pass by stack (x9)", return_type.name());
                    }
                }
                let x0 = read_u64(&reg("x0"))?;
                if x0 > 0xffff && sb_value.is_err() {
                    res = read_byte_from_addr(x0)?;
                    sb_value = sb_value_from_addr(x0, return_type);
                    if sb_value.is_ok() {
                        log::trace!("{} pass by stack (x0)", return_type.name());
                    }
                }
                let val = write_union_with(rwriter, &sb_value?, res as u32)?;
                event.write_value(rwriter, RETVAL, val.as_bytes());
            }
            Ok(())
        }
        ValueType::Array(_, _) => write_array(rwriter, event, &value_type, &return_type),
        ValueType::Slice(_) => {
            let value = get_slice_from_rax_rdx(rwriter, &value_type, &return_type)?;
            event.write_value(rwriter, RETVAL, value.as_bytes());
            Ok(())
        }
        ValueType::Other(_) => {
            if get_union_type(return_type).is_some() {
                // TODO handle Rust union (enum with fields)
                event.write_value(rwriter, RETVAL, rwriter.opaque_v().as_bytes());
                return Ok(());
            }
            let current_type = peeloff(return_type.clone());
            let number_of_fields = current_type.number_of_fields();
            let type_class = current_type.type_class();
            if type_class.contains(TypeClass::Enumeration) {
                log::trace!("{} rax", return_type.name());
                if let Ok(val) = enumerate_from_rax(&current_type) {
                    event.write_value(rwriter, RETVAL, val.as_bytes());
                } else {
                    event.write_value(rwriter, RETVAL, rwriter.opaque_v().as_bytes());
                }
                return Ok(());
            } else if number_of_fields == 0 && type_class.contains(TypeClass::Struct) {
                event.write_value(rwriter, RETVAL, rwriter.unit_v().as_bytes());
                return Ok(());
            } else if number_of_fields == 1 {
                let (left_name, left_type, left_sb_type) = {
                    let left = current_type.field_at_index(0);
                    let left_sb_type = left.type_();
                    let left_type = boildown_from(left_sb_type.clone())?;
                    (left.name().to_owned(), left_type, left_sb_type)
                };
                if matches!(left_type, ValueType::Array(_, _)) {
                    return write_array(rwriter, event, &left_type, &return_type);
                }
                match left_type.size_of() {
                    SizeOfType::Sized(left_size) if left_size <= 16 => {
                        log::trace!("{} rax, rdx", return_type.name());
                        let value = if left_type.is_primitive() {
                            get_prim_from_rax_rdx(rwriter, &left_type)?
                        } else if matches!(left_type, ValueType::Reference(_)) {
                            get_reference_from_rax_rdx(rwriter, &left_type, &left_sb_type)?
                        } else if left_type.is_fat_ptr() {
                            dyn_ref_value_from_rax_rdx(rwriter, &value_type, &return_type)?
                        } else {
                            rwriter.opaque_v()
                        };
                        let value =
                            rwriter.struct_v(return_type.name(), [(left_name, value)].into_iter());
                        event.write_value(rwriter, RETVAL, value.as_bytes());
                        return Ok(());
                    }
                    _ => (),
                }
            } else if number_of_fields == 2 {
                let (left_name, left_type, left_sb_type) = {
                    let left = current_type.field_at_index(0);
                    let left_sb_type = left.type_();
                    let left_type = boildown_from(left_sb_type.clone())?;
                    (left.name().to_owned(), left_type, left_sb_type)
                };
                let (right_name, right_type, right_sb_type) = {
                    let right = current_type.field_at_index(1);
                    let right_sb_type = right.type_();
                    let right_type = boildown_from(right_sb_type.clone())?;
                    (right.name().to_owned(), right_type, right_sb_type)
                };
                match (left_type.size_of(), right_type.size_of()) {
                    (SizeOfType::Sized(left_size), SizeOfType::Sized(right_size))
                        if left_size <= 8 && right_size <= 8 =>
                    {
                        if left_type.is_float() && right_type.is_float() && left_type != right_type
                        {
                            let (left_value, right_value) = if left_size >= right_size {
                                (
                                    get_float_from_xmm0(rwriter, &left_type)?,
                                    get_float_from_xmm1(rwriter, &right_type)?,
                                )
                            } else {
                                (
                                    get_float_from_xmm1(rwriter, &left_type)?,
                                    get_float_from_xmm0(rwriter, &right_type)?,
                                )
                            };

                            let value = rwriter.struct_v(
                                return_type.name(),
                                [(left_name, left_value), (right_name, right_value)].into_iter(),
                            );
                            event.write_value(rwriter, RETVAL, value.as_bytes());
                            return Ok(());
                        } else {
                            log::trace!("{} rax, rdx", return_type.name());
                            let left_value = if left_type.is_primitive() {
                                get_prim_from_rax_rdx(rwriter, &left_type)?
                            } else if matches!(left_type, ValueType::Reference(_)) {
                                get_reference_from_rax_rdx(rwriter, &left_type, &left_sb_type)?
                            } else {
                                rwriter.opaque_v()
                            };

                            let right_value = if right_type.is_primitive() {
                                if right_type.is_float() {
                                    if left_type.is_float() {
                                        get_float_from_xmm1(rwriter, &right_type)?
                                    } else {
                                        get_float_from_xmm0(rwriter, &right_type)?
                                    }
                                } else {
                                    get_prim_from_rdx(rwriter, &right_type)?
                                }
                            } else if matches!(right_type, ValueType::Reference(_)) {
                                get_reference_from_rdx(rwriter, &right_type, &right_sb_type)?
                            } else {
                                rwriter.opaque_v()
                            };

                            let value = rwriter.struct_v(
                                return_type.name(),
                                [(left_name, left_value), (right_name, right_value)].into_iter(),
                            );
                            event.write_value(rwriter, RETVAL, value.as_bytes());
                            return Ok(());
                        }
                    }
                    _ => (),
                }
            }

            event.write_sb_value(rwriter, &final_pass_by_stack(return_type)?);
            Ok(())
        }
    }
}

fn pointer_typename(name: &str) -> &'static str {
    if name.starts_with("&") {
        "ref"
    } else if name.starts_with("alloc::boxed::Box<") {
        "Box"
    } else if name.starts_with("alloc::rc::Rc<") {
        "Rc"
    } else if name.starts_with("alloc::sync::Arc<") {
        "Arc"
    } else {
        "ptr"
    }
}

/// Remove all type wrappers.
///
/// A(B { i: C }) -> C; where C is { x: i32 }
fn peeloff(sb_type: SBType) -> SBType {
    if sb_type.number_of_fields() == 1 {
        let inner = sb_type.field_at_index(0).type_();
        if !matches!(inner.name(), "&str") && inner.type_class().contains(TypeClass::Struct) {
            peeloff(inner)
        } else {
            sb_type
        }
    } else {
        sb_type
    }
}

fn boildown_from(sb_type: SBType) -> Result<ValueType, WriteErr> {
    let value_type = sb_type.name().parse().map_err(|_| WriteErr)?;
    boildown(value_type, sb_type)
}

fn rc_into_inner<T>(rc: Rc<T>) -> Result<T, WriteErr> {
    Rc::into_inner(rc).ok_or(WriteErr)
}
