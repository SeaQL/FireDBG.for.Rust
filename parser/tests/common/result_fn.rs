fn result_a() -> Result<(), u32> {
    Ok(())
}

fn result_b() -> Result<i32, u32> {
    Ok(0)
}

fn result_c() -> Result<&'static str, i32> {
    Ok("hi")
}

fn result_d() -> Result<&'static str, &'static str> {
    Err("eh")
}

use firedbg_protocol::source::*;
use firedbg_rust_parser::*;

pub fn get_breakpoints() -> Vec<FunctionDef> {
    vec![
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "result_a".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 1,
                    column: Some(35),
                },
                end: LineColumn {
                    line: 2,
                    column: Some(5),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "result_b".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 5,
                    column: Some(36),
                },
                end: LineColumn {
                    line: 6,
                    column: Some(5),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "result_c".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 9,
                    column: Some(45),
                },
                end: LineColumn {
                    line: 10,
                    column: Some(5),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "result_d".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 13,
                    column: Some(54),
                },
                end: LineColumn {
                    line: 14,
                    column: Some(5),
                },
            },
        },
    ]
}
