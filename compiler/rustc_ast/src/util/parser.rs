use rustc_span::kw;

use crate::ast::{self, BinOpKind, RangeLimits};
use crate::token::{self, BinOpToken, Token};

/// Associative operator.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AssocOp {
    /// A binary op.
    Binary(BinOpKind),
    /// `?=` where ? is one of the assignable BinOps
    AssignOp(BinOpKind),
    /// `=`
    Assign,
    /// `as`
    As,
    /// `..` or `..=` range
    Range(RangeLimits),
}

#[derive(PartialEq, Debug)]
pub enum Fixity {
    /// The operator is left-associative
    Left,
    /// The operator is right-associative
    Right,
    /// The operator is not associative
    None,
}

impl AssocOp {
    /// Creates a new AssocOp from a token.
    pub fn from_token(t: &Token) -> Option<AssocOp> {
        use AssocOp::*;
        match t.kind {
            token::Eq => Some(Assign),
            token::BinOp(BinOpToken::Plus) => Some(Binary(BinOpKind::Add)),
            token::BinOp(BinOpToken::Minus) => Some(Binary(BinOpKind::Sub)),
            token::BinOp(BinOpToken::Star) => Some(Binary(BinOpKind::Mul)),
            token::BinOp(BinOpToken::Slash) => Some(Binary(BinOpKind::Div)),
            token::BinOp(BinOpToken::Percent) => Some(Binary(BinOpKind::Rem)),
            token::BinOp(BinOpToken::Caret) => Some(Binary(BinOpKind::BitXor)),
            token::BinOp(BinOpToken::And) => Some(Binary(BinOpKind::BitAnd)),
            token::BinOp(BinOpToken::Or) => Some(Binary(BinOpKind::BitOr)),
            token::BinOp(BinOpToken::Shl) => Some(Binary(BinOpKind::Shl)),
            token::BinOp(BinOpToken::Shr) => Some(Binary(BinOpKind::Shr)),
            token::BinOpEq(BinOpToken::Plus) => Some(AssignOp(BinOpKind::Add)),
            token::BinOpEq(BinOpToken::Minus) => Some(AssignOp(BinOpKind::Sub)),
            token::BinOpEq(BinOpToken::Star) => Some(AssignOp(BinOpKind::Mul)),
            token::BinOpEq(BinOpToken::Slash) => Some(AssignOp(BinOpKind::Div)),
            token::BinOpEq(BinOpToken::Percent) => Some(AssignOp(BinOpKind::Rem)),
            token::BinOpEq(BinOpToken::Caret) => Some(AssignOp(BinOpKind::BitXor)),
            token::BinOpEq(BinOpToken::And) => Some(AssignOp(BinOpKind::BitAnd)),
            token::BinOpEq(BinOpToken::Or) => Some(AssignOp(BinOpKind::BitOr)),
            token::BinOpEq(BinOpToken::Shl) => Some(AssignOp(BinOpKind::Shl)),
            token::BinOpEq(BinOpToken::Shr) => Some(AssignOp(BinOpKind::Shr)),
            token::Lt => Some(Binary(BinOpKind::Lt)),
            token::Le => Some(Binary(BinOpKind::Le)),
            token::Ge => Some(Binary(BinOpKind::Ge)),
            token::Gt => Some(Binary(BinOpKind::Gt)),
            token::EqEq => Some(Binary(BinOpKind::Eq)),
            token::Ne => Some(Binary(BinOpKind::Ne)),
            token::AndAnd => Some(Binary(BinOpKind::And)),
            token::OrOr => Some(Binary(BinOpKind::Or)),
            token::DotDot => Some(Range(RangeLimits::HalfOpen)),
            // DotDotDot is no longer supported, but we need some way to display the error
            token::DotDotEq | token::DotDotDot => Some(Range(RangeLimits::Closed)),
            // `<-` should probably be `< -`
            token::LArrow => Some(Binary(BinOpKind::Lt)),
            _ if t.is_keyword(kw::As) => Some(As),
            _ => None,
        }
    }

    /// Gets the precedence of this operator
    pub fn precedence(&self) -> ExprPrecedence {
        use AssocOp::*;
        match *self {
            As => ExprPrecedence::Cast,
            Binary(bin_op) => bin_op.precedence(),
            Range(_) => ExprPrecedence::Range,
            Assign | AssignOp(_) => ExprPrecedence::Assign,
        }
    }

    /// Gets the fixity of this operator
    pub fn fixity(&self) -> Fixity {
        use AssocOp::*;
        // NOTE: it is a bug to have an operators that has same precedence but different fixities!
        match *self {
            Assign | AssignOp(_) => Fixity::Right,
            Binary(binop) => binop.fixity(),
            As => Fixity::Left,
            Range(_) => Fixity::None,
        }
    }

    pub fn is_comparison(&self) -> bool {
        use AssocOp::*;
        match *self {
            Binary(binop) => binop.is_comparison(),
            Assign | AssignOp(_) | As | Range(_) => false,
        }
    }

    pub fn is_assign_like(&self) -> bool {
        use AssocOp::*;
        match *self {
            Assign | AssignOp(_) => true,
            As | Binary(_) | Range(_) => false,
        }
    }

    /// This operator could be used to follow a block unambiguously.
    ///
    /// This is used for error recovery at the moment, providing a suggestion to wrap blocks with
    /// parentheses while having a high degree of confidence on the correctness of the suggestion.
    pub fn can_continue_expr_unambiguously(&self) -> bool {
        use AssocOp::*;
        use BinOpKind::*;
        matches!(
            self,
            Assign | // `{ 42 } = { 42 }`
            Binary(
                BitXor | // `{ 42 } ^ 3`
                Div | // `{ 42 } / 42`
                Rem | // `{ 42 } % 2`
                Shr | // `{ 42 } >> 2`
                Le | // `{ 42 } <= 3`
                Gt | // `{ 42 } > 3`
                Ge   // `{ 42 } >= 3`
            ) |
            AssignOp(_) | // `{ 42 } +=`
            // Equal | // `{ 42 } == { 42 }`    Accepting these here would regress incorrect
            // NotEqual | // `{ 42 } != { 42 }  struct literals parser recovery.
            As // `{ 42 } as usize`
        )
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ExprPrecedence {
    // return, break, yield, closures
    Jump,
    // = += -= *= /= %= &= |= ^= <<= >>=
    Assign,
    // .. ..=
    Range,
    // ||
    LOr,
    // &&
    LAnd,
    // == != < > <= >=
    Compare,
    // |
    BitOr,
    // ^
    BitXor,
    // &
    BitAnd,
    // << >>
    Shift,
    // + -
    Sum,
    // * / %
    Product,
    // as
    Cast,
    // unary - * ! & &mut
    Prefix,
    // paths, loops, function calls, array indexing, field expressions, method calls
    Unambiguous,
}

/// In `let p = e`, operators with precedence `<=` this one requires parentheses in `e`.
pub fn prec_let_scrutinee_needs_par() -> ExprPrecedence {
    ExprPrecedence::LAnd
}

/// Suppose we have `let _ = e` and the `order` of `e`.
/// Is the `order` such that `e` in `let _ = e` needs parentheses when it is on the RHS?
///
/// Conversely, suppose that we have `(let _ = a) OP b` and `order` is that of `OP`.
/// Can we print this as `let _ = a OP b`?
pub fn needs_par_as_let_scrutinee(order: ExprPrecedence) -> bool {
    order <= prec_let_scrutinee_needs_par()
}

/// Expressions that syntactically contain an "exterior" struct literal i.e., not surrounded by any
/// parens or other delimiters, e.g., `X { y: 1 }`, `X { y: 1 }.method()`, `foo == X { y: 1 }` and
/// `X { y: 1 } == foo` all do, but `(X { y: 1 }) == foo` does not.
pub fn contains_exterior_struct_lit(value: &ast::Expr) -> bool {
    match &value.kind {
        ast::ExprKind::Struct(..) => true,

        ast::ExprKind::Assign(lhs, rhs, _)
        | ast::ExprKind::AssignOp(_, lhs, rhs)
        | ast::ExprKind::Binary(_, lhs, rhs) => {
            // X { y: 1 } + X { y: 2 }
            contains_exterior_struct_lit(lhs) || contains_exterior_struct_lit(rhs)
        }
        ast::ExprKind::Await(x, _)
        | ast::ExprKind::Unary(_, x)
        | ast::ExprKind::Cast(x, _)
        | ast::ExprKind::Type(x, _)
        | ast::ExprKind::Field(x, _)
        | ast::ExprKind::Index(x, _, _)
        | ast::ExprKind::Match(x, _, ast::MatchKind::Postfix) => {
            // &X { y: 1 }, X { y: 1 }.y
            contains_exterior_struct_lit(x)
        }

        ast::ExprKind::MethodCall(box ast::MethodCall { receiver, .. }) => {
            // X { y: 1 }.bar(...)
            contains_exterior_struct_lit(receiver)
        }

        _ => false,
    }
}
