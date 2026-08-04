# Layer 1 - Domain / Token Primitive Int Kind
# Path: xiaoyi.domain.token.primitive.int.kind
#
# Layer hierarchy:
# - 1: domain
# - 2: token
# - 3: primitive
# - 4: int
# - 5: kind

from enum import StrEnum


class IntKind(StrEnum):
    """Signedness classification for integer token primitives."""

    SIGNED = "signed"
    UNSIGNED = "unsigned"
