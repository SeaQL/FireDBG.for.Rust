use crate::BreakableSpan;
use firedbg_protocol::source::LineColumn;
use syn::spanned::Spanned;

pub(crate) fn parse_body_loc(block: &syn::Block) -> BreakableSpan {
    BreakableSpan {
        start: parse_body_loc_start(block),
        end: parse_body_loc_end(block),
    }
}

pub(crate) fn parse_body_end(block: &syn::Block) -> LineColumn {
    block.span().end().into_loc()
}

/// Parse the line and column number of function block.
///
/// A function block is wrapped in a pair of braces, i.e. `{ ... }`
pub(crate) fn parse_body_loc_start(block: &syn::Block) -> LineColumn {
    // Given a function like... the `loc.column` will be at
    //   - `fn func<T>(i: T) -> T where T: Into<u64> { `
    //                                              ^
    //   - `fn func<T>(i: T) -> T where T: Into<u64>{ `
    //                                             ^
    //   - `fn func() { `
    //               ^
    //   - `fn func(){ `
    //              ^
    // However, we want the `loc.column` to be one character after `{`, i.e.
    //   - `fn func<T>(i: T) -> T where T: Into<u64> { `
    //                                                ^
    //   - `fn func<T>(i: T) -> T where T: Into<u64>{ `
    //                                               ^
    //   - `fn func() { `
    //                 ^
    //   - `fn func(){ `
    //                ^
    let mut loc: LineColumn = block.span().start().into_loc();
    // Hence, `loc.column + 2`
    if let Some(col) = loc.column.as_mut() {
        *col += 2;
    }
    loc
}

/// Parse the line and column number of function block.
///
/// A function block is wrapped in a pair of braces, i.e. `{ ... }`
///
/// 1. If the function block is empty, this function will return the result of [parse_body_loc_start] function
///
/// 2. If the function block has statement in it, this function will return the line and column number of the beginning of first statement.
///
/// ```compile_fail
/// fn hello_3(world: &World) { println!("hello {}", world) }
///                             ^
///
/// fn hello_5(world: &World) { let world = World { nth: 99 }; println!("hello {}", world) }
///                             ^
///
/// fn hello_7(world:&World){let world=World{nth:99};println!("hello {}",world)}
///                          ^
///
/// fn hello_15(world: &World) {let world=World{nth:99};
///                             ^
/// println!("hello {}",world)}
///
/// fn hello_17(world: &World) { let world=World{nth:99};
///                              ^
/// println!("hello {}",world)}
///
/// fn hello_13(world: &World) {
/// let world=World{nth:99};
/// ^
///         println!("hello {}",world)}
///
/// fn hello_19(world: &World) {
///     
///     let world=World{nth:99};
///     ^
///     println!("hello {}",world)
///
/// }
///
/// impl Display for World {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}th world", self.nth)
///         ^
///     }
/// }
/// ```
fn parse_body_loc_end(block: &syn::Block) -> LineColumn {
    let mut loc = parse_body_loc_start(block);
    if let Some(first_stmt) = block.stmts.first() {
        let mut first_stmt_loc: LineColumn = first_stmt.span().start().into_loc();
        if let Some(col) = first_stmt_loc.column.as_mut() {
            *col += 1;
        }
        loc = first_stmt_loc;
    }
    loc
}

pub(crate) trait IntoLineColumn {
    fn into_loc(self) -> LineColumn;
}

impl IntoLineColumn for proc_macro2::LineColumn {
    fn into_loc(self) -> LineColumn {
        LineColumn {
            line: self.line as u32,
            column: Some(self.column as u32),
        }
    }
}
