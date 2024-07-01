'use client';

import { SetStateAction, useState } from 'react';
import { AppHero } from '../ui/ui-layout';

export default function DashboardFeature() {
  const [solAmountOver, setSolAmountOver] = useState('');
  const [solAmountUnder, setSolAmountUnder] = useState('');

  const handleSolAmountChangeOver = (event: {
    target: { value: SetStateAction<string> };
  }) => {
    setSolAmountOver(event.target.value);
  };

  const handleSolAmountChangeUnder = (event: {
    target: { value: SetStateAction<string> };
  }) => {
    setSolAmountUnder(event.target.value);
  };

  const handleBetOver = () => {
    console.log(`Betting over with ${solAmountOver} SOL`);
    // Implement the betting logic here
  };

  const handleBetUnder = () => {
    console.log(`Betting under with ${solAmountUnder} SOL`);
    // Implement the betting logic here
  };

  return (
    <div>
      <AppHero
        title="Over / Under"
        subtitle="Bet on whether the current round's random number, 0 - 1000, will be higher or lower than the previous round's random number."
      />
      {/* Centered Current Round and Previous Number Section */}
      <div className="text-center" style={{ marginBottom: '2rem' }}>
        <div style={{ fontSize: '1rem', marginBottom: '1rem' }}>
          <p>Current Round: {5}</p>
        </div>
        <div style={{ fontSize: '1.50rem' }}>
          <p style={{ textAlign: 'center' }}>
            Previous Random Number:
            <br />
            <span style={{ display: 'block', fontSize: '2.5rem' }}>{217}</span>
          </p>
        </div>
      </div>

      {/* Flex container for Bet Over and Bet Under Sections */}
      <div className="flex justify-between max-w-6xl mx-auto sm:px-6 lg:px-8">
        {/* Bet Under Section */}
        <div
          className="flex justify-center items-center"
          style={{ width: '40%', alignSelf: 'flex-end', paddingBottom: '35%' }}
        >
          <button onClick={handleBetUnder} className="button">
            Bet Under
          </button>
          <input
            type="number"
            value={solAmountUnder}
            onChange={handleSolAmountChangeUnder}
            className="input"
            placeholder="Bet in SOL "
            style={{
              textAlign: 'right',
              marginLeft: '10px',
              border: '1px solid white',
            }}
          />
        </div>

        {/* Aesthetic Vertical Bar */}
        <div
          style={{
            height: '100px', // Adjust based on your design needs
            width: '1px',
            paddingBottom: '44%',
            backgroundColor: '#FFFFFF', // Or any color that fits the design
            alignSelf: 'center', // This centers the bar vertically within the flex container
          }}
        ></div>

        {/* Bet Over Section */}
        <div
          className="flex justify-center items-center"
          style={{ width: '40%', alignSelf: 'flex-end', paddingBottom: '35%' }}
        >
          <input
            type="number"
            value={solAmountOver}
            onChange={handleSolAmountChangeOver}
            className="input"
            placeholder="Bet in SOL"
            style={{
              textAlign: 'left',
              marginRight: '10px',
              border: '1px solid white',
            }}
          />
          <button onClick={handleBetOver} className="button">
            Bet Over
          </button>
        </div>
      </div>
    </div>
  );
}

{
  /* Render links */
}
{
  /* {links.map((link, index) => (
            <div key={index}>
              <a
                href={link.href}
                className="link"
                target="_blank"
                rel="noopener noreferrer"
              >
                {link.label}
              </a>
            </div>
          ))} */
}
