"""Configuration file source with path normalization.

Path: xiaoyi.core.config.source.file

Layer hierarchy:
- 0: core
- 1: config
- 2: source
- 3: file
- 4: path/absolute/unix/norm
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

try:
    import tomllib
except ImportError:
    import tomli as tomllib

try:
    import yaml
except ImportError:
    yaml = None

from xiaoyi.core.error import XiaoyiError, ErrorKind


class FileSource:
    """File-based configuration source."""

    def __init__(self, path: str | Path, required: bool = True):
        self.path = Path(path)
        self.required = required

    def optional(self) -> FileSource:
        self.required = False
        return self

    def load(self) -> dict[str, Any]:
        """Load and parse configuration file (TOML, JSON, YAML)."""
        if not self.path.exists():
            if self.required:
                raise XiaoyiError(ErrorKind.CONFIG, "config file not found").with_meta("path", str(self.path))
            return {}

        content = self.path.read_text(encoding="utf-8")
        ext = self.path.suffix.lower().lstrip(".")

        if ext == "toml":
            return tomllib.loads(content)
        elif ext == "json":
            return json.loads(content)
        elif ext in ("yaml", "yml"):
            if yaml is None:
                raise XiaoyiError(ErrorKind.CONFIG, "PyYAML not installed").with_meta("path", str(self.path))
            return yaml.safe_load(content) or {}
        else:
            raise XiaoyiError(ErrorKind.CONFIG, "unsupported config file format").with_meta("path", str(self.path)).with_meta("extension", ext)


# Path normalization utilities
def normalize(path: Path) -> Path:
    """Normalize a path (resolve . and .., remove redundant separators)."""
    parts = []
    for part in path.parts:
        if part == "..":
            if parts and parts[-1] != "..":
                parts.pop()
            else:
                parts.append(part)
        elif part == ".":
            continue
        else:
            parts.append(part)
    return Path(*parts) if parts else Path(".")


def absolute(path: Path) -> Path:
    """Convert to absolute path."""
    return path if path.is_absolute() else Path.cwd() / path


def unix(path: Path) -> str:
    """Convert to Unix-style path (forward slashes)."""
    return str(path).replace("\\", "/")