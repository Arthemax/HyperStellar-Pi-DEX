import pytest
from click.testing import CliRunner
from cli.hyper_prediction_cli import predict_trend

def test_predict_trend():
    runner = CliRunner()
    result = runner.invoke(predict_trend, ['--volatility', '50', '--compliance', '80', '--stability', '314159'])
    assert "trend" in result.output
