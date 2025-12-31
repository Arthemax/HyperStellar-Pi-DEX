import pytest
from click.testing import CliRunner
from cli.ai_evolution_cli import predict

def test_predict():
    runner = CliRunner()
    result = runner.invoke(predict, ['--input', '50'])
    assert "Prediction" in result.output
