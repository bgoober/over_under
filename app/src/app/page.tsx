// pages/page.tsx
// use client
import React, { useState } from 'react';

export default function BetPage() {
  const [solAmount, setSolAmount] = useState<number | ''>('');

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value === '' ? '' : Number(e.target.value);
    setSolAmount(value);
  };

  const handleLowerBet = () => {
    console.log('Lower Bet clicked');
    // Implement lower bet functionality
  };

  const handleHigherBet = () => {
    console.log('Higher Bet clicked');
    // Implement higher bet functionality
  };

  const styles = {
    pageContainer: { display: 'flex', height: '100vh' },
    buttonContainer: { flex: 1, display: 'flex', justifyContent: 'center', alignItems: 'center' },
    inputContainer: {
      flex: 1,
      display: 'flex',
      flexDirection: 'column' as 'column', // Type assertion here
      alignItems: 'center',
      justifyContent: 'center',
    },
    inputStyle: { marginBottom: '20px' },
    roundInfo: {
      width: '100px',
      height: '100px',
      borderRadius: '50%',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      border: '2px solid black',
    },
  };

  return (
    <div style={styles.pageContainer}>
      {/* Section 1: Lower Bet Button */}
      <div style={styles.buttonContainer}>
        <button onClick={handleLowerBet}>Lower Bet</button>
      </div>

      {/* Section 2: Higher Bet Button */}
      <div style={styles.buttonContainer}>
        <button onClick={handleHigherBet}>Higher Bet</button>
      </div>

      {/* Section 3: SOL Input and Round Information */}
      <div style={styles.inputContainer}>
        <input
          type="number"
          value={solAmount}
          onChange={handleInputChange}
          placeholder="SOL Amount"
          style={styles.inputStyle}
        />
        <div style={styles.roundInfo}>
          {/* Placeholder for round information */}
          Round Info
        </div>
      </div>
    </div>
  );
}