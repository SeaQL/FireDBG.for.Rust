use firedbg_protocol::source::LineColumn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StmtOrExpr {
    Stmt(Statement),
    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub ty: StatementType,
    pub loc: LineColumn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementType {
    Let {
        binding: Binding,
        mutable: bool,
    },
    LetAssign {
        binding: Binding,
        mutable: bool,
    },
    Assign {
        binding: Binding,
        assign_op: AssignOp,
    },
    Constant {
        binding: Binding,
    },
    Static {
        binding: Binding,
    },
    Break {},
    Continue {},
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Binding {
    Var(String),
    Field {
        base: String,
        inter: Option<String>,
        member: String,
    },
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,
    /// The addition assignment operator `+=`
    AddAssign,
    /// The bitwise AND assignment operator `&=`
    BitAndAssign,
    /// The bitwise OR assignment operator `|=`
    BitOrAssign,
    /// The bitwise XOR assignment operator `^=`
    BitXorAssign,
    /// The division assignment operator `/=`
    DivAssign,
    /// The multiplication assignment operator `*=`
    MulAssign,
    /// The remainder assignment operator `%=`
    RemAssign,
    /// The left shift assignment operator `<<=`
    ShlAssign,
    /// The right shift assignment operator `>>=`
    ShrAssign,
    /// The subtraction assignment operator `-=`
    SubAssign,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    pub ty: ExpressionType,
    pub loc: LineColumn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionType {
    Other(String),
}

impl From<Statement> for StmtOrExpr {
    fn from(stmt: Statement) -> Self {
        Self::Stmt(stmt)
    }
}

impl From<Expression> for StmtOrExpr {
    fn from(expr: Expression) -> Self {
        Self::Expr(expr)
    }
}
