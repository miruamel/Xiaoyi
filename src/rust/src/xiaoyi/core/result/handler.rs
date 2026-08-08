use crate::xiaoyi::core::result::Status;

/// Maps a status into a human-readable label.
///
/// @brief Convert status to display label
/// @param status Operation status
/// @return Human-readable label
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::core::result
pub fn label_for(status: Status) -> &'static str {
    match status {
        Status::Ok => "ok",
        Status::Cancelled => "cancelled",
        Status::Unknown => "unknown",
        Status::InvalidArgument => "invalid_argument",
        Status::DeadlineExceeded => "deadline_exceeded",
        Status::NotFound => "not_found",
        Status::AlreadyExists => "already_exists",
        Status::PermissionDenied => "permission_denied",
        Status::ResourceExhausted => "resource_exhausted",
        Status::FailedPrecondition => "failed_precondition",
        Status::Aborted => "aborted",
        Status::OutOfRange => "out_of_range",
        Status::Unimplemented => "unimplemented",
        Status::Internal => "internal",
        Status::Unavailable => "unavailable",
        Status::DataLoss => "data_loss",
        Status::Unauthenticated => "unauthenticated",
    }
}
