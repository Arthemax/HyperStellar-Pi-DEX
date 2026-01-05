import React, { useState, useEffect } from 'react';
import { Line } from 'react-chartjs-2';
import { Server, Keypair } from 'stellar-sdk';
import { PiCoinAPI } from 'pi-coin-sdk'; // From stellar-pi-coin-sdk
import './Dashboard.css'; // For quantum animations

const Dashboard = () => {
  const [account, setAccount] = useState(null);
  const [balances, setBalances] = useState({ stellar: 0, pi: 0, hyperpi: 0 });
  const [priceData, setPriceData] = useState([]);
  const [arbitrageAlert, setArbitrageAlert] = useState(false);
  const [quantumColor, setQuantumColor] = useState('#00ff00'); // Hyper-tech random color

  const stellarServer = new Server('https://horizon-testnet.stellar.org');
  const piApi = new PiCoinAPI({ apiKey: 'your_pi_api_key' }); // From stellar-pi-coin-sdk

  useEffect(() => {
    // Quantum-inspired color shift (random every 5 seconds)
    const interval = setInterval(() => {
      setQuantumColor(`#${Math.floor(Math.random()*16777215).toString(16)}`);
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  const connectWallet = async () => {
    // Simulate wallet connection (use Freighter or similar in production)
    const keypair = Keypair.random();
    setAccount(keypair.publicKey());
    // Load balances
    const stellarAccount = await stellarServer.loadAccount(keypair.publicKey());
    const stellarBal = stellarAccount.balances.find(b => b.asset_code === 'XLM')?.balance || 0;
    const piBal = await piApi.getBalance(keypair.publicKey());
    const hyperpiBal = 1000; // Simulate from stablecoin contract
    setBalances({ stellar: stellarBal, pi: piBal, hyperpi: hyperpiBal });
  };

  const fetchPrices = async () => {
    // AI-driven price prediction (simulate with random data)
    const newData = [...priceData, { time: new Date().toLocaleTimeString(), price: Math.random() * 100 }];
    setPriceData(newData.slice(-20)); // Keep last 20 points
    // Check for arbitrage (hyper-tech alert)
    if (Math.random() > 0.8) setArbitrageAlert(true);
  };

  useEffect(() => {
    const interval = setInterval(fetchPrices, 10000); // Update every 10s
    return () => clearInterval(interval);
  }, [priceData]);

  const swapTokens = async () => {
    // Simulate swap via DEX contract (use Soroban invoke in production)
    alert('Swapping tokens via DEX... (integrate with stellar-pi-coin-sdk for Pi Coin)');
    // Call arbitrage bot or bridge script off-chain
  };

  const stakeLiquidity = async () => {
    // Simulate staking in yield farming contract
    alert('Staking liquidity for quantum-boosted rewards!');
  };

  const chartData = {
    labels: priceData.map(d => d.time),
    datasets: [{
      label: 'HYPERPI Price (AI Predicted)',
      data: priceData.map(d => d.price),
      borderColor: quantumColor, // Quantum color
      fill: false,
    }],
  };

  return (
    <div className="dashboard" style={{ backgroundColor: quantumColor + '20' }}> {/* Quantum background */}
      <h1>HyperStellar-Pi-DEX Dashboard</h1>
      <button onClick={connectWallet}>Connect Wallet</button>
      {account && (
        <div>
          <h2>Account: {account}</h2>
          <p>Balances: Stellar XLM: {balances.stellar}, Pi Coin: {balances.pi}, HYPERPI: {balances.hyperpi}</p>
          <button onClick={swapTokens}>Swap Tokens</button>
          <button onClick={stakeLiquidity}>Stake for Yield</button>
          {arbitrageAlert && <div className="alert">🚀 Arbitrage Opportunity Detected! (AI Alert)</div>}
          <div className="chart">
            <Line data={chartData} />
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;
