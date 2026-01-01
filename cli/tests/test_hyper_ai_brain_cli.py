import pytest
from click.testing import CliRunner
from cli.hyper_ai_brain_cli import register_planet

def test_register_planet():
    runner = CliRunner()
    result = runner.invoke(register_planet, ['--name', 'Earth', '--resources', '1000000'])
    assert "Registered" in result.output
