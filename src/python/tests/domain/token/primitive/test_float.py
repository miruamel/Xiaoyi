"""
Test suite for xiaoyi.domain.token.primitive.float module.

@package xiaoyi.tests.domain.token.primitive
@brief Tests for F32, F64, F32Consts, F64Consts, is_finite/nan/infinite
@since 0.1.0
"""

import pytest
import math
from xiaoyi.domain.token.primitive.float.f32 import (
    F32,
    F32Bits,
    F32Consts,
    is_f32_finite,
    is_f32_nan,
    is_f32_infinite,
)
from xiaoyi.domain.token.primitive.float.f64 import (
    F64,
    F64Bits,
    F64Consts,
    is_f64_finite,
    is_f64_nan,
    is_f64_infinite,
)


class TestF32TypeAliases:
    """Tests for F32 and F32Bits type aliases."""

    def test_f32_is_float(self):
        """Test that F32 is float."""
        assert F32 is float

    def test_f32_bits_is_int(self):
        """Test that F32Bits is int."""
        assert F32Bits is int


class TestF64TypeAliases:
    """Tests for F64 and F64Bits type aliases."""

    def test_f64_is_float(self):
        """Test that F64 is float."""
        assert F64 is float

    def test_f64_bits_is_int(self):
        """Test that F64Bits is int."""
        assert F64Bits is int


class TestF32Consts:
    """Tests for F32Consts class."""

    def test_f32_consts_infinity(self):
        """Test F32Consts.INFINITY."""
        assert F32Consts.INFINITY == float("inf")
        assert math.isinf(F32Consts.INFINITY)
        assert F32Consts.INFINITY > 0

    def test_f32_consts_neg_infinity(self):
        """Test F32Consts.NEG_INFINITY."""
        assert F32Consts.NEG_INFINITY == float("-inf")
        assert math.isinf(F32Consts.NEG_INFINITY)
        assert F32Consts.NEG_INFINITY < 0

    def test_f32_consts_nan(self):
        """Test F32Consts.NAN."""
        assert math.isnan(F32Consts.NAN)

    def test_f32_consts_min_positive(self):
        """Test F32Consts.MIN_POSITIVE."""
        assert F32Consts.MIN_POSITIVE == 1.175494351e-38
        assert F32Consts.MIN_POSITIVE > 0

    def test_f32_consts_max(self):
        """Test F32Consts.MAX."""
        assert F32Consts.MAX == 3.402823466e38
        assert F32Consts.MAX > 0

    def test_f32_consts_min(self):
        """Test F32Consts.MIN."""
        assert F32Consts.MIN == -3.402823466e38
        assert F32Consts.MIN < 0

    def test_f32_consts_epsilon(self):
        """Test F32Consts.EPSILON."""
        assert F32Consts.EPSILON == 1.19209290e-7
        assert F32Consts.EPSILON > 0


class TestF64Consts:
    """Tests for F64Consts class."""

    def test_f64_consts_infinity(self):
        """Test F64Consts.INFINITY."""
        assert F64Consts.INFINITY == float("inf")
        assert math.isinf(F64Consts.INFINITY)
        assert F64Consts.INFINITY > 0

    def test_f64_consts_neg_infinity(self):
        """Test F64Consts.NEG_INFINITY."""
        assert F64Consts.NEG_INFINITY == float("-inf")
        assert math.isinf(F64Consts.NEG_INFINITY)
        assert F64Consts.NEG_INFINITY < 0

    def test_f64_consts_nan(self):
        """Test F64Consts.NAN."""
        assert math.isnan(F64Consts.NAN)

    def test_f64_consts_min_positive(self):
        """Test F64Consts.MIN_POSITIVE."""
        assert F64Consts.MIN_POSITIVE == 2.2250738585072014e-308
        assert F64Consts.MIN_POSITIVE > 0

    def test_f64_consts_max(self):
        """Test F64Consts.MAX."""
        assert F64Consts.MAX == 1.7976931348623157e308
        assert F64Consts.MAX > 0

    def test_f64_consts_min(self):
        """Test F64Consts.MIN."""
        assert F64Consts.MIN == -1.7976931348623157e308
        assert F64Consts.MIN < 0

    def test_f64_consts_epsilon(self):
        """Test F64Consts.EPSILON."""
        assert F64Consts.EPSILON == 2.220446049250313e-16
        assert F64Consts.EPSILON > 0


class TestIsF32Finite:
    """Tests for is_f32_finite function."""

    def test_is_f32_finite_normal(self):
        """Test finite normal values."""
        assert is_f32_finite(0.0) is True
        assert is_f32_finite(1.0) is True
        assert is_f32_finite(-1.0) is True
        assert is_f32_finite(3.14) is True
        assert is_f32_finite(F32Consts.MAX) is True
        assert is_f32_finite(F32Consts.MIN) is True
        assert is_f32_finite(F32Consts.MIN_POSITIVE) is True

    def test_is_f32_finite_infinite(self):
        """Test infinite values return False."""
        assert is_f32_finite(float("inf")) is False
        assert is_f32_finite(float("-inf")) is False
        assert is_f32_finite(F32Consts.INFINITY) is False
        assert is_f32_finite(F32Consts.NEG_INFINITY) is False

    def test_is_f32_finite_nan(self):
        """Test NaN returns False."""
        assert is_f32_finite(float("nan")) is False
        assert is_f32_finite(F32Consts.NAN) is False


class TestIsF32Nan:
    """Tests for is_f32_nan function."""

    def test_is_f32_nan_nan(self):
        """Test NaN values return True."""
        assert is_f32_nan(float("nan")) is True
        assert is_f32_nan(F32Consts.NAN) is True

    def test_is_f32_nan_finite(self):
        """Test finite values return False."""
        assert is_f32_nan(0.0) is False
        assert is_f32_nan(1.0) is False
        assert is_f32_nan(F32Consts.MAX) is False

    def test_is_f32_nan_infinite(self):
        """Test infinite values return False."""
        assert is_f32_nan(float("inf")) is False
        assert is_f32_nan(float("-inf")) is False


class TestIsF32Infinite:
    """Tests for is_f32_infinite function."""

    def test_is_f32_infinite_positive(self):
        """Test positive infinity returns True."""
        assert is_f32_infinite(float("inf")) is True
        assert is_f32_infinite(F32Consts.INFINITY) is True

    def test_is_f32_infinite_negative(self):
        """Test negative infinity returns True."""
        assert is_f32_infinite(float("-inf")) is True
        assert is_f32_infinite(F32Consts.NEG_INFINITY) is True

    def test_is_f32_infinite_finite(self):
        """Test finite values return False."""
        assert is_f32_infinite(0.0) is False
        assert is_f32_infinite(1.0) is False
        assert is_f32_infinite(F32Consts.MAX) is False

    def test_is_f32_infinite_nan(self):
        """Test NaN returns False."""
        assert is_f32_infinite(float("nan")) is False
        assert is_f32_infinite(F32Consts.NAN) is False


class TestIsF64Finite:
    """Tests for is_f64_finite function."""

    def test_is_f64_finite_normal(self):
        """Test finite normal values."""
        assert is_f64_finite(0.0) is True
        assert is_f64_finite(1.0) is True
        assert is_f64_finite(-1.0) is True
        assert is_f64_finite(3.14) is True
        assert is_f64_finite(F64Consts.MAX) is True
        assert is_f64_finite(F64Consts.MIN) is True
        assert is_f64_finite(F64Consts.MIN_POSITIVE) is True

    def test_is_f64_finite_infinite(self):
        """Test infinite values return False."""
        assert is_f64_finite(float("inf")) is False
        assert is_f64_finite(float("-inf")) is False
        assert is_f64_finite(F64Consts.INFINITY) is False
        assert is_f64_finite(F64Consts.NEG_INFINITY) is False

    def test_is_f64_finite_nan(self):
        """Test NaN returns False."""
        assert is_f64_finite(float("nan")) is False
        assert is_f64_finite(F64Consts.NAN) is False


class TestIsF64Nan:
    """Tests for is_f64_nan function."""

    def test_is_f64_nan_nan(self):
        """Test NaN values return True."""
        assert is_f64_nan(float("nan")) is True
        assert is_f64_nan(F64Consts.NAN) is True

    def test_is_f64_nan_finite(self):
        """Test finite values return False."""
        assert is_f64_nan(0.0) is False
        assert is_f64_nan(1.0) is False
        assert is_f64_nan(F64Consts.MAX) is False

    def test_is_f64_nan_infinite(self):
        """Test infinite values return False."""
        assert is_f64_nan(float("inf")) is False
        assert is_f64_nan(float("-inf")) is False


class TestIsF64Infinite:
    """Tests for is_f64_infinite function."""

    def test_is_f64_infinite_positive(self):
        """Test positive infinity returns True."""
        assert is_f64_infinite(float("inf")) is True
        assert is_f64_infinite(F64Consts.INFINITY) is True

    def test_is_f64_infinite_negative(self):
        """Test negative infinity returns True."""
        assert is_f64_infinite(float("-inf")) is True
        assert is_f64_infinite(F64Consts.NEG_INFINITY) is True

    def test_is_f64_infinite_finite(self):
        """Test finite values return False."""
        assert is_f64_infinite(0.0) is False
        assert is_f64_infinite(1.0) is False
        assert is_f64_infinite(F64Consts.MAX) is False

    def test_is_f64_infinite_nan(self):
        """Test NaN returns False."""
        assert is_f64_infinite(float("nan")) is False
        assert is_f64_infinite(F64Consts.NAN) is False


class TestFloatIntegration:
    """Integration tests for float module."""

    def test_f32_f64_consts_different(self):
        """Test that F32 and F64 constants differ."""
        assert F32Consts.MAX != F64Consts.MAX
        assert F32Consts.MIN != F64Consts.MIN
        assert F32Consts.EPSILON != F64Consts.EPSILON
        assert F32Consts.MIN_POSITIVE != F64Consts.MIN_POSITIVE

    def test_f32_f64_same_infinity_nan(self):
        """Test that infinity and NaN are same for both."""
        assert F32Consts.INFINITY == F64Consts.INFINITY
        assert F32Consts.NEG_INFINITY == F64Consts.NEG_INFINITY
        assert math.isnan(F32Consts.NAN) and math.isnan(F64Consts.NAN)

    def test_classification_consistency(self):
        """Test that classification functions are consistent."""
        # Finite values
        assert is_f32_finite(1.0) and not is_f32_nan(1.0) and not is_f32_infinite(1.0)
        assert is_f64_finite(1.0) and not is_f64_nan(1.0) and not is_f64_infinite(1.0)

        # Infinite values
        assert not is_f32_finite(float("inf")) and not is_f32_nan(float("inf")) and is_f32_infinite(float("inf"))
        assert not is_f64_finite(float("inf")) and not is_f64_nan(float("inf")) and is_f64_infinite(float("inf"))

        # NaN values
        assert not is_f32_finite(float("nan")) and is_f32_nan(float("nan")) and not is_f32_infinite(float("nan"))
        assert not is_f64_finite(float("nan")) and is_f64_nan(float("nan")) and not is_f64_infinite(float("nan"))

    def test_all_functions_handle_both_types(self):
        """Test that both f32 and f64 functions work on Python floats."""
        test_values = [0.0, 1.0, -1.0, 3.14, float("inf"), float("-inf"), float("nan")]

        for val in test_values:
            # Both should handle the same values
            f32_finite = is_f32_finite(val)
            f64_finite = is_f64_finite(val)
            assert f32_finite == f64_finite

            f32_nan = is_f32_nan(val)
            f64_nan = is_f64_nan(val)
            assert f32_nan == f64_nan

            f32_inf = is_f32_infinite(val)
            f64_inf = is_f64_infinite(val)
            assert f32_inf == f64_inf