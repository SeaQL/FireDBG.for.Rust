use crate::{
    AssignOp, Binding, Expression, ExpressionType, IntoLineColumn, Statement, StatementType,
    StmtOrExpr,
};
use syn::{__private::ToTokens, spanned::Spanned};

// Entry point of statement and expression parsing
pub(crate) fn parse_body(syn_stmts: &[syn::Stmt]) -> Vec<StmtOrExpr> {
    syn_stmts.iter().fold(Vec::new(), |mut acc, syn_stmt| {
        match syn_stmt {
            // Parse `let a;` let or `let a = 1;` let assign declaration
            syn::Stmt::Local(local) => acc.extend(local.parse_body()),
            // Parse statements or expressions
            //  - a statement should ends with a `;`
            //  - a expression does NOT end with a `;`
            syn::Stmt::Expr(expr, semi) => acc.extend((expr, semi).parse_body()),
            // Parse items, we only interested in these two items
            //  - `const PI_CONST: f64 = 3.14;`
            //  - `static PI_STATIC: f64 = 3.14;`
            syn::Stmt::Item(item) => acc.extend(item.parse_body()),
            // Parse macro invocation, it can be a
            //  - a statement: `println!("HI");`
            //  - a expression: `format!("HI")`
            syn::Stmt::Macro(stmt_macro) => acc.extend(stmt_macro.parse_body()),
        }
        acc
    })
}

// Parse any AST into statement or expression
pub(crate) trait ParseBody {
    fn parse_body(&self) -> Vec<StmtOrExpr>;
}

// Parse `let a;` let or `let a = 1;` let assign declaration
impl ParseBody for syn::Local {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        let mut mutable = false;
        // Parse the binding value
        let binding = match &self.pat {
            syn::Pat::Ident(pat_ident) => {
                mutable = pat_ident.mutability.is_some();
                Binding::Var(pat_ident.ident.to_string())
            }
            _ => Binding::Other(self.pat.to_token_stream().to_string()),
        };
        // Determine if it's a
        //  - let: `let a;`
        //  - let assign: `let a = 1;`
        let ty = match self.init {
            Some(_) => StatementType::LetAssign { binding, mutable },
            None => StatementType::Let { binding, mutable },
        };
        let mut body = vec![Statement {
            ty,
            loc: self.span().end().into_loc(),
        }
        .into()];
        // Parse the right side of "let assign" if any
        if let Some(local_init) = &self.init {
            // Parse `= Some(1)` in `let a = Some(1);`
            match local_init.expr.as_ref() {
                syn::Expr::Block(_)
                | syn::Expr::ForLoop(_)
                | syn::Expr::If(_)
                | syn::Expr::Match(_)
                | syn::Expr::While(_)
                | syn::Expr::Loop(_) => {
                    body.extend(local_init.expr.parse_body());
                }
                _ => {}
            };
            // Parse `else { return }` in `let Ok(x) = r else { return };`
            if let Some((_, expr)) = &local_init.diverge {
                body.extend(expr.parse_body());
            }
        }
        body
    }
}

// Parse statements or expressions
//  - a statement should ends with a `;`
//  - a expression does NOT end with a `;`
impl ParseBody for (&syn::Expr, &Option<syn::token::Semi>) {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        let (expr, semi) = self;
        match (expr, semi) {
            // These statements or expressions are of interest
            (syn::Expr::Assign(_), _)
            | (syn::Expr::Binary(_), _)
            | (syn::Expr::Block(_), _)
            | (syn::Expr::Break(_), _)
            | (syn::Expr::Continue(_), _)
            | (syn::Expr::ForLoop(_), _)
            | (syn::Expr::If(_), _)
            | (syn::Expr::Match(_), _)
            | (syn::Expr::While(_), _)
            | (syn::Expr::Loop(_), _) => expr.parse_body(),
            // Any other statements end with `;`
            (expr, Some(_)) => vec![Statement {
                ty: StatementType::Other(expr.to_token_stream().to_string()),
                loc: expr.span().end().into_loc(),
            }
            .into()],
            // Any other expressions without `;` at the end
            (expr, None) => vec![Expression {
                ty: ExpressionType::Other(expr.to_token_stream().to_string()),
                loc: expr.span().end().into_loc(),
            }
            .into()],
        }
    }
}

// Parse the inner statements or expressions
impl ParseBody for syn::Expr {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        match self {
            syn::Expr::Assign(expr_assign) => expr_assign.parse_body(),
            syn::Expr::Binary(expr_binary) => expr_binary.parse_body(),
            syn::Expr::Block(expr_block) => expr_block.parse_body(),
            syn::Expr::Break(expr_break) => expr_break.parse_body(),
            syn::Expr::Continue(expr_continue) => expr_continue.parse_body(),
            syn::Expr::ForLoop(expr_for_loop) => expr_for_loop.parse_body(),
            syn::Expr::If(expr_if) => expr_if.parse_body(),
            syn::Expr::Match(expr_match) => expr_match.parse_body(),
            syn::Expr::While(expr_while) => expr_while.parse_body(),
            syn::Expr::Loop(expr_loop) => expr_loop.parse_body(),
            _ => vec![],
        }
    }
}

// Parse `a = 1;` assignment
impl ParseBody for syn::ExprAssign {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Assign {
                binding: binding(&self.left),
                assign_op: AssignOp::Assign,
            },
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse `a += 1;` assignment
impl ParseBody for syn::ExprBinary {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Assign {
                binding: binding(&self.left),
                assign_op: assign_op(&self.op),
            },
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse the value binding
fn binding(expr: &syn::Expr) -> Binding {
    match expr {
        syn::Expr::Path(expr_path)
            if expr_path.path.leading_colon.is_none() && expr_path.path.segments.len() == 1 =>
        {
            Binding::Var(expr_path.path.to_token_stream().to_string())
        }
        syn::Expr::Field(expr_field) => {
            // For example the field looks like `nested_of_6.0.4.3.2.1.0 = 6;`
            let parse_member = |member: &syn::Member| match member {
                syn::Member::Named(ident) => ident.to_string(),
                syn::Member::Unnamed(index) => index.index.to_string(),
            };
            let mut paths = vec![parse_member(&expr_field.member)];
            let mut curr = Some(expr_field.base.as_ref());
            while let Some(expr) = curr {
                if let syn::Expr::Field(expr_field) = expr {
                    paths.push(parse_member(&expr_field.member));
                    curr = Some(expr_field.base.as_ref());
                } else if let syn::Expr::Path(expr_path) = expr {
                    paths.push(expr_path.to_token_stream().to_string());
                    curr = None;
                } else {
                    paths.push(expr.to_token_stream().to_string());
                    curr = None;
                }
            }
            // `paths = ["0", "1", "2", "3", "4", "0", "nested_of_6"]`
            let base = paths.pop().expect("Have base"); // "nested_of_6"
            paths.reverse(); // `paths = ["0", "4", "3", "2", "1", "0"]`
            let member = paths.pop().expect("Have member"); // "0"
            let inter = paths.join("."); // "0.4.3.2.1"
            Binding::Field {
                base,
                inter: if inter.is_empty() { None } else { Some(inter) },
                member,
            }
        }
        _ => Binding::Other(expr.to_token_stream().to_string()),
    }
}

// Parse the assignment operator
fn assign_op(bin_op: &syn::BinOp) -> AssignOp {
    match bin_op {
        syn::BinOp::AddAssign(_) => AssignOp::AddAssign,
        syn::BinOp::SubAssign(_) => AssignOp::SubAssign,
        syn::BinOp::MulAssign(_) => AssignOp::MulAssign,
        syn::BinOp::DivAssign(_) => AssignOp::DivAssign,
        syn::BinOp::RemAssign(_) => AssignOp::RemAssign,
        syn::BinOp::BitXorAssign(_) => AssignOp::BitXorAssign,
        syn::BinOp::BitAndAssign(_) => AssignOp::BitAndAssign,
        syn::BinOp::BitOrAssign(_) => AssignOp::BitOrAssign,
        syn::BinOp::ShlAssign(_) => AssignOp::ShlAssign,
        syn::BinOp::ShrAssign(_) => AssignOp::ShrAssign,
        _ => AssignOp::Other(bin_op.to_token_stream().to_string()),
    }
}

// Parse all the statements and expressions in a block `{ ... }`
impl ParseBody for syn::ExprBlock {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        parse_body(&self.block.stmts)
    }
}

// Parse a `break;` statement
impl ParseBody for syn::ExprBreak {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Break {},
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse a `continue;` statement
impl ParseBody for syn::ExprContinue {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Continue {},
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse all the statements and expressions in the for loop block `for _ in vec { ... }`
impl ParseBody for syn::ExprForLoop {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        parse_body(&self.body.stmts)
    }
}

// Parse all the statements and expressions in the if block and its else branch `if { ... } else if bool { ... } else { ... }`
impl ParseBody for syn::ExprIf {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        let mut body = parse_body(&self.then_branch.stmts);
        if let Some((_, expr)) = &self.else_branch {
            body.extend(expr.parse_body());
        }
        body
    }
}

// Parse all the statements and expressions in each match arms
impl ParseBody for syn::ExprMatch {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        self.arms.iter().fold(Vec::new(), |mut acc, match_arm| {
            acc.extend(match_arm.parse_body());
            acc
        })
    }
}

// Parse all the statements and expressions in a match arm
impl ParseBody for syn::Arm {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        self.body.parse_body()
    }
}

// Parse all the statements and expressions in the while loop block `while bool { ... }`
impl ParseBody for syn::ExprWhile {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        parse_body(&self.body.stmts)
    }
}

// Parse all the statements and expressions in the loop block `loop { ... }`
impl ParseBody for syn::ExprLoop {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        parse_body(&self.body.stmts)
    }
}

// Parse items, we only interested in these two items
//  - `const PI_CONST: f64 = 3.14;`
//  - `static PI_STATIC: f64 = 3.14;`
impl ParseBody for syn::Item {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        match self {
            syn::Item::Const(item_const) => item_const.parse_body(),
            syn::Item::Static(item_static) => item_static.parse_body(),
            _ => vec![],
        }
    }
}

// Parse a const declaration statement, `const PI_CONST: f64 = 3.14;`
impl ParseBody for syn::ItemConst {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Constant {
                binding: Binding::Var(self.ident.to_string()),
            },
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse a static declaration statement, `static PI_STATIC: f64 = 3.14;`
impl ParseBody for syn::ItemStatic {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        vec![Statement {
            ty: StatementType::Static {
                binding: Binding::Var(self.ident.to_string()),
            },
            loc: self.span().end().into_loc(),
        }
        .into()]
    }
}

// Parse macro invocation, it can be a
//  - a statement: `println!("HI");`
//  - a expression: `format!("HI")`
impl ParseBody for syn::StmtMacro {
    fn parse_body(&self) -> Vec<StmtOrExpr> {
        let s = self.to_token_stream().to_string();
        let loc = self.span().end().into_loc();
        let stmt_or_expr = match self.semi_token {
            Some(_) => Statement {
                ty: StatementType::Other(s),
                loc,
            }
            .into(),
            None => Expression {
                ty: ExpressionType::Other(s),
                loc,
            }
            .into(),
        };
        vec![stmt_or_expr]
    }
}
