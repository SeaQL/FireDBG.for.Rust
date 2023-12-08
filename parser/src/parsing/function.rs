use crate::{parsing::parse_body_loc, FunctionDef, FunctionType};
use syn::__private::ToTokens;

// Parse function breakpoints
pub(crate) trait ParseFunction {
    fn parse(&self) -> Vec<FunctionDef>;
}

// Entry point of function breaking parsing
impl ParseFunction for syn::Item {
    fn parse(&self) -> Vec<FunctionDef> {
        match self {
            // Parse standalone function
            syn::Item::Fn(item_fn) => item_fn.parse(),
            // Parse impl block
            syn::Item::Impl(item_impl) => item_impl.parse(),
            // Parse default implementation of trait method
            syn::Item::Trait(item_trait) => item_trait.parse(),
            // Parse `Item` in module
            syn::Item::Mod(item_mod) => item_mod.parse(),
            _ => Vec::new(),
        }
    }
}

// Parse standalone function
impl ParseFunction for syn::ItemFn {
    fn parse(&self) -> Vec<FunctionDef> {
        let fn_name = fn_name(&self.sig);
        let is_async = is_async(&self.sig);
        let return_type = return_type(&self.sig);
        let ty = FunctionType::FreeFn {
            fn_name,
            is_async,
            return_type,
        };
        let loc = parse_body_loc(&self.block);
        let parent_breakpoint = FunctionDef { ty, loc };
        parse_nested_func(parent_breakpoint, &self.block)
    }
}

// Parse impl block
impl ParseFunction for syn::ItemImpl {
    fn parse(&self) -> Vec<FunctionDef> {
        self.items.iter().fold(Vec::new(), |mut acc, impl_item| {
            match impl_item {
                // We only want to parse the function in the impl block
                syn::ImplItem::Fn(impl_item_fn) => {
                    acc.extend((self, impl_item_fn).parse());
                }
                _ => {}
            }
            acc
        })
    }
}

// Parse the function in the impl block
// We need a tuple value for parsing because we need to read the self type from the `ItemImpl`
impl ParseFunction for (&syn::ItemImpl, &syn::ImplItemFn) {
    fn parse(&self) -> Vec<FunctionDef> {
        let (item_impl, impl_item_fn) = self;
        let self_type = self_type(&item_impl.self_ty);
        let fn_name = fn_name(&impl_item_fn.sig);
        let is_async = is_async(&impl_item_fn.sig);
        let is_static = is_static(&impl_item_fn.sig);
        let return_type = return_type(&impl_item_fn.sig);
        let ty = match &item_impl.trait_ {
            Some((_, path, _)) => FunctionType::ImplTraitFn {
                trait_name: trait_name(path),
                self_type,
                fn_name,
                is_async,
                is_static,
                return_type,
            },
            None => FunctionType::ImplFn {
                self_type,
                fn_name,
                is_async,
                is_static,
                return_type,
            },
        };
        let loc = parse_body_loc(&impl_item_fn.block);
        let parent_breakpoint = FunctionDef { ty, loc };
        parse_nested_func(parent_breakpoint, &impl_item_fn.block)
    }
}

// Parse default implementation of trait method
impl ParseFunction for syn::ItemTrait {
    fn parse(&self) -> Vec<FunctionDef> {
        self.items.iter().fold(Vec::new(), |mut acc, trait_item| {
            match trait_item {
                syn::TraitItem::Fn(trait_item_fn) => {
                    acc.extend((self, trait_item_fn).parse());
                }
                _ => {}
            }
            acc
        })
    }
}

// Parse default implementation of trait method
// We need a tuple value for parsing because we need to read the self type from the `ItemTrait`
impl ParseFunction for (&syn::ItemTrait, &syn::TraitItemFn) {
    fn parse(&self) -> Vec<FunctionDef> {
        let (item_trait, trait_item_fn) = self;
        match &trait_item_fn.default {
            Some(block) => {
                let trait_name = item_trait.ident.to_string();
                let fn_name = fn_name(&trait_item_fn.sig);
                let is_async = is_async(&trait_item_fn.sig);
                let is_static = is_static(&trait_item_fn.sig);
                let return_type = return_type(&trait_item_fn.sig);
                let ty = FunctionType::TraitDefaultFn {
                    trait_name,
                    fn_name,
                    is_async,
                    is_static,
                    return_type,
                };
                let loc = parse_body_loc(block);
                let parent_breakpoint = FunctionDef { ty, loc };
                parse_nested_func(parent_breakpoint, block)
            }
            None => Vec::new(),
        }
    }
}

// Parse `Item` in module
impl ParseFunction for syn::ItemMod {
    fn parse(&self) -> Vec<FunctionDef> {
        match &self.content {
            Some((_, items)) => items.iter().fold(Vec::new(), |mut acc, item| {
                acc.extend(item.parse());
                acc
            }),
            None => Vec::new(),
        }
    }
}

fn self_type(ty: &syn::Type) -> String {
    format!("{}", ty.to_token_stream())
}

fn trait_name(path: &syn::Path) -> String {
    format!("{}", path.segments.to_token_stream())
}

fn fn_name(signature: &syn::Signature) -> String {
    signature.ident.to_string()
}

fn is_async(signature: &syn::Signature) -> bool {
    signature.asyncness.is_some()
}

fn return_type(signature: &syn::Signature) -> bool {
    match &signature.output {
        // A function without explicit return type
        // `fn func() { ... }`
        syn::ReturnType::Default => false,
        // A function with explicit return type
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            // A function return unit (i.e. an empty tuple)
            // `fn func() -> () { ... }`
            syn::Type::Tuple(type_tuple) if type_tuple.elems.is_empty() => false,
            // A never retuning function
            // `fn func() -> ! { ... }`
            syn::Type::Never(_) => false,
            // With return type
            _ => true,
        },
    }
}

fn is_static(signature: &syn::Signature) -> bool {
    !signature
        .inputs
        .iter()
        .any(|fn_arg| matches!(fn_arg, syn::FnArg::Receiver(_)))
}

fn parse_nested_func(parent_breakpoint: FunctionDef, block: &syn::Block) -> Vec<FunctionDef> {
    let mut nested_func = block.stmts.iter().fold(Vec::new(), |mut acc, stmt| {
        match stmt {
            syn::Stmt::Item(item) => {
                acc.extend(item.parse().into_iter().map(|mut breakpoint| {
                    breakpoint.ty = breakpoint.ty.into_nested_func(&parent_breakpoint);
                    breakpoint
                }));
            }
            _ => {}
        }
        acc
    });
    let mut breakpoints = vec![parent_breakpoint];
    breakpoints.append(&mut nested_func);
    breakpoints
}
