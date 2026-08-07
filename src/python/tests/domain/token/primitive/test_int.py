"""
Test suite for xiaoyi.domain.token.primitive.int module.

@package xiaoyi.tests.domain.token.primitive
@brief Tests for IntKind, IntWidth, IntType, create_int_type, normalize_int, wrap_int, convert_int_checked
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.primitive.int.kind import (
    IntKind,
    SIGNED,
    UNSIGNED,
    default_int_type,
)
from xiaoyi.domain.token.primitive.int.width import (
    IntWidth,
    W8, W16, W32, W64, W128,
    default_width,
)
from xiaoyi.domain.token.primitive.int.int_type import (
    IntType,
    create_int_type,
)
from xiaoyi.domain.token.primitive.int.normalize import (
    int_byte_size,
    is_int_signed,
    normalize_int,
    wrap_int,
    convert_int_checked,
)
from xiaoyi.domain.token.primitive.int.rep import (
    Endianness,
    native_endianness,
    DEFAULT_REP,
)


class TestIntKind:
    """Tests for IntKind enum."""

    def test_int_kind_values(self):
        """Test IntKind values."""
        assert IntKind.SIGNED == "signed"
        assert IntKind.UNSIGNED == "unsigned"

    def test_signed_unsigned_constants(self):
        """Test SIGNED and UNSIGNED constants."""
        assert SIGNED == IntKind.SIGNED
        assert UNSIGNED == IntKind.UNSIGNED

    def test_int_kind_iteration(self):
        """Test IntKind iteration."""
        kinds = list(IntKind)
        assert len(kinds) == 2


class TestIntWidth:
    """Tests for IntWidth enum."""

    def test_int_width_values(self):
        """Test IntWidth values."""
        assert IntWidth.W8 == 8
        assert IntWidth.W16 == 16
        assert IntWidth.W32 == 32
        assert IntWidth.W64 == 64
        assert IntWidth.W128 == 128

    def test_width_constants(self):
        """Test width constants."""
        assert W8 == IntWidth.W8
        assert W16 == IntWidth.W16
        assert W32 == IntWidth.W32
        assert W64 == IntWidth.W64
        assert W128 == IntWidth.W128

    def test_int_width_iteration(self):
        """Test IntWidth iteration."""
        widths = list(IntWidth)
        assert len(widths) == 5

    def test_default_width(self):
        """Test default_width function."""
        assert default_width() == IntWidth.W64


class TestIntType:
    """Tests for IntType class."""

    def test_int_type_creation(self):
        """Test creating IntType."""
        int_type = IntType(kind=IntKind.SIGNED, width=IntWidth.W32)
        assert int_type.kind == IntKind.SIGNED
        assert int_type.width == IntWidth.W32

    def test_create_int_type(self):
        """Test create_int_type function."""
        int_type = create_int_type(IntKind.UNSIGNED, IntWidth.W64)
        assert int_type.kind == IntKind.UNSIGNED
        assert int_type.width == IntWidth.W64

    def test_default_int_type(self):
        """Test default_int_type function."""
        int_type = default_int_type()
        assert int_type.kind == IntKind.SIGNED
        assert int_type.width == IntWidth.W64


class TestIntByteSize:
    """Tests for int_byte_size function."""

    def test_int_byte_size_w8(self):
        """Test byte size for 8-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        assert int_byte_size(int_type) == 1

    def test_int_byte_size_w16(self):
        """Test byte size for 16-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W16)
        assert int_byte_size(int_type) == 2

    def test_int_byte_size_w32(self):
        """Test byte size for 32-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        assert int_byte_size(int_type) == 4

    def test_int_byte_size_w64(self):
        """Test byte size for 64-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W64)
        assert int_byte_size(int_type) == 8

    def test_int_byte_size_w128(self):
        """Test byte size for 128-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W128)
        assert int_byte_size(int_type) == 16


class TestIsIntSigned:
    """Tests for is_int_signed function."""

    def test_is_int_signed_true(self):
        """Test signed type returns True."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        assert is_int_signed(int_type) is True

    def test_is_int_signed_false(self):
        """Test unsigned type returns False."""
        int_type = create_int_type(IntKind.UNSIGNED, IntWidth.W32)
        assert is_int_signed(int_type) is False


class TestNormalizeInt:
    """Tests for normalize_int function."""

    def test_normalize_int_signed_w8(self):
        """Test normalizing signed 8-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # Range: -128 to 127
        assert normalize_int(0, int_type) == 0
        assert normalize_int(127, int_type) == 127
        assert normalize_int(-128, int_type) == -128

    def test_normalize_int_unsigned_w8(self):
        """Test normalizing unsigned 8-bit."""
        int_type = create_int_type(IntKind.UNSIGNED, IntWidth.W8)
        # Range: 0 to 255
        assert normalize_int(0, int_type) == 0
        assert normalize_int(255, int_type) == 255

    def test_normalize_int_signed_w32(self):
        """Test normalizing signed 32-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        max_val = 2**31 - 1
        min_val = -2**31
        assert normalize_int(max_val, int_type) == max_val
        assert normalize_int(min_val, int_type) == min_val


class TestWrapInt:
    """Tests for wrap_int function."""

    def test_wrap_int_signed_w8(self):
        """Test wrapping signed 8-bit."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 128 wraps to -128
        assert wrap_int(128, int_type) == -128
        # 255 wraps to -1
        assert wrap_int(255, int_type) == -1
        # -129 wraps to 127
        assert wrap_int(-129, int_type) == 127

    def test_wrap_int_unsigned_w8(self):
        """Test wrapping unsigned 8-bit."""
        int_type = create_int_type(IntKind.UNSIGNED, IntWidth.W8)
        # 256 wraps to 0
        assert wrap_int(256, int_type) == 0
        # 257 wraps to 1
        assert wrap_int(257, int_type) == 1
        # -1 wraps to 255
        assert wrap_int(-1, int_type) == 255

    def test_wrap_int_preserves_in_range(self):
        """Test wrapping preserves values in range."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        for i in range(-128, 128):
            assert wrap_int(i, int_type) == i


class TestConvertIntChecked:
    """Tests for convert_int_checked function."""

    def test_convert_same_type(self):
        """Test converting between same type."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        assert convert_int_checked(42, int_type, int_type) == 42

    def test_convert_signed_to_unsigned_in_range(self):
        """Test converting signed to unsigned when in range."""
        from_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        to_type = create_int_type(IntKind.UNSIGNED, IntWidth.W8)
        # 42 is in range for both
        assert convert_int_checked(42, from_type, to_type) == 42

    def test_convert_unsigned_to_signed_in_range(self):
        """Test converting unsigned to signed when in range."""
        from_type = create_int_type(IntKind.UNSIGNED, IntWidth.W8)
        to_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 42 is in range for both
        assert convert_int_checked(42, from_type, to_type) == 42

    def test_convert_wider_to_narrower_in_range(self):
        """Test converting wider to narrower when in range."""
        from_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        to_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 42 fits in both
        assert convert_int_checked(42, from_type, to_type) == 42

    def test_convert_narrower_to_wider(self):
        """Test converting narrower to wider."""
        from_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        to_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        assert convert_int_checked(42, from_type, to_type) == 42

    def test_convert_out_of_range_raises(self):
        """Test converting out of range raises error."""
        from_type = create_int_type(IntKind.SIGNED, IntWidth.W32)
        to_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 300 doesn't fit in signed 8-bit (-128 to 127)
        with pytest.raises(ValueError, match="Value out of range"):
            convert_int_checked(300, from_type, to_type)

    def test_convert_unsigned_to_signed_out_of_range(self):
        """Test converting large unsigned to signed raises error."""
        from_type = create_int_type(IntKind.UNSIGNED, IntWidth.W8)
        to_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 200 doesn't fit in signed 8-bit
        with pytest.raises(ValueError):
            convert_int_checked(200, from_type, to_type)


class TestEndianness:
    """Tests for Endianness enum."""

    def test_endianness_values(self):
        """Test Endianness values."""
        assert Endianness.LITTLE == "little"
        assert Endianness.BIG == "big"
        assert Endianness.NATIVE == "native"

    def test_native_endianness(self):
        """Test native_endianness function."""
        # Python runs on little-endian in practice
        assert native_endianness() == Endianness.LITTLE


class TestDefaultRep:
    """Tests for DEFAULT_REP constant."""

    def test_default_rep_structure(self):
        """Test DEFAULT_REP has correct structure."""
        kind, width, endianness = DEFAULT_REP
        assert kind == IntKind.SIGNED
        assert width == IntWidth.W64
        assert endianness == Endianness.LITTLE


class TestIntIntegration:
    """Integration tests for int module."""

    def test_normalize_then_wrap(self):
        """Test normalize then wrap preserves in-range values."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        for i in range(-128, 128):
            normalized = normalize_int(i, int_type)
            wrapped = wrap_int(normalized, int_type)
            assert wrapped == i

    def test_wrap_then_normalize(self):
        """Test wrap then normalize."""
        int_type = create_int_type(IntKind.SIGNED, IntWidth.W8)
        # 128 wraps to -128, normalize keeps -128
        assert normalize_int(wrap_int(128, int_type), int_type) == -128

    def test_all_widths_work(self):
        """Test all width combinations work."""
        for width in [W8, W16, W32, W64, W128]:
            signed = create_int_type(IntKind.SIGNED, width)
            unsigned = create_int_type(IntKind.UNSIGNED, width)

            # Test basic operations
            assert int_byte_size(signed) == width // 8
            assert int_byte_size(unsigned) == width // 8
            assert is_int_signed(signed) is True
            assert is_int_signed(unsigned) is False