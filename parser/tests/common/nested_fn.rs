use std::fs::File;

mod abc {
    // Some comments...
}

async fn free_func_a(i: i32) -> i32 {
    for n in 0..i {
        println!("free_func_a for loop {n}");
    }
    fn nested_func_a_a() {
        fn nested_func_a_a_a() {
            
        }
        fn nested_func_a_a_b() {
            fn nested_func_a_a_b_a() {
                mod a {
                    fn mod_a_nested_func() { }
                    mod b {
                        fn mod_b_nested_func() { }
                    }
                }
            }
        }
    }
    i
}

mod module_a {
    use super::*;

    async fn free_func_b(i: i32) -> i32 {
        fn nested_func_b_a() {
            fn nested_func_b_a_a() {
                fn nested_func_b_a_a_a() {
    
                }
            }
        }
        i
    }

    mod module_a_a {
        async fn free_func_c(i: i32) -> i32 {
            fn nested_func_c_a() {
                fn nested_func_c_a_a() {
                    fn nested_func_c_a_a_a() {
        
                    }
                }
            }
            i
        }

        mod module_a_a_a {
            async fn free_func_d(i: i32) -> i32 {
                fn nested_func_d_a() {
                    fn nested_func_d_a_a() {
                        fn nested_func_d_a_a_a() {
            
                        }
                    }
                }
                i
            }
        }
    }
}

struct StructA;

impl StructA {
    fn impl_func_a(self) -> i32 {
        fn nested_func_a() -> i32 {
            fn nested_func_a_a() { }
            fn nested_func_a_b() { }
            0
        }
        nested_func_a()
    }
}

trait TraitA {
    fn trait_a_default_func() -> usize {
        fn trait_a_nested_func_a() { }
        fn trait_a_nested_func_b() -> usize {
            trait_a_nested_func_a();
            fn trait_a_nested_func_b_a() { }
            fn trait_a_nested_func_b_b() -> usize {
                fn trait_a_nested_func_b_b_a() -> usize {
                    mod a {
                        fn mod_a_nested_func() { }
                        mod b {
                            fn mod_b_nested_func() { }
                        }
                    }
                    0
                }
                trait_a_nested_func_b_b_a()
            }
            trait_a_nested_func_b_b()
        }
        trait_a_nested_func_b()
    }

    fn trait_a_required_func();
}

impl TraitA for StructA {
    fn trait_a_required_func() {
        fn trait_a_required_func_nested() { }
        mod a {
            fn mod_a_nested_func() { }
            mod b {
                fn mod_b_nested_func() { }
            }
        }
    }
}

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
                    column: Some(38),
                },
                end: LineColumn {
                    line: 8,
                    column: Some(5),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_a".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 11,
                    column: Some(27),
                },
                end: LineColumn {
                    line: 12,
                    column: Some(9),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_a_a".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 12,
                    column: Some(33),
                },
                end: LineColumn {
                    line: 12,
                    column: Some(33),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_a_b".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 15,
                    column: Some(33),
                },
                end: LineColumn {
                    line: 16,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_a_b_a".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 16,
                    column: Some(39),
                },
                end: LineColumn {
                    line: 17,
                    column: Some(17),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_a_nested_func".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 18,
                    column: Some(45),
                },
                end: LineColumn {
                    line: 18,
                    column: Some(45),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_b_nested_func".into(),
                parent_func: "free_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 20,
                    column: Some(49),
                },
                end: LineColumn {
                    line: 20,
                    column: Some(49),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_b".into(),
                is_async: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 32,
                    column: Some(42),
                },
                end: LineColumn {
                    line: 33,
                    column: Some(9),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_b_a".into(),
                parent_func: "free_func_b".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 33,
                    column: Some(31),
                },
                end: LineColumn {
                    line: 34,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_b_a_a".into(),
                parent_func: "free_func_b".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 34,
                    column: Some(37),
                },
                end: LineColumn {
                    line: 35,
                    column: Some(17),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_b_a_a_a".into(),
                parent_func: "free_func_b".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 35,
                    column: Some(43),
                },
                end: LineColumn {
                    line: 35,
                    column: Some(43),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_c".into(),
                is_async: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 44,
                    column: Some(46),
                },
                end: LineColumn {
                    line: 45,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_c_a".into(),
                parent_func: "free_func_c".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 45,
                    column: Some(35),
                },
                end: LineColumn {
                    line: 46,
                    column: Some(17),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_c_a_a".into(),
                parent_func: "free_func_c".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 46,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 47,
                    column: Some(21),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_c_a_a_a".into(),
                parent_func: "free_func_c".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 47,
                    column: Some(47),
                },
                end: LineColumn {
                    line: 47,
                    column: Some(47),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::FreeFn {
                fn_name: "free_func_d".into(),
                is_async: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 56,
                    column: Some(50),
                },
                end: LineColumn {
                    line: 57,
                    column: Some(17),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_d_a".into(),
                parent_func: "free_func_d".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 57,
                    column: Some(39),
                },
                end: LineColumn {
                    line: 58,
                    column: Some(21),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_d_a_a".into(),
                parent_func: "free_func_d".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 58,
                    column: Some(45),
                },
                end: LineColumn {
                    line: 59,
                    column: Some(25),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_d_a_a_a".into(),
                parent_func: "free_func_d".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 59,
                    column: Some(51),
                },
                end: LineColumn {
                    line: 59,
                    column: Some(51),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::ImplFn {
                self_type: "StructA".into(),
                fn_name: "impl_func_a".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 73,
                    column: Some(34),
                },
                end: LineColumn {
                    line: 74,
                    column: Some(9),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a".into(),
                parent_func: "impl_func_a".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 74,
                    column: Some(36),
                },
                end: LineColumn {
                    line: 75,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_a".into(),
                parent_func: "impl_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 75,
                    column: Some(35),
                },
                end: LineColumn {
                    line: 75,
                    column: Some(35),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "nested_func_a_b".into(),
                parent_func: "impl_func_a".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 76,
                    column: Some(35),
                },
                end: LineColumn {
                    line: 76,
                    column: Some(35),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::TraitDefaultFn {
                trait_name: "TraitA".into(),
                fn_name: "trait_a_default_func".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 84,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 85,
                    column: Some(9),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_nested_func_a".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 85,
                    column: Some(37),
                },
                end: LineColumn {
                    line: 85,
                    column: Some(37),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_nested_func_b".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 86,
                    column: Some(46),
                },
                end: LineColumn {
                    line: 87,
                    column: Some(13),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_nested_func_b_a".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 88,
                    column: Some(43),
                },
                end: LineColumn {
                    line: 88,
                    column: Some(43),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_nested_func_b_b".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 89,
                    column: Some(52),
                },
                end: LineColumn {
                    line: 90,
                    column: Some(17),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_nested_func_b_b_a".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 90,
                    column: Some(58),
                },
                end: LineColumn {
                    line: 91,
                    column: Some(21),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_a_nested_func".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 92,
                    column: Some(49),
                },
                end: LineColumn {
                    line: 92,
                    column: Some(49),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_b_nested_func".into(),
                parent_func: "trait_a_default_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 94,
                    column: Some(53),
                },
                end: LineColumn {
                    line: 94,
                    column: Some(53),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructA".into(),
                fn_name: "trait_a_required_func".into(),
                is_async: false,
                is_static: true,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 110,
                    column: Some(33),
                },
                end: LineColumn {
                    line: 111,
                    column: Some(9),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "trait_a_required_func_nested".into(),
                parent_func: "trait_a_required_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 111,
                    column: Some(44),
                },
                end: LineColumn {
                    line: 111,
                    column: Some(44),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_a_nested_func".into(),
                parent_func: "trait_a_required_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 113,
                    column: Some(37),
                },
                end: LineColumn {
                    line: 113,
                    column: Some(37),
                },
            },
        },
        FunctionDef {
            ty: FunctionType::NestedFn {
                fn_name: "mod_b_nested_func".into(),
                parent_func: "trait_a_required_func".into(),
                is_async: false,
                return_type: false,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 115,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 115,
                    column: Some(41),
                },
            },
        },
    ]
}
