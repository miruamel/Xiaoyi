//! # Keywords
//!
//! `keyword` defines all reserved keywords in the Xiaoyi language.
//!
//! Path: `xiaoyi::domain::token::syntax::keyword`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `syntax`
//! - Layer 3: `keyword`
//!
//! @module domain::token::syntax::keyword
//! @brief Language reserved keywords
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::syntax
//! @see crate::domain::token::syntax::operator

use crate::xiaoyi::domain::token::syntax::SyntaxKind;

/// Keyword token.
///
/// @brief Reserved keyword with text
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyword {
    /// Keyword text.
    pub text: &'static str,
    /// Keyword kind.
    pub kind: KeywordKind,
}

/// Keyword categories.
///
/// @brief Keyword classification
/// @group Domain
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordKind {
    /// Control flow (if, else, while, for, return)
    ControlFlow,
    /// Declaration (let, const, fn, struct, enum)
    Declaration,
    /// Type (int, float, bool, string)
    Type,
    /// Module (mod, use, pub)
    Module,
    /// Async (async, await, spawn)
    Async,
    /// Error handling (try, catch, throw)
    ErrorHandling,
}

/// All keywords.
///
/// @brief Keyword lookup table
/// @group Domain
/// @since 0.1.0
pub const KEYWORDS: &[Keyword] = &[
    Keyword {
        text: "if",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "else",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "while",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "for",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "return",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "break",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "continue",
        kind: KeywordKind::ControlFlow,
    },
    Keyword {
        text: "let",
        kind: KeywordKind::Declaration,
    },
    Keyword {
        text: "const",
        kind: KeywordKind::Declaration,
    },
    Keyword {
        text: "fn",
        kind: KeywordKind::Declaration,
    },
    Keyword {
        text: "struct",
        kind: KeywordKind::Declaration,
    },
    Keyword {
        text: "enum",
        kind: KeywordKind::Declaration,
    },
    Keyword {
        text: "int",
        kind: KeywordKind::Type,
    },
    Keyword {
        text: "float",
        kind: KeywordKind::Type,
    },
    Keyword {
        text: "bool",
        kind: KeywordKind::Type,
    },
    Keyword {
        text: "string",
        kind: KeywordKind::Type,
    },
    Keyword {
        text: "mod",
        kind: KeywordKind::Module,
    },
    Keyword {
        text: "use",
        kind: KeywordKind::Module,
    },
    Keyword {
        text: "pub",
        kind: KeywordKind::Module,
    },
    Keyword {
        text: "async",
        kind: KeywordKind::Async,
    },
    Keyword {
        text: "await",
        kind: KeywordKind::Async,
    },
    Keyword {
        text: "spawn",
        kind: KeywordKind::Async,
    },
    Keyword {
        text: "try",
        kind: KeywordKind::ErrorHandling,
    },
    Keyword {
        text: "catch",
        kind: KeywordKind::ErrorHandling,
    },
    Keyword {
        text: "throw",
        kind: KeywordKind::ErrorHandling,
    },
];

/// Check if identifier is a keyword.
///
/// @param ident Identifier string
/// @return Some(Keyword) if keyword, None otherwise
/// @since 0.1.0
pub fn from_ident(ident: &str) -> Option<&'static Keyword> {
    KEYWORDS.iter().find(|k| k.text == ident)
}

/// Check if string is a keyword.
///
/// @param s String to check
/// @return true if keyword
/// @since 0.1.0
pub fn is_keyword(s: &str) -> bool {
    from_ident(s).is_some()
}
