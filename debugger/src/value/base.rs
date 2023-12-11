//! Our brilliant intern is a LISP nerd. He insisted in postfix format.
use crate::{Addr, Bytes, UnionType};

/*
Format
<val> :== <word>*
<word> :== " string" | usize | <op>
<op> :==
  prim, (string string -- val)
  arr, (val[n] n:usize -- val)
  ref, (string k:usize -- val)
  struct, ((string val)[n] n:usize string -- val)
  enum, (string string -- val)
  alloc, (string)
  set, (k:usize val)
  slice, (val[n] n:usize -- val)
  vector, (val[n] n:usize -- val)
  union, declare type, ((string val)[m] m:usize k:usize string string[n] n:usize -- val)
  union, reference to declared type, ((string val)[m] m:usize k:usize r:usize, -- val)
  strlit, (string -- val)
*/

/// Represent value using algebra, subject to change: e.g. Enum, Variable { name, type, value }, etc.
/// Should have cases for &str, enum, etc.
pub trait Val<A> {
    type E;

    /// Allocate a memory address. Return true if this address is the first time being allocated.
    fn alloc_env(&mut self, r: Addr) -> bool;
    /// Set the Value at this memory address.
    fn set_env(&mut self, r: Addr, v: Self::E);

    fn prim_v(&self, ty: &str, val: &[u8]) -> Self::E;
    fn bytes_v(&self, ty: &str, val: Bytes) -> Self::E;
    fn arr_v<I: Iterator<Item = Self::E>>(&self, iter: I) -> Self::E;

    /// The Addr must have been allocated already.
    fn ref_v(&self, ty: &str, r: Addr) -> Self::E;
    fn struct_v<I: Iterator<Item = (String, Self::E)>>(&self, ty: &str, fields: I) -> Self::E;
    /// C style enum, no field
    fn enumerate_v(&self, ty: &str, variant: &str) -> Self::E;
    fn unit_v(&self) -> Self::E;
    /// what we couldn't inspect
    fn opaque_v(&self) -> Self::E;
}

/// Rust extension to `Val`.
pub trait RVal<A>: Val<A> {
    fn strlit_v(&self, val: &[u8]) -> Self::E;
    /// rust complex enum with fields
    fn union_v<I: Iterator<Item = (String, Self::E)>>(
        &mut self,
        t: &UnionType,
        index: usize,
        fields: I,
    ) -> Self::E;
    fn vector_v<I: Iterator<Item = Self::E>>(&self, iter: I) -> Self::E;
    fn slice_v<I: Iterator<Item = Self::E>>(&self, iter: I) -> Self::E;
}
