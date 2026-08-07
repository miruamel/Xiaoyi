"""
Pytest configuration for Xiaoyi tests.

@package xiaoyi.tests
@brief Pytest fixtures and configuration
@since 0.1.0
"""

import pytest


@pytest.fixture
def event_loop():
    """Provide an event loop for async tests."""
    import asyncio
    loop = asyncio.new_event_loop()
    yield loop
    loop.close()


@pytest.fixture
def sample_config_data():
    """Sample configuration data for testing."""
    return {
        "app": {
            "name": "xiaoyi",
            "version": "0.1.0",
        },
        "database": {
            "host": "localhost",
            "port": 5432,
        },
    }


@pytest.fixture
def temp_file(tmp_path):
    """Create a temporary file for testing."""
    def _create(content: str, suffix: str = ".txt"):
        file = tmp_path / f"test{suffix}"
        file.write_text(content)
        return file
    return _create