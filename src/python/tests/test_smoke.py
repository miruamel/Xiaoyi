"""
Cross-module smoke test for Xiaoyi Python package.

@package xiaoyi.tests
@brief Cross-module integration test
@since 0.1.0
"""

import pytest
import asyncio


class TestAllModulesImport:
    """Test that all modules can be imported without errors."""

    def test_core_error_imports(self):
        from xiaoyi.core.error import (
            ErrorKind,
            XiaoyiError,
            create_error,
            is_xiaoyi_error,
        )
        assert ErrorKind is not None
        assert XiaoyiError is not None
        assert create_error is not None
        assert is_xiaoyi_error is not None

    def test_core_result_imports(self):
        from xiaoyi.core.result import (
            Ok,
            Err,
            Result,
            ok,
            err,
            is_ok,
            is_err,
            unwrap,
            unwrap_err,
            map,
            map_err,
            and_then,
            or_else,
            to_awaitable,
        )
        assert Ok is not None
        assert Err is not None

    def test_core_config_imports(self):
        from xiaoyi.core.config.config import (
            Config,
            ConfigSource,
            ConfigSourceError,
            ConfigValue,
            ConfigMergeStrategy,
        )
        from xiaoyi.core.config.builder import ConfigBuilder
        assert Config is not None
        assert ConfigBuilder is not None

    def test_core_config_source_file_imports(self):
        from xiaoyi.core.config.source.file import (
            FileSource,
            FileSourceOptions,
            ConfigFormat,
        )
        assert FileSource is not None

    def test_core_config_source_env_imports(self):
        from xiaoyi.core.config.source.env import (
            EnvSource,
            EnvSourceOptions,
        )
        assert EnvSource is not None

    def test_core_config_source_vault_imports(self):
        from xiaoyi.core.config.source.vault import (
            VaultSource,
            VaultSourceOptions,
            encrypt_config,
            decrypt_config,
            derive_key,
            generate_salt,
        )
        assert VaultSource is not None

    def test_domain_token_syntax_keyword_imports(self):
        from xiaoyi.domain.token.syntax.keyword import (
            Keyword,
            KeywordKind,
            KEYWORDS,
            keyword_from_ident,
            is_keyword,
        )
        assert KEYWORDS is not None
        assert len(KEYWORDS) > 0

    def test_domain_token_syntax_operator_imports(self):
        from xiaoyi.domain.token.syntax.operator import (
            Operator,
            OperatorKind,
            Associativity,
            OPERATORS,
            operator_from_symbol,
            operators_with_prefix,
        )
        assert OPERATORS is not None
        assert len(OPERATORS) > 0

    def test_domain_token_syntax_delimiter_imports(self):
        from xiaoyi.domain.token.syntax.delimiter import (
            Delimiter,
            DelimiterKind,
            DELIMITERS,
            matching_close,
            matching_open,
            is_open_delimiter,
            is_close_delimiter,
            is_delimiter_pair,
        )
        assert DELIMITERS is not None
        assert len(DELIMITERS) > 0

    def test_domain_token_syntax_literal_imports(self):
        from xiaoyi.domain.token.syntax.literal import (
            Literal,
            LiteralKind,
            int_literal,
            float_literal,
            string_literal,
            bool_literal,
            parse_literal,
        )
        assert Literal is not None

    def test_domain_token_syntax_kinds_imports(self):
        from xiaoyi.domain.token.syntax.kinds import SyntaxKind
        assert SyntaxKind.KEYWORD == "keyword"

    def test_domain_token_primitive_string_imports(self):
        from xiaoyi.domain.token.primitive.string import (
            String,
            Str,
            new_string,
            from_string,
            is_valid_utf8,
            char_len,
        )
        assert String is str

    def test_domain_token_primitive_bool_imports(self):
        from xiaoyi.domain.token.primitive.bool import (
            Bool,
            TRUE,
            FALSE,
            bool_not,
            bool_and,
            bool_or,
        )
        assert Bool is bool
        assert TRUE is True
        assert FALSE is False

    def test_domain_token_primitive_int_imports(self):
        from xiaoyi.domain.token.primitive.int.kind import (
            IntKind,
            SIGNED,
            UNSIGNED,
        )
        from xiaoyi.domain.token.primitive.int.width import (
            IntWidth,
            W8, W16, W32, W64, W128,
        )
        from xiaoyi.domain.token.primitive.int.int_type import (
            IntType,
            create_int_type,
        )
        from xiaoyi.domain.token.primitive.int.normalize import (
            normalize_int,
            wrap_int,
            convert_int_checked,
        )
        from xiaoyi.domain.token.primitive.int.rep import (
            Endianness,
            native_endianness,
        )
        assert IntKind is not None
        assert IntWidth is not None

    def test_domain_token_primitive_float_imports(self):
        from xiaoyi.domain.token.primitive.float.f32 import (
            F32,
            F32Consts,
            is_f32_finite,
        )
        from xiaoyi.domain.token.primitive.float.f64 import (
            F64,
            F64Consts,
            is_f64_finite,
        )
        assert F32 is float
        assert F64 is float


class TestCoreFunctionality:
    """Test basic functionality across core modules."""

    def test_error_creation_and_checking(self):
        from xiaoyi.core.error import create_error, is_xiaoyi_error, ErrorKind

        error = create_error(ErrorKind.CONFIG, "Test error")
        assert is_xiaoyi_error(error) is True
        assert str(error) == "[config] Test error"

    def test_result_operations(self):
        from xiaoyi.core.result import ok, err, is_ok, is_err, unwrap, map
        from xiaoyi.core.error import XiaoyiError, ErrorKind

        # Success path
        result = ok(42)
        assert is_ok(result) is True
        assert unwrap(result) == 42
        assert unwrap(map(result, lambda x: x * 2)) == 84

        # Error path
        error = XiaoyiError(kind=ErrorKind.RUNTIME, message="Failed")
        result = err(error)
        assert is_err(result) is True
        assert unwrap_err(result) == error

    @pytest.mark.asyncio
    async def test_result_async(self):
        from xiaoyi.core.result import ok, err, to_awaitable

        result = await to_awaitable(ok(42))
        assert result.value == 42


class TestConfigIntegration:
    """Test configuration system integration."""

    @pytest.mark.asyncio
    async def test_config_builder_with_file_source(self, tmp_path):
        from xiaoyi.core.config.builder import ConfigBuilder
        from xiaoyi.core.config.source.file import FileSource, FileSourceOptions

        config_file = tmp_path / "config.toml"
        config_file.write_text('app = { name = "test" }')

        builder = ConfigBuilder()
        builder.add_source(FileSource(FileSourceOptions(path=str(config_file))))

        config = await builder.build()
        assert config.data == {"app": {"name": "test"}}

    @pytest.mark.asyncio
    async def test_config_builder_with_env_source(self, monkeypatch):
        from xiaoyi.core.config.builder import ConfigBuilder
        from xiaoyi.core.config.source.env import EnvSource, EnvSourceOptions

        monkeypatch.setenv("XIAOYI_TEST_KEY", "test_value")

        builder = ConfigBuilder()
        builder.add_source(EnvSource(EnvSourceOptions(prefix="XIAOYI_")))

        config = await builder.build()
        assert config.data.get("test_key") == "test_value"


class TestTokenSyntaxIntegration:
    """Test token syntax integration."""

    def test_keyword_operator_delimiter_interaction(self):
        from xiaoyi.domain.token.syntax.keyword import is_keyword, KEYWORDS
        from xiaoyi.domain.token.syntax.operator import operator_from_symbol, OPERATORS
        from xiaoyi.domain.token.syntax.delimiter import is_delimiter_pair, DELIMITERS

        # No overlap between keywords, operators, delimiters
        keyword_texts = {kw.text for kw in KEYWORDS}
        operator_symbols = {op.symbol for op in OPERATORS}
        delimiter_texts = {d.text for d in DELIMITERS}

        assert keyword_texts.isdisjoint(operator_symbols)
        assert keyword_texts.isdisjoint(delimiter_texts)
        assert operator_symbols.isdisjoint(delimiter_texts)

    def test_literal_parsing(self):
        from xiaoyi.domain.token.syntax.literal import parse_literal, LiteralKind

        # Integer
        lit = parse_literal("42")
        assert lit.kind == LiteralKind.INTEGER
        assert lit.value == 42

        # Float
        lit = parse_literal("3.14")
        assert lit.kind == LiteralKind.FLOAT
        assert lit.value == 3.14

        # String
        lit = parse_literal('"hello"')
        assert lit.kind == LiteralKind.STRING
        assert lit.value == "hello"

        # Boolean
        lit = parse_literal("true")
        assert lit.kind == LiteralKind.BOOLEAN
        assert lit.value is True


class TestTokenPrimitiveIntegration:
    """Test token primitive integration."""

    def test_string_operations(self):
        from xiaoyi.domain.token.primitive.string import (
            new_string, from_string, is_valid_utf8, char_len
        )

        s = new_string()
        assert s == ""

        s = from_string("hello")
        assert s == "hello"

        assert is_valid_utf8("hello".encode("utf-8")) is True
        assert char_len("hello") == 5

    def test_bool_operations(self):
        from xiaoyi.domain.token.primitive.bool import (
            TRUE, FALSE, bool_not, bool_and, bool_or
        )

        assert TRUE is True
        assert FALSE is False

        assert bool_not(True) is False
        assert bool_and(True, False) is False
        assert bool_or(True, False) is True

    def test_int_operations(self):
        from xiaoyi.domain.token.primitive.int.kind import IntKind, SIGNED, UNSIGNED
        from xiaoyi.domain.token.primitive.int.width import IntWidth, W32
        from xiaoyi.domain.token.primitive.int.int_type import create_int_type
        from xiaoyi.domain.token.primitive.int.normalize import (
            normalize_int, wrap_int, int_byte_size
        )

        int_type = create_int_type(SIGNED, W32)
        assert int_byte_size(int_type) == 4
        assert normalize_int(42, int_type) == 42

    def test_float_operations(self):
        from xiaoyi.domain.token.primitive.float.f32 import (
            F32Consts, is_f32_finite, is_f32_nan, is_f32_infinite
        )
        from xiaoyi.domain.token.primitive.float.f64 import (
            F64Consts, is_f64_finite, is_f64_nan, is_f64_infinite
        )

        assert is_f32_finite(1.0) is True
        assert is_f32_nan(float("nan")) is True
        assert is_f32_infinite(float("inf")) is True

        assert is_f64_finite(1.0) is True
        assert is_f64_nan(float("nan")) is True
        assert is_f64_infinite(float("inf")) is True


class TestVaultEncryptionIntegration:
    """Test vault encryption integration."""

    @pytest.mark.asyncio
    async def test_vault_encrypt_decrypt(self):
        from xiaoyi.core.config.source.vault.encrypt import encrypt_config, serialize_vault
        from xiaoyi.core.config.source.vault.decrypt import decrypt_vault_bytes

        config = {"secret": "value"}
        password = "test-password"

        salt, encrypted = await encrypt_config(config, password)
        vault_bytes = serialize_vault(salt, encrypted)

        decrypted = await decrypt_vault_bytes(vault_bytes, password)
        assert decrypted == config


class TestPackageStructure:
    """Test package structure and exports."""

    def test_xiaoyi_package_exists(self):
        import xiaoyi
        assert xiaoyi is not None

    def test_core_subpackage(self):
        import xiaoyi.core
        import xiaoyi.core.error
        import xiaoyi.core.result
        import xiaoyi.core.config
        import xiaoyi.core.config.source

    def test_domain_subpackage(self):
        import xiaoyi.domain
        import xiaoyi.domain.token
        import xiaoyi.domain.token.syntax
        import xiaoyi.domain.token.primitive

    def test_all_public_exports_available(self):
        # Test that main modules expose their public API
        from xiaoyi.core.error import __all__ as error_exports
        from xiaoyi.core.result import __all__ as result_exports
        from xiaoyi.core.config.config import __all__ as config_exports
        from xiaoyi.core.config.builder import __all__ as builder_exports

        assert "ErrorKind" in error_exports
        assert "XiaoyiError" in error_exports
        assert "create_error" in error_exports
        assert "is_xiaoyi_error" in error_exports

        assert "ok" in result_exports
        assert "err" in result_exports
        assert "is_ok" in result_exports
        assert "is_err" in result_exports
        assert "unwrap" in result_exports
        assert "map" in result_exports