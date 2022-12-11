use std::fmt::Debug;

use crate::diagnostic::Loc;

/// All possible types used by parser.
///
/// `Set`, `List` and `Dict` are three special classes that should be implemented by code generator
/// backend. Specially, `Any` means any type except `Nil` is possible.
pub enum _Type {
    Any,
    Float,
    Int,
    Str,
    Class(String),
    Function,
    Nil,
}

pub enum Stat_ {
    Expr(Expr),
    Continue,
    Break,
    Return(Option<Expr>),
    Class(String, Vec<(String, Loc)>, Vec<Expr>),
    /// An optional break condition & a body
    Loop(Option<Expr>, Vec<Stat>),
    /// variables, iterator, statements
    For(Box<Expr>, Box<Expr>, Vec<Stat>),
    Error,
}

pub struct Stat {
    pub loc: Loc,
    pub val: Stat_,
}

impl Debug for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Stat_::*;
        match &self.val {
            Expr(expr) => f.debug_tuple("").field(&expr).finish(),
            Continue => f.debug_tuple("continue").finish(),
            Break => f.debug_tuple("break").finish(),
            Return(expr) => {
                if let Some(expr) = expr {
                    f.debug_tuple("return").field(&expr).finish()
                } else {
                    f.debug_tuple("return").finish()
                }
            }
            Error => f.debug_tuple("error").finish(),
            Class(name, fields, methods) => f
                .debug_tuple("class")
                .field(name)
                .field(&fields.iter().map(|x| &x.0).collect::<Vec<&String>>())
                .field(methods)
                .finish(),
            Loop(cond, body) => f.debug_tuple("loop").field(cond).field(body).finish(),
            For(vars, iter, expr) => f
                .debug_tuple("for")
                .field(vars)
                .field(&"in")
                .field(iter)
                .field(&"do")
                .field(expr)
                .finish(),
        }
    }
}

impl Stat {
    pub fn new(val: Stat_, loc: Loc) -> Self {
        Self { loc, val }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OpInfix {
    Assign,
    Range,
    Or,
    And,
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    Plus,
    Minus,
    Mul,
    Div,
    DivFloor,
    Mod,
    Exp,
    Comma,
    Member,
}

#[derive(Clone, Copy, Debug)]
pub enum OpPrefix {
    Not,
    Neg,
}

#[derive(Clone, Copy, Debug)]
pub enum OpPostfix {
    Index,
    Call,
}

#[derive(Debug)]
pub enum Expr_ {
    Block(Vec<Stat>),
    /// An `if..then..elsif..then..else..end`
    /// Expression is in order
    If(Vec<Expr>),
    Prefix(OpPrefix, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Infix(OpInfix, Box<Expr>, Box<Expr>),
    /// Define a function
    ///
    /// First expression is declaration(None for no parameters), second is function body
    /// If its name is None, then this is a lambda expression
    Def(Option<String>, Vec<String>, Vec<Stat>, Vec<(String, Expr)>),
    Id(String),
    Parentheses(Box<Expr>),
    Const(Const),
    Error,
}

pub struct Expr {
    pub loc: Loc,
    pub val: Expr_,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.val {
            Expr_::Block(b) => f.debug_list().entries(b.iter()).finish(),
            Expr_::Prefix(op, expr) => f.debug_tuple("").field(&op).field(&expr).finish(),
            Expr_::Infix(op, e1, e2) => f.debug_tuple("").field(&e1).field(&op).field(&e2).finish(),
            Expr_::Id(id) => write!(f, "{:?}", id),
            Expr_::Const(c) => write!(f, "{:?}", c),
            Expr_::Error => write!(f, "Error"),
            Expr_::If(v) => f.debug_tuple("").field(&"if").field(&v).finish(),
            Expr_::Def(name, decl, body, binds) => {
                let mut f = f.debug_tuple("def");
                if let Some(name) = name {
                    f.field(name).field(decl).field(body).field(binds).finish()
                } else {
                    f.field(decl).field(body).field(binds).finish()
                }
            }
            Expr_::Parentheses(expr) => f
                .debug_tuple("")
                .field(&"(")
                .field(expr)
                .field(&")")
                .finish(),
            Expr_::Call(expr, call) => f.debug_tuple("Call").field(expr).field(call).finish(),
            Expr_::Index(expr, index) => f.debug_tuple("Call").field(expr).field(index).finish(),
        }
    }
}

pub enum Const {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    // keys-values
    Dict(Vec<Expr>, Vec<Expr>),
    Nil,
}

impl Debug for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::Int(i) => write!(f, "{}", i),
            Const::Float(fp) => write!(f, "{}", fp),
            Const::Str(s) => write!(f, "{}", s),
            Const::Bool(b) => write!(f, "{}", b),
            Const::List(l) => f.debug_list().entries(l.iter()).finish(),
            Const::Nil => write!(f, "nil"),
            Const::Set(val) => f.debug_set().entries(val).finish(),
            Const::Dict(keys, vals) => f.debug_map().entries(keys.iter().zip(vals.iter())).finish(),
        }
    }
}

#[derive(Default)]
pub struct Ast {
    pub statements: Vec<Stat>,
}
