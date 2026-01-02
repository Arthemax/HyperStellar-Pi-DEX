import pytest
from click.testing import CliRunner
from cli.monitoring_cli import log_metric

def test_log_metric():
    runner = CliRunner()
    result = runner.invoke(log_metric, ['--name', 'volatility', '--value', '50'])
    assert "Metric logged" in result.output
