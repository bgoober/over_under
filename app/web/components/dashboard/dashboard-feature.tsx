'use client';

import { SetStateAction, useState } from 'react';
import { AppHero } from '../ui/ui-layout';
import { ExplainerUiModal } from '../cluster/cluster-ui';
import { useProgram } from "../../utils/useProgram";
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import {BN, web3} from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';

export default function DashboardFeature() {
  const [solAmountOver, setSolAmountOver] = useState('');
  const [solAmountUnder, setSolAmountUnder] = useState('');
  const [showModal, setShowModal] = useState(false);
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const { program } = useProgram({ connection, wallet });

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
    sendInstruction(inputValue); // Replace with your actual function call

    // Implement the betting logic here
  };

  const handleBetUnder = async () => {
    console.log(`Betting under with ${solAmountUnder} SOL`);
  //  sendInstruction(inputValue); // Replace with your actual function call
  if (!program|| !wallet) return;
  // amount: u64, bet: u8, round: u64
  const amount = new BN(solAmountUnder);
  const roundnumber = new BN(solAmountUnder);
  const betnumber = 1;

  const [global] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global"), wallet.publicKey.toBuffer()],
    program.programId
  );
  console.log(`global: `, global.toBase58());
  const globalAccount = await program.account.global.fetch(new PublicKey("4NSN6vFkimNYFcDdcL5Yrjjd1Di1wqga4N2ygjzYFsFt"));
  console.log("ok");
  const _roundBN = new BN(globalAccount.round.toString());

    // Convert to 8-byte Buffer in little-endian for other operations
    const _roundBuffer = _roundBN.toArrayLike(Buffer, "le", 8);
    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), _roundBuffer],
      program.programId
    );
    console.log("Ok2");

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), round.toBuffer()],
      program.programId
    );
    console.log("if you don't see this, it's not working");

    const roundAccount = await program.account.round.fetch(round);
    console.log(`round: `, roundAccount.round.toString());

    console.log(`global round: `, globalAccount.round.toString());

    let round_number = roundAccount.round.toNumber();
    console.log(`round number: `, round_number);

    // This should be the player's public key or similar identifier
    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), round.toBuffer(), wallet.publicKey.toBuffer()],
      program.programId
    );

  const tx = await program.methods.placeBet(amount, betnumber, roundnumber)
  .accounts({
    player: wallet?.publicKey,
    house: global,
    global,
    round,
    vault,
    bet
  })
  .rpc({
    skipPreflight:true
  })  ;
    // Implement the betting logic here
  };

  return (
    <div>
      <AppHero
        title="Over / Under"
        subtitle="Bet on whether the current round's random number will be higher or lower than the previous round's random number, 0 - 100."
      >
        {/* Explainer Modal Section */}
        <div
          className="explainer-modal"
          style={{ textAlign: 'center', marginBottom: '1rem' }}
        >
          <ExplainerUiModal
            show={showModal}
            hideModal={() => setShowModal(false)}
          />
          <button
            className="btn btn-xs lg:btn-md btn-primary"
            onClick={() => setShowModal(true)}
            style={{ margin: 'auto', marginBottom: '1.5rem' }}
          >
            How It Works
          </button>
        </div>
        {/* Centered Current Round and Previous Number Section */}
        <div className="text-center" style={{ marginBottom: '2rem' }}>
          <div style={{ fontSize: '1rem', marginBottom: '1rem' }}>
            <p>Current Round: {5}</p>
            <p>Number of Players: {0}/10</p>
          </div>
          <div style={{ fontSize: '1.50rem' }}>
            <p style={{ textAlign: 'center' }}>
              Previous Random Number:
              <br />
              <span style={{ display: 'block', fontSize: '2.5rem' }}>
                {42}
              </span>
            </p>
          </div>
        </div>

        {/* Flex container for Bet Over and Bet Under Sections */}
        <div className="flex justify-between max-w-6xl mx-auto sm:px-6 lg:px-8">
          {/* Bet Under Section */}
          <div
            className="flex justify-center items-center"
            style={{
              width: '40%',
              alignSelf: 'flex-end',
              paddingBottom: '35%',
            }}
          >
            <button onClick={handleBetUnder} className="button">
              Bet{' '}
              <span
                style={{
                  textDecoration: 'underline',
                  textDecorationColor: 'white',
                  textDecorationThickness: '1px',
                  textUnderlineOffset: '3px',
                }}
              >
                Under
              </span>
            </button>
            <input
              type="number"
              value={solAmountUnder}
              onChange={handleSolAmountChangeUnder}
              className="input"
              placeholder="Bet SOL "
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
            style={{
              width: '40%',
              alignSelf: 'flex-end',
              paddingBottom: '35%',
            }}
          >
            <input
              type="number"
              value={solAmountOver}
              onChange={handleSolAmountChangeOver}
              className="input"
              placeholder="Bet SOL"
              style={{
                textAlign: 'left',
                marginRight: '10px',
                border: '1px solid white',
              }}
            />
            <button onClick={handleBetUnder} className="button">
              Bet{' '}
              <span
                style={{
                  textDecoration: 'underline',
                  textDecorationColor: 'white',
                  textDecorationThickness: '1px',
                  textUnderlineOffset: '3px',
                }}
              >
                Over
              </span>
            </button>
          </div>
        </div>
      </AppHero>
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
