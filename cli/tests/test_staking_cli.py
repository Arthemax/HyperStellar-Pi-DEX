import pytest
from click.testing import CliRunner
from cli.staking_cli import stake

def test_stake():
    runner = CliRunner()
    result = runner.invoke(stake, ['--staker', 'user1', '--amount', '1000'])
    assert "Staked" in result.output
