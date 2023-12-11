use crate::{
    Addr, ArrayType, Bytes, PValue, RVal, RValue, RValueLift, RefAddr, StringType, UnionType, Val,
};
use firedbg_protocol::IndexMap;
use std::collections::HashMap;

#[derive(Debug)]
/// Stream Reader
pub struct Reader {
    source: SourceReader,
    context: ReaderContext,
}

#[derive(Debug)]
struct SourceReader {
    source: Bytes,
    cur: usize,
}

#[derive(Debug)]
struct ReaderContext {
    env: HashMap<Addr, RValue>,
}

#[derive(Debug)]
enum Token {
    Str(String),
    Bytes(Bytes),
    Int(u64),
    Op(String),
}

impl Reader {
    pub fn new() -> Self {
        Self {
            source: SourceReader::new(),
            context: ReaderContext::new(),
        }
    }

    pub fn read_values(&mut self) -> Vec<(String, RValue)> {
        self.source.read_values(&mut self.context)
    }

    pub fn read_string(&mut self) -> Option<String> {
        match self.source.next_token() {
            Some(Token::Str(string)) => Some(string),
            Some(other) => panic!("Expected String, got {other:?}"),
            None => None,
        }
    }

    pub fn read_int(&mut self) -> Option<u64> {
        match self.source.next_token() {
            Some(Token::Int(i)) => Some(i),
            Some(other) => panic!("Expected Integer, got {other:?}"),
            None => None,
        }
    }

    pub fn set_source(&mut self, source: Bytes, offset: usize) {
        self.source.set_source(source, offset)
    }
}

impl ReaderContext {
    pub fn new() -> Self {
        Self {
            env: Default::default(),
        }
    }
}

/// Reader and Writer uses the same interface. How brilliant is that?
impl Val<ReaderContext> for ReaderContext {
    type E = RValue;

    fn alloc_env(&mut self, _: Addr) -> bool {
        panic!("Should not be called by Reader");
    }

    fn set_env(&mut self, addr: Addr, val: RValue) {
        self.env.insert(addr, val);
    }

    fn prim_v(&self, ty: &str, b: &[u8]) -> RValue {
        RValue::Prim(match ty {
            "bool" => PValue::bool(b[0] != 0),
            "char" => PValue::char(
                match char::from_u32(u32::from_ne_bytes(b[..].try_into().unwrap())) {
                    Some(c) => c,
                    None => return RValue::Opaque,
                },
            ),
            "u8" => PValue::u8(u8::from_ne_bytes(b[..].try_into().unwrap())),
            "i8" => PValue::i8(i8::from_ne_bytes(b[..].try_into().unwrap())),
            "u16" => PValue::u16(u16::from_ne_bytes(b[..].try_into().unwrap())),
            "i16" => PValue::i16(i16::from_ne_bytes(b[..].try_into().unwrap())),
            "u32" => PValue::u32(u32::from_ne_bytes(b[..].try_into().unwrap())),
            "i32" => PValue::i32(i32::from_ne_bytes(b[..].try_into().unwrap())),
            "u64" => PValue::u64(u64::from_ne_bytes(b[..].try_into().unwrap())),
            "i64" => PValue::i64(i64::from_ne_bytes(b[..].try_into().unwrap())),
            "usize" => PValue::usize(match b.len() {
                4 => u32::from_ne_bytes(b[..].try_into().unwrap()) as u64,
                8 => u64::from_ne_bytes(b[..].try_into().unwrap()),
                _ => panic!("Not a usize: {b:?}"),
            }),
            "isize" => PValue::isize(match b.len() {
                4 => i32::from_ne_bytes(b[..].try_into().unwrap()) as i64,
                8 => i64::from_ne_bytes(b[..].try_into().unwrap()),
                _ => panic!("Not a isize: {b:?}"),
            }),
            "u128" => PValue::u128(u128::from_ne_bytes(b[..].try_into().unwrap())),
            "i128" => PValue::i128(i128::from_ne_bytes(b[..].try_into().unwrap())),
            "f32" => PValue::f32(f32::from_ne_bytes(b[..].try_into().unwrap())),
            "f64" => PValue::f64(f64::from_ne_bytes(b[..].try_into().unwrap())),
            _ => panic!("unknown ty `{ty}`"),
        })
    }

    fn bytes_v(&self, ty: &str, val: Bytes) -> RValue {
        RValue::Bytes {
            typename: ty.into(),
            value: val.into_bytes(),
        }
    }

    fn arr_v<I: Iterator<Item = RValue>>(&self, iter: I) -> RValue {
        RValue::Array {
            typename: ArrayType::Arr,
            data: iter.collect(),
        }
    }

    fn ref_v(&self, ty: &str, addr: Addr) -> RValue {
        if let Some(val) = self.env.get(&addr) {
            RValue::Ref {
                typename: ty.parse().unwrap(),
                addr: RefAddr::Addr(addr),
                value: Box::new(val.clone()),
            }
        } else {
            RValue::UnresolvedRef {
                addr: RefAddr::Addr(addr),
            }
        }
    }

    fn struct_v<I: Iterator<Item = (String, RValue)>>(&self, _: &str, _: I) -> RValue {
        panic!("Please pass ownership");
    }

    fn enumerate_v(&self, _: &str, _: &str) -> RValue {
        panic!("Please pass ownership");
    }

    fn unit_v(&self) -> RValue {
        RValue::Unit
    }

    fn opaque_v(&self) -> RValue {
        RValue::Opaque
    }
}

impl ReaderContext {
    fn strlit_v(&self, v: Vec<u8>) -> Option<RValue> {
        Some(RValue::String {
            typename: StringType::StrLit,
            value: string_from_utf8(v)?,
        })
    }

    fn enumerate_v(&self, ty: String, variant: String) -> RValue {
        RValue::Enum {
            typename: ty,
            variant: variant,
        }
    }

    fn struct_v<I: Iterator<Item = (String, RValue)>>(&self, ty: String, fields: I) -> RValue {
        let mut f = IndexMap::new();
        fields.for_each(|p| {
            f.insert(p.0, p.1);
        });
        if ty.as_str() == "()" && f.is_empty() {
            return RValue::Unit;
        }
        if ty.as_str() == "alloc::string::String" && f.len() == 1 && f.contains_key("vec") {
            return RValue::String {
                typename: StringType::String,
                value: match match f.into_values().next() {
                    Some(RValue::Opaque) => None,
                    Some(RValue::Bytes { value, .. }) => string_from_utf8(value),
                    _ => None,
                } {
                    Some(value) => value,
                    None => return RValue::Opaque,
                },
            };
        }
        RValue::Struct {
            typename: ty,
            fields: f,
        }
    }
}

impl RVal<ReaderContext> for ReaderContext {
    fn strlit_v(&self, _: &[u8]) -> RValue {
        panic!("Please pass ownership");
    }

    fn union_v<I: Iterator<Item = (String, RValue)>>(
        &mut self,
        t: &UnionType,
        index: usize,
        fields: I,
    ) -> RValue {
        let mut f = IndexMap::new();
        fields.for_each(|p| {
            f.insert(p.0, p.1);
        });
        RValue::Union {
            typeinfo: t.clone(),
            variant: t.variants[index].clone(),
            fields: f,
        }
    }

    fn vector_v<I: Iterator<Item = RValue>>(&self, iter: I) -> RValue {
        RValue::Array {
            typename: ArrayType::Vec,
            data: iter.collect(),
        }
    }

    fn slice_v<I: Iterator<Item = RValue>>(&self, iter: I) -> RValue {
        RValue::Array {
            typename: ArrayType::Slice,
            data: iter.collect(),
        }
    }
}

impl SourceReader {
    pub fn new() -> SourceReader {
        SourceReader {
            source: Bytes::new(),
            cur: 0,
        }
    }

    pub fn set_source(&mut self, source: Bytes, offset: usize) {
        self.source = source;
        self.cur = offset;
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.cur >= self.source.len() {
            return None;
        }
        let p = self.cur;
        let q = self.next_char(b' ');
        let tok = self.source.slice(p, q);
        if matches!(tok, &[b'"']) {
            self.cur += 2;
            let r = self.next_char(b'"');
            let bytes = self.source.slice(self.cur, r).to_owned();
            self.cur = r + 1;
            let tok = String::from_utf8(bytes).unwrap();
            return Some(Token::Str(tok));
        }
        if matches!(tok, &[b'#']) {
            self.cur += 2;
            let len = u32::from_ne_bytes([
                self.source.get(self.cur),
                self.source.get(self.cur + 1),
                self.source.get(self.cur + 2),
                self.source.get(self.cur + 3),
            ]);
            self.cur += 4;
            let bytes = self
                .source
                .slice(self.cur, self.cur + len as usize)
                .to_owned();
            self.cur += len as usize;
            return Some(Token::Bytes(Bytes::from(bytes)));
        }
        self.cur = q + 1; // assume one space as delimiter
        let tok = std::str::from_utf8(tok).unwrap();
        return Some(match tok.parse() {
            Ok(n) => Token::Int(n),
            Err(_) => Token::Op(tok.to_owned()),
        });
    }

    fn next_char(&self, c: u8) -> usize {
        let mut q = self.cur;
        while q < self.source.len() - 1 {
            if self.source.get(q) == c {
                break;
            }
            q += 1;
        }
        q
    }

    /// First, tokenize the binary stream.
    /// Then, push some primitives onto the value stack.
    /// When we read an `op` token, pop off some values from the stack based on the `op`.
    pub fn read_values(&mut self, ctx: &mut ReaderContext) -> Vec<(String, RValue)> {
        let mut str_stack = Vec::<String>::new();
        let mut byte_stack = Vec::<Bytes>::new();
        let mut int_stack = Vec::<u64>::new();
        let mut val_stack = Vec::<RValue>::new();
        let mut names = Vec::<String>::new();

        while let Some(tok) = self.next_token() {
            match tok {
                Token::Str(s) => str_stack.push(s),
                Token::Bytes(s) => byte_stack.push(s),
                Token::Int(x) => int_stack.push(x),
                Token::Op(op) => {
                    if op == "setenv" {
                        let addr = Addr::new(byte_stack.pop().unwrap().as_bytes());
                        let val = val_stack.pop().unwrap();
                        ctx.set_env(addr, val);
                    } else if op == "prim" {
                        let val = byte_stack.pop().unwrap();
                        let ty = str_stack.pop().unwrap();
                        val_stack.push(ctx.prim_v(&ty, val.as_bytes()));
                    } else if op == "bytes" {
                        let val = byte_stack.pop().unwrap();
                        let ty = str_stack.pop().unwrap();
                        val_stack.push(ctx.bytes_v(&ty, val));
                    } else if op == "arr" {
                        let size = int_stack.pop().unwrap();
                        let mut elem = Vec::new();
                        for _ in 0..size {
                            elem.push(val_stack.pop().unwrap());
                        }
                        val_stack.push(ctx.arr_v(elem.into_iter().rev()));
                    } else if op == "ref" {
                        let addr = Addr::new(byte_stack.pop().unwrap().as_bytes());
                        let ty = str_stack.pop().unwrap();
                        val_stack.push(ctx.ref_v(&ty, addr));
                    } else if op == "struct" {
                        let name = str_stack.pop().unwrap();
                        let fc = int_stack.pop().unwrap();
                        let mut field = Vec::new();
                        for _ in 0..fc {
                            let val = val_stack.pop().unwrap();
                            let n = str_stack.pop().unwrap();
                            field.push((n, val));
                        }
                        val_stack.push(ctx.struct_v(name, field.into_iter().rev()));
                    } else if op == "enum" {
                        let variant = str_stack.pop().unwrap();
                        let name = str_stack.pop().unwrap();
                        val_stack.push(ctx.enumerate_v(name, variant));
                    } else if op == "unit" {
                        val_stack.push(ctx.unit_v());
                    } else if op == "opaque" {
                        val_stack.push(ctx.opaque_v());
                    } else if op == "strlit" {
                        let val = byte_stack.pop().unwrap();
                        val_stack.push(
                            ctx.strlit_v(val.into_bytes())
                                .unwrap_or_else(|| ctx.opaque_v()),
                        );
                    } else if op == "union_decl" {
                        let vc = int_stack.pop().unwrap();
                        let mut variants = Vec::new();
                        for _ in 0..vc {
                            variants.push(str_stack.pop().unwrap());
                        }
                        variants.reverse();
                        let name = str_stack.pop().unwrap();
                        let ty = UnionType {
                            name: name,
                            variants,
                        };
                        let index = int_stack.pop().unwrap() as usize;
                        let fc = int_stack.pop().unwrap();
                        let mut field = Vec::new();
                        for _ in 0..fc {
                            let val = val_stack.pop().unwrap();
                            let n = str_stack.pop().unwrap();
                            field.push((n, val));
                        }
                        val_stack.push(ctx.union_v(&ty, index, field.into_iter().rev()));
                    } else if op == "vec" {
                        let size = int_stack.pop().unwrap();
                        let mut elem = Vec::new();
                        for _ in 0..size {
                            elem.push(val_stack.pop().unwrap());
                        }
                        val_stack.push(ctx.vector_v(elem.into_iter().rev()));
                    } else if op == "slice" {
                        let size = int_stack.pop().unwrap();
                        let mut elem = Vec::new();
                        for _ in 0..size {
                            elem.push(val_stack.pop().unwrap());
                        }
                        val_stack.push(ctx.slice_v(elem.into_iter().rev()));
                    } else if op == "name" {
                        names.push(str_stack.pop().unwrap());
                    }
                }
            }
        }

        val_stack.iter_mut().for_each(|v| {
            v.lift();
        });
        assert_eq!(names.len(), val_stack.len());
        names.into_iter().zip(val_stack.into_iter()).collect()
    }
}

fn string_from_utf8(mut bytes: Vec<u8>) -> Option<String> {
    // we'd truncate the raw bytes when sending
    // so the last char may be incomplete
    for _ in 0..4 {
        match String::from_utf8(bytes) {
            Ok(s) => return Some(s),
            Err(e) => {
                bytes = e.into_bytes();
                bytes.pop();
            }
        }
    }
    None
}
