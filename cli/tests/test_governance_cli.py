import pytest
from click.testing import CliRunner
from cli.governance_cli import create_proposal

def test_create_proposal():
    runner = CliRunner()
    result = runner.invoke(create_proposal, ['--creator', 'admin', '--description', 'test', '--impact', '80', '--feasibility', '70', '--ethics', '90'])
    assert "Proposal created" in result.output
