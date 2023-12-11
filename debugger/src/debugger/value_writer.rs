use crate::{write_value, Addr, Bytes, RVal, UnionType, Val, WriteErr, RECURSIVE_DEREF_LIMIT};
use firedbg_protocol::IndexMap;
use lldb::SBValue;
use rustc_hash::FxHashMap;
use std::fmt::Display;

#[derive(Debug)]
/// Rust Value Writer
pub struct RValueWriter<'a> {
    writer: ValueWriter<'a>,
}

#[derive(Debug)]
/// Base Value Writer
pub struct ValueWriter<'a> {
    /// Memory address -> Value; cleared per breakpoint.
    /// Each event has its isolated env.
    /// Because address on the stack is only valid over the duration of the frame.
    /// This map has an order. This is very tricky; we have to sort topologically.
    /// The deepest ref is always written first.
    env: IndexMap<Addr, Option<Bytes>>,
    allocation: &'a FxHashMap<u64, String>,
}

/// Token level methods
impl Bytes {
    /// Emit a Str Token
    pub fn identifier(&mut self, s: &str) {
        if s.contains('\"') {
            panic!("An identifier should not contain `\"`");
        }
        self.push_str("\" ");
        self.push_slice(s.as_bytes());
        self.push_str("\"");
    }

    /// Emit a Int Token
    pub fn integer<I: Integer>(&mut self, i: I) {
        self.push_string(i.to_string());
        self.space();
    }

    /// Emit a Bytes Token
    pub fn blob(&mut self, bytes: Bytes) {
        self.push_str("# ");
        self.push_slice(&(bytes.len() as u32).to_ne_bytes());
        self.push_bytes(bytes);
    }

    /// Emit a Bytes Token
    pub fn blob_slice(&mut self, bytes: &[u8]) {
        self.push_str("# ");
        self.push_slice(&(bytes.len() as u32).to_ne_bytes());
        self.push_slice(bytes);
    }

    /// Emit a token separator
    pub fn space(&mut self) {
        self.push_byte(b' ');
    }
}

/// A union of `usize` | `u64` | `u32`
pub trait Integer: Display {}
impl<T: Integer> Integer for &T {}
impl Integer for usize {}
impl Integer for u64 {}
impl Integer for u32 {}

trait ValueWriterT {
    fn p_alloc_env(&mut self, r: Addr) -> bool;
    fn p_set_env(&mut self, r: Addr, v: Bytes);
    fn emit_env(&mut self) -> Bytes;
}

impl<S: ValueWriterT> Val<ValueWriter<'_>> for S {
    type E = Bytes;

    fn alloc_env(&mut self, r: Addr) -> bool {
        self.p_alloc_env(r)
    }

    fn set_env(&mut self, r: Addr, v: Bytes) {
        self.p_set_env(r, v);
    }

    fn prim_v(&self, ty: &str, val: &[u8]) -> Bytes {
        let mut msg = Bytes::new();
        msg.identifier(ty);
        msg.blob_slice(val);
        msg.space();
        msg.push_str("prim");
        msg
    }

    fn bytes_v(&self, ty: &str, val: Bytes) -> Bytes {
        let mut msg = Bytes::new();
        msg.identifier(ty);
        msg.blob(val);
        msg.push_str("bytes");
        msg
    }

    fn arr_v<I: Iterator<Item = Bytes>>(&self, iter: I) -> Bytes {
        let mut msg = Bytes::new();
        let mut len: usize = 0;
        for x in iter {
            msg.push_bytes(x);
            msg.space();
            len += 1;
        }
        msg.integer(len);
        msg.push_str("arr");
        msg
    }

    fn ref_v(&self, ty: &str, r: Addr) -> Bytes {
        let mut msg = Bytes::new();
        msg.identifier(ty);
        msg.blob(Bytes::from(r.to_bytes()));
        msg.push_str("ref");
        msg
    }

    fn struct_v<I: Iterator<Item = (String, Bytes)>>(&self, ty: &str, fields: I) -> Bytes {
        let mut msg = Bytes::new();
        let mut len: usize = 0;
        for (n, v) in fields {
            msg.identifier(&n);
            msg.push_bytes(v);
            msg.space();
            len += 1;
        }
        msg.integer(len);
        msg.identifier(&trim_type_name(ty));
        msg.push_str("struct");
        msg
    }

    fn enumerate_v(&self, ty: &str, variant: &str) -> Bytes {
        let mut msg = Bytes::new();
        msg.identifier(ty);
        msg.identifier(variant);
        msg.push_str("enum");
        msg
    }

    fn unit_v(&self) -> Bytes {
        let mut msg = Bytes::new();
        msg.push_str("unit");
        msg
    }

    fn opaque_v(&self) -> Bytes {
        let mut msg = Bytes::new();
        msg.push_str("opaque");
        msg
    }
}

impl ValueWriterT for ValueWriter<'_> {
    fn p_alloc_env(&mut self, addr: Addr) -> bool {
        if self.env.get(&addr).is_some() {
            // this ensures the deepest ref is always written first
            let val = self.env.shift_remove(&addr).expect("addr exists");
            self.env.insert(addr, val);
            false
        } else {
            self.env.insert(addr, None);
            true
        }
    }

    fn p_set_env(&mut self, addr: Addr, val: Bytes) {
        *self.env.get_mut(&addr).expect("addr exists") = Some(val);
    }

    fn emit_env(&mut self) -> Bytes {
        let mut msg = Bytes::new();
        for (addr, val) in self.env.iter_mut().rev() {
            // reverse!
            if let Some(val) = val.take() {
                msg.blob(Bytes::from(addr.to_bytes()));
                msg.push_bytes(val);
                msg.space();
                msg.push_str("setenv");
                msg.space();
            }
        }
        msg
    }
}

impl<'a> RValueWriter<'a> {
    pub fn new(allocation: &'a FxHashMap<u64, String>) -> Self {
        Self {
            writer: ValueWriter {
                env: Default::default(),
                allocation,
            },
        }
    }

    pub fn write_value(&mut self, val: &SBValue) -> Result<Bytes, WriteErr> {
        write_value(self, val, *RECURSIVE_DEREF_LIMIT)
    }

    pub fn emit_env(&mut self) -> Bytes {
        <Self as ValueWriterT>::emit_env(self)
    }

    /// Returns what was allocated at this memory address
    pub fn allocated_at(&self, addr: u64) -> Option<&str> {
        self.writer.allocation.get(&addr).map(|s| s.as_str())
    }
}

impl ValueWriterT for RValueWriter<'_> {
    fn p_alloc_env(&mut self, r: Addr) -> bool {
        self.writer.p_alloc_env(r)
    }
    fn p_set_env(&mut self, r: Addr, v: Bytes) {
        self.writer.p_set_env(r, v)
    }
    fn emit_env(&mut self) -> Bytes {
        self.writer.emit_env()
    }
}

impl RVal<ValueWriter<'_>> for RValueWriter<'_> {
    fn strlit_v(&self, val: &[u8]) -> Bytes {
        let mut msg = Bytes::new();
        msg.blob_slice(val);
        msg.push_str("strlit");
        msg
    }

    fn union_v<I: Iterator<Item = (String, Bytes)>>(
        &mut self,
        ty: &UnionType,
        index: usize,
        fields: I,
    ) -> Bytes {
        let mut msg = Bytes::new();
        let mut len: usize = 0;
        for (n, v) in fields {
            msg.identifier(&n);
            msg.push_bytes(v);
            msg.space();
            len += 1;
        }
        msg.integer(len);
        msg.integer(index);
        msg.identifier(&ty.name);
        len = 0;
        for v in &ty.variants {
            msg.identifier(v);
            len += 1;
        }
        msg.integer(len);
        msg.push_str("union_decl");
        msg
    }

    fn vector_v<I: Iterator<Item = Bytes>>(&self, iter: I) -> Bytes {
        let mut msg = Bytes::new();
        let mut len: usize = 0;
        for x in iter {
            msg.push_bytes(x);
            msg.space();
            len += 1;
        }
        msg.integer(len);
        msg.push_str("vec");
        msg
    }

    fn slice_v<I: Iterator<Item = Bytes>>(&self, iter: I) -> Bytes {
        let mut msg = Bytes::new();
        let mut len: usize = 0;
        for x in iter {
            msg.push_bytes(x);
            msg.space();
            len += 1;
        }
        msg.integer(len);
        msg.push_str("slice");
        msg
    }
}

pub(crate) fn trim_type_name(ty: &str) -> String {
    ty.replace(", alloc::alloc::Global>", ">")
}
