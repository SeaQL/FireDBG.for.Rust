pub struct StructA;

impl StructA {
    async fn impl_func_a<T: Into<u64>>(i: T) -> T {
        i
    }
}

pub struct StructB {
    field_1: i32,
    pub field_2: usize,
    pub(crate) field_3: u64,
}

impl StructB {
    pub fn impl_func_b1<T>(&mut self, i: T) -> impl std::future::Future<Output = T> where T: Into<i32> { // FIXME: Should this be parsed as async function?
        async { i }
    }

    pub(crate) fn impl_func_b2() -> usize {
        24
    }
}

pub struct StructC(i32, pub usize, pub(crate) u64);

impl StructC {
    pub fn impl_func_c() -> Self {
        Self(0, 1, 2)
    }
}

mod module_a {
    use super::*;

    pub struct StructD;

    impl StructD {
        pub fn impl_func_d(self: std::pin::Pin<&mut Self>) -> Self {
            Self
        }
    }

    mod module_a_a {
        pub struct StructE;

        impl StructE {
            pub fn impl_func_e() -> Self {
                Self
            }
        }
    }

    pub struct StructF;

    impl StructF {
        pub fn impl_func_f() -> Self {
            Self
        }

        pub fn impl_func_f_empty() {

        }
    }
}

use firedbg_protocol::source::*;
use firedbg_rust_parser::*;

pub fn get_breakpoints() -> Vec<FunctionDef> {
    vec![
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructA".into(),
                fn_name: "impl_func_a".into(),
                is_async: true,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 4,
                    column: Some(52),
                },
                end: LineColumn {
                    line: 5,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 6,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructB".into(),
                fn_name: "impl_func_b1".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 16,
                    column: Some(105),
                },
                end: LineColumn {
                    line: 17,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 18,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructB".into(),
                fn_name: "impl_func_b2".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 20,
                    column: Some(44),
                },
                end: LineColumn {
                    line: 21,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 22,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructC".into(),
                fn_name: "impl_func_c".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 28,
                    column: Some(35),
                },
                end: LineColumn {
                    line: 29,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 30,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructD".into(),
                fn_name: "impl_func_d".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 39,
                    column: Some(69),
                },
                end: LineColumn {
                    line: 40,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 41,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructE".into(),
                fn_name: "impl_func_e".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 48,
                    column: Some(43),
                },
                end: LineColumn {
                    line: 49,
                    column: Some(17),
                },
            },
            end: LineColumn {
                line: 50,
                column: Some(13),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructF".into(),
                fn_name: "impl_func_f".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 57,
                    column: Some(39),
                },
                end: LineColumn {
                    line: 58,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 59,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructF".into(),
                fn_name: "impl_func_f_empty".into(),
                is_async: false,
                is_static: true,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 61,
                    column: Some(37),
                },
                end: LineColumn {
                    line: 61,
                    column: Some(37),
                },
            },
            end: LineColumn {
                line: 63,
                column: Some(9),
            },
        },
    ]
}
