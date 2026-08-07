"""
Test suite for xiaoyi.domain.token.primitive.bool module.

@package xiaoyi.tests.domain.token.primitive
@brief Tests for Bool, TRUE, FALSE, bool_not, bool_and, bool_or
@since 0.1.0
"""

import pytest
from xiaoyi.domain.token.primitive.bool import (
    Bool,
    TRUE,
    FALSE,
    bool_not,
    bool_and,
    bool_or,
)


class TestBoolTypeAlias:
    """Tests for Bool type alias."""

    def test_bool_is_bool(self):
        """Test that Bool is bool."""
        assert Bool is bool


class TestConstants:
    """Tests for TRUE and FALSE constants."""

    def test_true_constant(self):
        """Test TRUE constant."""
        assert TRUE is True
        assert TRUE == True

    def test_false_constant(self):
        """Test FALSE constant."""
        assert FALSE is False
        assert FALSE == False

    def test_true_false_distinct(self):
        """Test TRUE and FALSE are distinct."""
        assert TRUE is not FALSE


class TestBoolNot:
    """Tests for bool_not function."""

    def test_bool_not_true(self):
        """Test NOT of True."""
        assert bool_not(True) is False

    def test_bool_not_false(self):
        """Test NOT of False."""
        assert bool_not(False) is True

    def test_bool_not_involution(self):
        """Test double NOT returns original."""
        assert bool_not(bool_not(True)) is True
        assert bool_not(bool_not(False)) is False


class TestBoolAnd:
    """Tests for bool_and function."""

    def test_bool_and_true_true(self):
        """Test AND of True and True."""
        assert bool_and(True, True) is True

    def test_bool_and_true_false(self):
        """Test AND of True and False."""
        assert bool_and(True, False) is False

    def test_bool_and_false_true(self):
        """Test AND of False and True."""
        assert bool_and(False, True) is False

    def test_bool_and_false_false(self):
        """Test AND of False and False."""
        assert bool_and(False, False) is False

    def test_bool_and_commutative(self):
        """Test AND is commutative."""
        assert bool_and(True, False) == bool_and(False, True)
        assert bool_and(True, True) == bool_and(True, True)

    def test_bool_and_associative(self):
        """Test AND is associative."""
        assert bool_and(bool_and(True, False), True) == bool_and(True, bool_and(False, True))


class TestBoolOr:
    """Tests for bool_or function."""

    def test_bool_or_true_true(self):
        """Test OR of True and True."""
        assert bool_or(True, True) is True

    def test_bool_or_true_false(self):
        """Test OR of True and False."""
        assert bool_or(True, False) is True

    def test_bool_or_false_true(self):
        """Test OR of False and True."""
        assert bool_or(False, True) is True

    def test_bool_or_false_false(self):
        """Test OR of False and False."""
        assert bool_or(False, False) is False

    def test_bool_or_commutative(self):
        """Test OR is commutative."""
        assert bool_or(True, False) == bool_or(False, True)
        assert bool_or(False, False) == bool_or(False, False)

    def test_bool_or_associative(self):
        """Test OR is associative."""
        assert bool_or(bool_or(True, False), True) == bool_or(True, bool_or(False, True))


class TestBoolIntegration:
    """Integration tests for bool module."""

    def test_de_morgan_laws(self):
        """Test De Morgan's laws."""
        # not (a and b) == (not a) or (not b)
        assert bool_not(bool_and(True, False)) == bool_or(bool_not(True), bool_not(False))
        assert bool_not(bool_and(True, True)) == bool_or(bool_not(True), bool_not(True))

        # not (a or b) == (not a) and (not b)
        assert bool_not(bool_or(True, False)) == bool_and(bool_not(True), bool_not(False))
        assert bool_not(bool_or(False, False)) == bool_and(bool_not(False), bool_not(False))

    def test_identity_laws(self):
        """Test identity laws."""
        assert bool_and(True, TRUE) is True
        assert bool_or(False, FALSE) is False

    def test_domination_laws(self):
        """Test domination laws."""
        assert bool_and(False, TRUE) is False
        assert bool_or(True, FALSE) is True

    def test_idempotent_laws(self):
        """Test idempotent laws."""
        assert bool_and(True, True) is True
        assert bool_or(False, False) is False