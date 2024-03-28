use async_trait::async_trait;

#[async_trait]
pub trait TraitA {
    async fn func_default<T: Into<i32> + Send>(i: T) -> &'static str {
        "TraitA::func_default()"
    }

    fn func_required() -> &'static str;
}

pub trait TraitB {
    // fn func_self_required(&self) -> impl std::future::Future<Output = &'static str>; // FIXME: Should this be parsed as async function?
    fn func_self_required(&self) -> &'static str;
}

pub struct StructA;

#[async_trait]
impl TraitA for StructA {
    fn func_required() -> &'static str {
        "<StructA as TraitA>::func_required()"
    }
}

impl TraitB for StructA {
    fn func_self_required(&self) -> &'static str {
        "<StructA as TraitB>::func_self_required()"
    }
}

pub struct StructB {
    field_1: i32,
    pub field_2: usize,
    pub(crate) field_3: u64,
}

#[async_trait]
impl TraitA for StructB {
    async fn func_default<T>(i: T) -> &'static str where T: Into<i32> + Send {
        "<StructB as TraitA>::func_default()"
    }

    fn func_required() -> &'static str {
        "<StructB as TraitA>::func_required()"
    }
}

impl TraitB for StructB {
    fn func_self_required(&self) -> &'static str {
        "<StructB as TraitB>::func_self_required()"
    }
}

pub struct StructC(i32, pub usize, pub(crate) u64);

#[async_trait]
impl TraitA for StructC {
    fn func_required() -> &'static str {
        "<StructC as TraitA>::func_required()"
    }
}

impl TraitB for StructC {
    fn func_self_required(&self) -> &'static str {
        "<StructC as TraitB>::func_self_required()"
    }
}

mod module_a {
    use super::*;

    pub trait TraitC {
        fn func_default<T: Into<i32> + Send>(i: T) -> &'static str {
            "TraitC::func_default()"
        }

        fn func_required() -> &'static str;
    }

    pub trait TraitD {
        fn func_self_required(&self) -> &'static str;
    }

    pub struct StructD;

    #[async_trait]
    impl TraitA for StructD {
        fn func_required() -> &'static str {
            "<StructD as TraitA>::func_required()"
        }
    }

    impl TraitB for StructD {
        fn func_self_required(&self) -> &'static str {
            "<StructD as TraitB>::func_self_required()"
        }
    }

    impl TraitD for super::StructA {
        fn func_self_required(&self) -> &'static str {
            "<super::StructA as TraitD>::func_self_required()"
        }
    }
}

use firedbg_protocol::source::*;
use firedbg_rust_parser::*;

pub fn get_breakpoints() -> Vec<FunctionDef> {
    vec![
        FunctionDef {
            ty: FunctionType::TraitDefaultFn {
                trait_name: "TraitA".into(),
                fn_name: "func_default".into(),
                is_async: true,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 5,
                    column: Some(71),
                },
                end: LineColumn {
                    line: 6,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 7,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructA".into(),
                fn_name: "func_required".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 21,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 22,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 23,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitB".into(),
                self_type: "StructA".into(),
                fn_name: "func_self_required".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 27,
                    column: Some(51),
                },
                end: LineColumn {
                    line: 28,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 29,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructB".into(),
                fn_name: "func_default".into(),
                is_async: true,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 40,
                    column: Some(79),
                },
                end: LineColumn {
                    line: 41,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 42,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructB".into(),
                fn_name: "func_required".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 44,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 45,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 46,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitB".into(),
                self_type: "StructB".into(),
                fn_name: "func_self_required".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 50,
                    column: Some(51),
                },
                end: LineColumn {
                    line: 51,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 52,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructC".into(),
                fn_name: "func_required".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 59,
                    column: Some(41),
                },
                end: LineColumn {
                    line: 60,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 61,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitB".into(),
                self_type: "StructC".into(),
                fn_name: "func_self_required".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 65,
                    column: Some(51),
                },
                end: LineColumn {
                    line: 66,
                    column: Some(9),
                },
            },
            end: LineColumn {
                line: 67,
                column: Some(5),
            },
        },
        FunctionDef {
            ty: FunctionType::TraitDefaultFn {
                trait_name: "TraitC".into(),
                fn_name: "func_default".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 74,
                    column: Some(69),
                },
                end: LineColumn {
                    line: 75,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 76,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitA".into(),
                self_type: "StructD".into(),
                fn_name: "func_required".into(),
                is_async: false,
                is_static: true,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 89,
                    column: Some(45),
                },
                end: LineColumn {
                    line: 90,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 91,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitB".into(),
                self_type: "StructD".into(),
                fn_name: "func_self_required".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 95,
                    column: Some(55),
                },
                end: LineColumn {
                    line: 96,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 97,
                column: Some(9),
            },
        },
        FunctionDef {
            ty: FunctionType::ImplTraitFn {
                trait_name: "TraitD".into(),
                self_type: "super :: StructA".into(),
                fn_name: "func_self_required".into(),
                is_async: false,
                is_static: false,
                return_type: true,
            },
            loc: BreakableSpan {
                start: LineColumn {
                    line: 101,
                    column: Some(55),
                },
                end: LineColumn {
                    line: 102,
                    column: Some(13),
                },
            },
            end: LineColumn {
                line: 103,
                column: Some(9),
            },
        },
    ]
}
