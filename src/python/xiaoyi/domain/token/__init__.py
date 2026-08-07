"""
# Domain Tokens

`token` provides the core token representation with primitive types
and syntax-level tokens.

Path: `xiaoyi.domain.token`

- Layer 0: `domain`
- Layer 1: `token` — token representation.
- Layer 2: `primitive` — primitive type definitions.
- Layer 3: `syntax` — syntax tokens.
- Layer 4-5: kind/width/rep/normalize.

@package xiaoyi.domain.token
@brief Core token representation with primitives
@group Domain
@since 0.1.0
@author Miruamel
@see xiaoyi.domain
@see xiaoyi.domain.token.primitive
@see xiaoyi.domain.token.syntax
"""

from .primitive import *
from .syntax import *

__all__ = [
    "primitive",
    "syntax",
]