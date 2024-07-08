import React, { useState, useEffect } from 'react';
import { ethers } from 'ethers';

const GlobalAndRoundAccount = ({ globalAccountAddress, contractABI, providerURL }) => {
  const [globalState, setGlobalState] = useState(null);
  const [roundState, setRoundState] = useState(null);

  // Initialize ethers provider
  const provider = new ethers.providers.JsonRpcProvider(providerURL);
  const contract = new ethers.Contract(globalAccountAddress, contractABI, provider);

  const fetchGlobalState = async () => {
    // Assume getGlobalState is a contract function to get the global state
    const state = await contract.getGlobalState();
    setGlobalState(state);
  };

  const fetchRoundState = async () => {
    // Assume getCurrentRoundAddress and getRoundState are contract functions
    const roundAddress = await contract.getCurrentRoundAddress();
    const roundContract = new ethers.Contract(roundAddress, contractABI, provider);
    const state = await roundContract.getRoundState();
    setRoundState(state);
  };

  useEffect(() => {
    fetchGlobalState();
    fetchRoundState();
    // Polling every 30 seconds
    const interval = setInterval(() => {
      fetchRoundState();
    }, 30000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div>
      <h2>Global State</h2>
      <p>{JSON.stringify(globalState)}</p>
      <h2>Round State</h2>
      <p>{JSON.stringify(roundState)}</p>
    </div>
  );
};

export default GlobalAndRoundAccount;