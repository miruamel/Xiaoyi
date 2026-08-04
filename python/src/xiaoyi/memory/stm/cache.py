"""Memory STM (Short-Term Memory) cache.

Path: xiaoyi.memory.stm.cache

Layer hierarchy:
- 0: memory
- 1: stm
- 2: cache/buffer/sliding/recent
- 3: entry/eviction/policy

Fast in-memory cache with TTL and LRU eviction for recent context.
"""

from __future__ import annotations

import time
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Any, Generic, Optional, TypeVar

K = TypeVar("K")
V = TypeVar("V")


@dataclass
class CacheEntry(Generic[V]):
    value: V
    created_at: float = field(default_factory=time.time)
    expires_at: Optional[float] = None
    access_count: int = 0

    def is_expired(self) -> bool:
        if self.expires_at is None:
            return False
        return time.time() > self.expires_at


class StmCache(Generic[K, V]):
    """Fast in-memory cache with TTL and LRU eviction."""

    def __init__(self, max_size: int, default_ttl: Optional[float] = None):
        self._cache: OrderedDict[K, CacheEntry[V]] = OrderedDict()
        self._max_size = max_size
        self._default_ttl = default_ttl

    def get(self, key: K) -> Optional[V]:
        if key not in self._cache:
            return None
        entry = self._cache[key]
        if entry.is_expired():
            del self._cache[key]
            return None
        entry.access_count += 1
        # Move to end (most recently used)
        self._cache.move_to_end(key)
        return entry.value

    def insert(self, key: K, value: V) -> None:
        if key in self._cache:
            del self._cache[key]
        elif len(self._cache) >= self._max_size:
            # Evict LRU
            self._cache.popitem(last=False)

        expires_at = None
        if self._default_ttl is not None:
            expires_at = time.time() + self._default_ttl

        self._cache[key] = CacheEntry(value=value, expires_at=expires_at)

    def __len__(self) -> int:
        return len(self._cache)

    def clear(self) -> None:
        self._cache.clear()