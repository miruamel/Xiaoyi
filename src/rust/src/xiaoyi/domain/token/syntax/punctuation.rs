use crate::xiaoyi::domain::token::syntax::SyntaxKind;

/// Returns whether a syntax kind is punctuation.
///
/// @brief Check if syntax kind is punctuation
/// @param kind Syntax kind
/// @return True if punctuation
/// @since 0.1.0
/// @author Miruamel
/// @see SyntaxKind
pub fn is_punctuation(kind: SyntaxKind) -> bool {
    matches!(kind, SyntaxKind::Delimiter)
}
