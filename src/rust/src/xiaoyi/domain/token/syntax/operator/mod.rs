//! # Operators
//!
//! `operator` defines all operators with precedence and associativity.
//!
//! Path: `xiaoyi::domain::token::syntax::operator`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `syntax`
//! - Layer 3: `operator`
//!
//! @module domain::token::syntax::operator
//! @brief Operators with precedence
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::syntax
//! @see crate::domain::token::syntax::keyword

use crate::xiaoyi::domain::token::syntax::SyntaxKind;

/// Operator token.
///
/// @brief Operator with precedence and associativity
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operator {
    /// Operator symbol.
    pub symbol: &'static str,
    /// Operator kind.
    pub kind: OperatorKind,
    /// Precedence (higher = tighter binding).
    pub precedence: u8,
    /// Associativity.
    pub associativity: Associativity,
}

/// Operator categories.
///
/// @brief Operator classification
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    /// Arithmetic (+, -, *, /, %)
    Arithmetic,
    /// Comparison (==, !=, <, >, <=, >=)
    Comparison,
    /// Logical (&&, ||, !)
    Logical,
    /// Bitwise (&, |, ^, ~, <<, >>)
    Bitwise,
    /// Assignment (=, +=, -=, *=, /=, %=)
    Assignment,
    /// Member access (., .., ?.)
    MemberAccess,
    /// Call/Index ((), [])
    CallIndex,
}

/// Associativity.
///
/// @brief Left-to-right or right-to-left
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Associativity {
    /// Left associative (a + b + c = (a + b) + c)
    Left,
    /// Right associative (a = b = c = a = (b = c))
    Right,
    /// Non-associative (a < b < c is invalid)
    None,
}

/// All operators ordered by precedence (highest first).
///
/// @brief Operator precedence table
/// @group Domain
/// @since 0.1.0
pub const OPERATORS: &[Operator] = &[
    // Member access / call / index (highest precedence)
    Operator {
        symbol: ".",
        kind: OperatorKind::MemberAccess,
        precedence: 15,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "..",
        kind: OperatorKind::MemberAccess,
        precedence: 15,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "?.",
        kind: OperatorKind::MemberAccess,
        precedence: 15,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "()",
        kind: OperatorKind::CallIndex,
        precedence: 15,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "[]",
        kind: OperatorKind::CallIndex,
        precedence: 15,
        associativity: Associativity::Left,
    },
    // Unary
    Operator {
        symbol: "-",
        kind: OperatorKind::Arithmetic,
        precedence: 14,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "!",
        kind: OperatorKind::Logical,
        precedence: 14,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "~",
        kind: OperatorKind::Bitwise,
        precedence: 14,
        associativity: Associativity::Right,
    },
    // Multiplicative
    Operator {
        symbol: "*",
        kind: OperatorKind::Arithmetic,
        precedence: 13,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "/",
        kind: OperatorKind::Arithmetic,
        precedence: 13,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "%",
        kind: OperatorKind::Arithmetic,
        precedence: 13,
        associativity: Associativity::Left,
    },
    // Additive
    Operator {
        symbol: "+",
        kind: OperatorKind::Arithmetic,
        precedence: 12,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "-",
        kind: OperatorKind::Arithmetic,
        precedence: 12,
        associativity: Associativity::Left,
    },
    // Shift
    Operator {
        symbol: "<<",
        kind: OperatorKind::Bitwise,
        precedence: 11,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: ">>",
        kind: OperatorKind::Bitwise,
        precedence: 11,
        associativity: Associativity::Left,
    },
    // Comparison
    Operator {
        symbol: "<",
        kind: OperatorKind::Comparison,
        precedence: 10,
        associativity: Associativity::None,
    },
    Operator {
        symbol: ">",
        kind: OperatorKind::Comparison,
        precedence: 10,
        associativity: Associativity::None,
    },
    Operator {
        symbol: "<=",
        kind: OperatorKind::Comparison,
        precedence: 10,
        associativity: Associativity::None,
    },
    Operator {
        symbol: ">=",
        kind: OperatorKind::Comparison,
        precedence: 10,
        associativity: Associativity::None,
    },
    // Equality
    Operator {
        symbol: "==",
        kind: OperatorKind::Comparison,
        precedence: 9,
        associativity: Associativity::Left,
    },
    Operator {
        symbol: "!=",
        kind: OperatorKind::Comparison,
        precedence: 9,
        associativity: Associativity::Left,
    },
    // Bitwise AND
    Operator {
        symbol: "&",
        kind: OperatorKind::Bitwise,
        precedence: 8,
        associativity: Associativity::Left,
    },
    // Bitwise XOR
    Operator {
        symbol: "^",
        kind: OperatorKind::Bitwise,
        precedence: 7,
        associativity: Associativity::Left,
    },
    // Bitwise OR
    Operator {
        symbol: "|",
        kind: OperatorKind::Bitwise,
        precedence: 6,
        associativity: Associativity::Left,
    },
    // Logical AND
    Operator {
        symbol: "&&",
        kind: OperatorKind::Logical,
        precedence: 5,
        associativity: Associativity::Left,
    },
    // Logical OR
    Operator {
        symbol: "||",
        kind: OperatorKind::Logical,
        precedence: 4,
        associativity: Associativity::Left,
    },
    // Assignment (lowest, right-associative)
    Operator {
        symbol: "=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "+=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "-=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "*=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "/=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
    Operator {
        symbol: "%=",
        kind: OperatorKind::Assignment,
        precedence: 3,
        associativity: Associativity::Right,
    },
];

/// Find operator by symbol.
///
/// @param sym Operator symbol
/// @return Some(Operator) if found
/// @since 0.1.0
pub fn from_symbol(sym: &str) -> Option<&'static Operator> {
    OPERATORS.iter().find(|op| op.symbol == sym)
}

/// Get all operators starting with prefix.
///
/// @param prefix Prefix string
/// @return Matching operators
/// @since 0.1.0
pub fn with_prefix(prefix: &str) -> Vec<&'static Operator> {
    OPERATORS
        .iter()
        .filter(|op| op.symbol.starts_with(prefix))
        .collect()
}
