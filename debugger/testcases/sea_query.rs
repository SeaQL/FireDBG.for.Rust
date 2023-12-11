// use std::rc::Rc as RcOrArc;
use std::sync::Arc as RcOrArc;

fn main() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .to_string(),
        ""
    );
}

#[derive(Debug)]
pub enum Character {
    Table,
    Id,
    Character,
    FontSize,
    SizeW,
    SizeH,
    FontId,
    Ascii,
    CreatedAt,
    UserData,
}

pub type Char = Character;

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "character",
                Self::Id => "id",
                Self::Character => "character",
                Self::FontSize => "font_size",
                Self::SizeW => "size_w",
                Self::SizeH => "size_h",
                Self::FontId => "font_id",
                Self::Ascii => "ascii",
                Self::CreatedAt => "created_at",
                Self::UserData => "user_data",
            }
        )
        .unwrap();
    }
}

pub struct Query;

impl Query {
    pub fn select() -> SelectStatement {
        SelectStatement::new()
    }
}

pub struct SelectStatement {
    pub(crate) selects: Vec<SelectExpr>,
    pub(crate) from: Vec<TableRef>,
}

impl SelectStatement {
    /// Construct a new [`SelectStatement`]
    pub fn new() -> Self {
        Self {
            selects: vec![],
            from: vec![],
        }
    }

    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.exprs(
            cols.into_iter()
                .map(|c| SimpleExpr::Column(c.into_column_ref()))
                .collect::<Vec<SimpleExpr>>(),
        )
    }

    pub fn exprs<T, I>(&mut self, exprs: I) -> &mut Self
    where
        T: Into<SelectExpr>,
        I: IntoIterator<Item = T>,
    {
        self.selects
            .append(&mut exprs.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn from<R>(&mut self, tbl_ref: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.from_from(tbl_ref.into_table_ref())
    }

    fn from_from(&mut self, select: TableRef) -> &mut Self {
        self.from.push(select);
        self
    }

    fn to_string(&self) -> String {
        "".into()
    }
}

pub struct SelectExpr {
    pub expr: SimpleExpr,
}

impl<T> From<T> for SelectExpr
where
    T: Into<SimpleExpr>,
{
    fn from(expr: T) -> Self {
        SelectExpr { expr: expr.into() }
    }
}

pub enum SimpleExpr {
    Column(ColumnRef),
}

pub enum ColumnRef {
    Column(DynIden),
}

pub trait IntoColumnRef {
    fn into_column_ref(self) -> ColumnRef;
}

impl<T: 'static> IntoColumnRef for T
where
    T: IntoIden,
{
    fn into_column_ref(self) -> ColumnRef {
        ColumnRef::Column(self.into_iden())
    }
}

pub enum TableRef {
    Table(DynIden),
}

pub trait IntoTableRef {
    fn into_table_ref(self) -> TableRef;
}

impl<T: 'static> IntoTableRef for T
where
    T: IntoIden,
{
    fn into_table_ref(self) -> TableRef {
        TableRef::Table(self.into_iden())
    }
}

pub type DynIden = SeaRc<dyn Iden>;

#[repr(transparent)]
pub struct SeaRc<I>(pub(crate) RcOrArc<I>)
where
    I: ?Sized;

impl SeaRc<dyn Iden> {
    pub fn new<I>(i: I) -> SeaRc<dyn Iden>
    where
        I: Iden + 'static,
    {
        SeaRc(RcOrArc::new(i))
    }
}

pub trait IntoIden {
    fn into_iden(self) -> DynIden;
}

impl<T: 'static> IntoIden for T
where
    T: Iden,
{
    fn into_iden(self) -> DynIden {
        SeaRc::new(self)
    }
}

pub trait Iden {
    fn unquoted(&self, s: &mut dyn std::fmt::Write);
}
