use std::fs::File;

mod abc {
    // Some comments...
}

async fn free_func_a(i: i32) -> i32{
    for n in 0..i {
        println!("free_func_a for loop {n}");
    }
    i
}

pub fn free_func_b(i: usize) -> impl std::future::Future<Output = usize> { // FIXME: Should this be parsed as async function?
    while false {
        panic!("free_func_b");
    }
    async move { i }
}

pub(crate) fn free_func_c<T: Into<i64>>(i: T) -> T {
    loop {
        break;
    }
    i
}

mod module_a {
    use super::*;

    pub(crate) fn free_func_d<T>(i: T) -> T where T: Into<u64> {
        i
    }

    mod module_a_a {
        fn free_func_e(i: u64) -> u64 {
            i
        }

        mod module_a_a_a {
            fn free_func_f(i: &u64) -> &u64 {
                i
            }
        }
    }

    pub(crate) fn free_func_g(i: u64) -> u64 {
        i
    }
}

fn free_func_h(i: u64) -> () {



}

fn free_func_i(i: u64) -> ! { unimplemented!() }

use firedbg_protocol::source::*;
use firedbg_rust_parser::*;

pub fn get_breakpoints() -> Vec<FunctionDef> {
    vec![
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_a".into(),
                is_async: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 7,
                    column: Some(37),
                },
                end: LineColumn {
                    line: 8,
                    column: Some(5),
                },
            },
            end: LineColumn {
                line: 12,
                column: Some(1),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_b".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 14,
                    column: Some(75),
                },
                end: LineColumn {
                    line: 15,
                    column: Some(5),
                },
            },
            end: LineColumn {
                line: 19,
                column: Some(1),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_c".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 21,
                    column: Some(53),
                },
                end: LineColumn {
                    line: 22,
                    column: Some(5),
                },
            },
            end: LineColumn {
                line: 26,
                column: Some(1),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_d".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 31,
                    column: Some(65),
                },
                end: LineColumn {
                    line: 32,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 33,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_e".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 36,
                    column: Some(40),
                },
                end: LineColumn {
                    line: 37,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 38,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_f".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 41,
                    column: Some(56),
                },
                end: LineColumn {
                    line: 42,
                    column: Some(17),
                },
            },
            end: LineColumn {
                line: 43,
                column: Some(13),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_g".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 47,
                    column: Some(47),
                },
                end: LineColumn {
                    line: 48,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 49,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_h".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 52,
                    column: Some(31),
                },
                end: LineColumn {
                    line: 52,
                    column: Some(31),
                },
            },
            end: LineColumn {
                line: 56,
                column: Some(1),
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_i".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 58,
                    column: Some(30),
                },
                end: LineColumn {
                    line: 58,
                    column: Some(31),
                },
            },
            end: LineColumn {
                line: 58,
                column: Some(48),
            },
        },
    ]
}
