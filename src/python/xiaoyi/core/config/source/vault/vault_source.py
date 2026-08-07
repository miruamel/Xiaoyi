"""
# Vault Configuration Source

`vault_source` provides encrypted configuration loading from vault files.

Path: `xiaoyi.core.config.source.vault_source`

@module xiaoyi.core.config.source.vault_source
@brief Encrypted file-based configuration source
@group Core
@security Loads encrypted configuration. Key must be provided securely.
@since 0.1.0
@author Miruamel
@see xiaoyi.core.config.source
@see xiaoyi.core.config.source.vault
@see xiaoyi.core.config.source.vault.aes
@see xiaoyi.core.config.source.vault.decrypt
"""

import asyncio
from pathlib import Path
from typing import Callable, Dict, Any, Optional

from ...config import ConfigSource, ConfigSourceError
from .decrypt import decrypt_vault_bytes


class VaultSourceOptions:
    """
    Vault source options.

    @brief Configuration for vault source
    @group Core
    @since 0.1.0
    @security Password should be provided via secure means (env var, secret manager).
    """

    def __init__(
        self,
        path: str,
        password: str,
        priority: int = 300,
    ):
        """
        Create vault source options.

        @param path Vault file path
        @param password Encryption password
        @param priority Source priority
        @since 0.1.0
        """
        self.path = path
        self.password = password
        self.priority = priority


class VaultSource(ConfigSource):
    """
    Encrypted configuration source.

    @brief Encrypted file-based configuration source
    @group Core
    @since 0.1.0
    @security Password should be provided via secure means (env var, secret manager).
    @example
    ```python
    source = VaultSource(VaultSourceOptions(path="./config.vault", password="secret"))
    config = await source.load()
    ```
    """

    def __init__(self, options: VaultSourceOptions):
        self._path = options.path
        self._password = options.password
        self._priority = options.priority
        self._name = f"vault:{options.path}"

    @property
    def name(self) -> str:
        return self._name

    @property
    def priority(self) -> int:
        return self._priority

    async def load(self) -> Dict[str, Any]:
        """
        Load and decrypt configuration from vault file.

        @return Decrypted configuration object
        @throws ConfigSourceError If file not found or decryption fails
        @since 0.1.0
        @security Validates authentication tag. Throws on tampering.
        """
        try:
            path = Path(self._path)
            if not path.exists():
                raise ConfigSourceError(self.name, f"Vault file not found: {self._path}")

            data = path.read_bytes()
            return await decrypt_vault_bytes(data, self._password)
        except ConfigSourceError:
            raise
        except Exception as e:
            raise ConfigSourceError(self.name, str(e), e)

    def watch(self, callback: Callable[[Dict[str, Any]], None]) -> Callable[[], None]:
        """
        Watch for vault file changes.

        @param callback Change callback
        @return Unsubscribe function
        @since 0.1.0
        """
        stopped = False
        last_mtime = 0

        async def check():
            nonlocal last_mtime, stopped
            while not stopped:
                try:
                    path = Path(self._path)
                    if path.exists():
                        stat = path.stat()
                        if stat.st_mtime > last_mtime:
                            last_mtime = stat.st_mtime
                            config = await self.load()
                            callback(config)
                except Exception:
                    pass  # Ignore errors during watch
                await asyncio.sleep(1)

        task = asyncio.create_task(check())

        def unsubscribe():
            nonlocal stopped
            stopped = True
            task.cancel()

        return unsubscribe


__all__ = [
    "VaultSourceOptions",
    "VaultSource",
]