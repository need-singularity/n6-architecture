/// AST node types — the structured representation of HEXA-LANG programs
///
/// Node count follows n=6 arithmetic:
///   Decl:     tau=4 variants
///   Stmt:     n=6 variants
///   Expr:     sigma+phi=14 variants (Mk.I + ArrayLit + Match)
///   TypeExpr: n/phi=3 variants

use crate::lexer::Span;

/// Top-level program: a sequence of declarations
#[derive(Clone, Debug)]
pub struct Program {
    pub decls: Vec<Decl>,
}

// ── Visibility ──

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    Private,
    Public,
}

// ── Declarations (tau+n/phi=7 variants) ──

#[derive(Clone, Debug)]
pub enum Decl {
    FnDecl(FnDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TypeAlias(TypeAliasDecl),
    ModuleDecl(ModuleDecl),
    UseDecl(UseDecl),
    TraitDecl(TraitDecl),
    ImplBlock(ImplBlock),
}

/// Trait method signature (no body)
#[derive(Clone, Debug)]
pub struct TraitMethodSig {
    pub name: String,
    pub params: Vec<(String, TypeExpr)>, // excludes &self
    pub ret_ty: Option<TypeExpr>,
    pub span: Span,
}

/// Trait definition: `trait Name { fn method(&self, ...); ... }`
#[derive(Clone, Debug)]
pub struct TraitDecl {
    pub name: String,
    pub methods: Vec<TraitMethodSig>,
    pub span: Span,
}

/// Impl block: `impl TraitName for TypeName { fn method(&self) { ... } ... }`
#[derive(Clone, Debug)]
pub struct ImplBlock {
    pub trait_name: String,
    pub target_type: String,
    pub methods: Vec<FnDecl>,
    pub span: Span,
}

/// Module declaration: `mod name { decl* }`
#[derive(Clone, Debug)]
pub struct ModuleDecl {
    pub name: String,
    pub decls: Vec<Decl>,
    pub span: Span,
}

/// Use declaration: `use path::to::item;`
#[derive(Clone, Debug)]
pub struct UseDecl {
    /// Path segments, e.g., ["math", "add"] for `use math::add;`
    pub path: Vec<String>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    /// Generic type parameters: `fn foo<T, U>(...)` => ["T", "U"]
    pub type_params: Vec<String>,
    pub name: String,
    pub vis: Visibility,
    pub params: Vec<(String, TypeExpr)>,
    pub ret_ty: Option<TypeExpr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<(String, TypeExpr)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<(String, Option<Vec<TypeExpr>>)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub name: String,
    pub ty: TypeExpr,
    pub span: Span,
}

// ── Block ──

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

// ── Statements (n=6) ──

#[derive(Clone, Debug)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty: Option<TypeExpr>,
        init: Option<Expr>,
        span: Span,
    },
    Assign {
        target: Expr,
        value: Expr,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    If {
        cond: Expr,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Block,
        span: Span,
    },
    /// `for var in iterable { body }` — desugared to while during lowering
    ForLoop {
        var: String,
        iterable: Expr,
        body: Block,
        span: Span,
    },
    ExprStmt {
        expr: Expr,
        span: Span,
    },
}

// ── Expressions (sigma+phi=14) ──

#[derive(Clone, Debug)]
pub enum Expr {
    IntLit(i64, Span),
    FloatLit(f64, Span),
    BoolLit(bool, Span),
    StrLit(String, Span),
    Ident(String, Span),
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnOp,
        operand: Box<Expr>,
        span: Span,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Field {
        obj: Box<Expr>,
        name: String,
        span: Span,
    },
    Index {
        arr: Box<Expr>,
        idx: Box<Expr>,
        span: Span,
    },
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
        span: Span,
    },
    /// Array literal: `[1, 2, 3]`
    ArrayLit {
        elements: Vec<Expr>,
        span: Span,
    },
    /// Match expression: `match expr { Pattern => body, ... }`
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
    /// Try expression: `expr?`
    TryExpr {
        expr: Box<Expr>,
        span: Span,
    },
    /// Closure: `|x, y| expr` or `|x: i64| -> i64 { body }`
    Closure {
        params: Vec<(String, Option<TypeExpr>)>,
        ret_ty: Option<Box<TypeExpr>>,
        body: Box<Expr>,
        span: Span,
    },
    /// Generic function call: `foo::<i64>(x)` (type args erased during mono)
    GenericCall {
        func: Box<Expr>,
        type_args: Vec<TypeExpr>,
        args: Vec<Expr>,
        span: Span,
    },
    Block(Block),
}

/// A single arm of a match expression
#[derive(Clone, Debug)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
    pub span: Span,
}

/// Pattern for match arms (7 variants)
#[derive(Clone, Debug)]
pub enum Pattern {
    /// Wildcard: `_`
    Wildcard(Span),
    /// Integer literal: `42`
    Literal(i64, Span),
    /// Variable binding: `x`
    Binding(String, Span),
    /// Enum variant: `Color::Red` or `Color::RGB(pat1, pat2)`
    Variant {
        enum_name: String,
        variant_name: String,
        bindings: Vec<Pattern>,
        span: Span,
    },
    /// Struct destructure: `Point { x, y }`
    StructDestructure {
        struct_name: String,
        fields: Vec<(String, Option<Pattern>)>,
        span: Span,
    },
    /// Tuple pattern: `(pat1, pat2)`
    Tuple {
        elements: Vec<Pattern>,
        span: Span,
    },
    /// Guard pattern: `pat if cond`
    Guard {
        pattern: Box<Pattern>,
        condition: Box<Expr>,
        span: Span,
    },
}

impl Pattern {
    pub fn span(&self) -> Span {
        match self {
            Pattern::Wildcard(s) => *s,
            Pattern::Literal(_, s) => *s,
            Pattern::Binding(_, s) => *s,
            Pattern::Variant { span, .. } => *span,
            Pattern::StructDestructure { span, .. } => *span,
            Pattern::Tuple { span, .. } => *span,
            Pattern::Guard { span, .. } => *span,
        }
    }
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::IntLit(_, s) => *s,
            Expr::FloatLit(_, s) => *s,
            Expr::BoolLit(_, s) => *s,
            Expr::StrLit(_, s) => *s,
            Expr::Ident(_, s) => *s,
            Expr::Binary { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::Field { span, .. } => *span,
            Expr::Index { span, .. } => *span,
            Expr::StructInit { span, .. } => *span,
            Expr::ArrayLit { span, .. } => *span,
            Expr::Match { span, .. } => *span,
            Expr::TryExpr { span, .. } => *span,
            Expr::Closure { span, .. } => *span,
            Expr::GenericCall { span, .. } => *span,
            Expr::Block(b) => b.span,
        }
    }
}

// ── Binary operators ──

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    // Arithmetic (n=6)
    Add, Sub, Mul, Div, Mod, Pow,
    // Comparison (n=6)
    Eq, Neq, Lt, Gt, Le, Ge,
    // Logic (n/phi=3 binary)
    And, Or, BitAnd, BitOr, BitXor,
    // Range
    Range,
    RangeInclusive,
}

// ── Unary operators ──

#[derive(Clone, Debug, PartialEq)]
pub enum UnOp {
    Neg,    // -
    Not,    // !
}

// ── Type expressions (n/phi=3) ──

#[derive(Clone, Debug)]
pub enum TypeExpr {
    /// Simple named type: `i64`, `bool`, `MyStruct`
    Named(String, Span),
    /// Array type: `[i64; 6]`
    Array(Box<TypeExpr>, usize, Span),
    /// Function type: `fn(i64, bool) -> f64`
    Fn(Vec<TypeExpr>, Box<TypeExpr>, Span),
    /// Type parameter: `T` in generic context
    TypeParam(String, Span),
}

impl TypeExpr {
    pub fn span(&self) -> Span {
        match self {
            TypeExpr::Named(_, s) => *s,
            TypeExpr::Array(_, _, s) => *s,
            TypeExpr::Fn(_, _, s) => *s,
            TypeExpr::TypeParam(_, s) => *s,
        }
    }
}
