use firedbg_protocol::source::LineColumn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionDef {
    pub ty: FunctionType,
    pub loc: BreakableSpan,
    pub end: LineColumn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakableSpan {
    pub start: LineColumn,
    pub end: LineColumn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionType {
    FreeFn {
        fn_name: String,
        is_async: bool,
        return_type: bool,
    },
    ImplFn {
        self_type: String,
        fn_name: String,
        is_async: bool,
        is_static: bool,
        return_type: bool,
    },
    ImplTraitFn {
        trait_name: String,
        self_type: String,
        fn_name: String,
        is_async: bool,
        is_static: bool,
        return_type: bool,
    },
    TraitDefaultFn {
        trait_name: String,
        fn_name: String,
        is_async: bool,
        is_static: bool,
        return_type: bool,
    },
    NestedFn {
        fn_name: String,
        parent_func: String,
        is_async: bool,
        return_type: bool,
    },
}

impl FunctionType {
    pub fn into_nested_func(self, parent_breakpoint: &FunctionDef) -> FunctionType {
        let parent_func = parent_breakpoint.ty.fn_name().to_string();
        match self {
            FunctionType::FreeFn {
                fn_name,
                is_async,
                return_type,
            } => FunctionType::NestedFn {
                fn_name,
                parent_func,
                is_async,
                return_type,
            },
            FunctionType::ImplFn {
                fn_name,
                is_async,
                return_type,
                ..
            } => FunctionType::NestedFn {
                fn_name,
                parent_func,
                is_async,
                return_type,
            },
            FunctionType::ImplTraitFn {
                fn_name,
                is_async,
                return_type,
                ..
            } => FunctionType::NestedFn {
                fn_name,
                parent_func,
                is_async,
                return_type,
            },
            FunctionType::TraitDefaultFn {
                fn_name,
                is_async,
                return_type,
                ..
            } => FunctionType::NestedFn {
                fn_name,
                parent_func,
                is_async,
                return_type,
            },
            FunctionType::NestedFn {
                fn_name,
                is_async,
                return_type,
                ..
            } => FunctionType::NestedFn {
                fn_name,
                parent_func,
                is_async,
                return_type,
            },
        }
    }

    pub fn fn_name(&self) -> &str {
        match self {
            FunctionType::FreeFn { fn_name, .. } => &fn_name,
            FunctionType::ImplFn { fn_name, .. } => &fn_name,
            FunctionType::ImplTraitFn { fn_name, .. } => &fn_name,
            FunctionType::TraitDefaultFn { fn_name, .. } => &fn_name,
            FunctionType::NestedFn { fn_name, .. } => &fn_name,
        }
    }
}
