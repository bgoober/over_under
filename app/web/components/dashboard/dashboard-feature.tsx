// @ts-ignore

'use client';

import { SetStateAction, useState } from 'react';
import { AppHero } from '../ui/ui-layout';
import { ExplainerUiModal } from '../cluster/cluster-ui';
import { useProgram } from '../../utils/useProgram';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import { BN, web3 } from '@coral-xyz/anchor';
import { Keypair, PublicKey, sendAndConfirmTransaction } from '@solana/web3.js';

import house_wallet from '/home/agent/.config/solana/id.json';
import { SystemProgram } from '@solana/web3.js';
// import wallet2 from '../../../../wallet.json';
const house = Keypair.fromSecretKey(new Uint8Array(house_wallet));
// const player1 = Keypair.fromSecretKey(new Uint8Array(wallet2));

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

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature, ...block });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(signature);
    return signature;
  };

  const handleBetOver = async () => {
    console.log(`Betting OVER with ${solAmountOver} SOL`);
    if (!program || !wallet) return;
    // Assuming solAmountUnder is a string representing the SOL amount,
    // convert it to a BigNumber representing lamports.
    // 1 SOL = 1,000,000,000 lamports
    const lamportsPerSol = new BN(1_000_000_000);
    const amountInSol = new BN(solAmountOver); // This might need parsing if solAmountUnder is not already a BN compatible format
    const amountInLamports = amountInSol.mul(lamportsPerSol);
    const betnumber = 1;

    const [global] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('global'), house.publicKey.toBuffer()],
      program.programId
    );
    const globalAccount = await program.account.global.fetch(global);

    // console.log(`global round: `, globalAccount.round);

    const _roundBN = new BN((globalAccount.round as number).toString());
    const _roundBuffer = _roundBN.toArrayLike(Buffer, 'le', 8);
    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('round'), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const roundAccount = await program.account.round.fetch(round);

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), round.toBuffer()],
      program.programId
    );

    let round_number = new BN((roundAccount.round as number).toString());

    // This should be the player's public key or similar identifier
    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('bet'), round.toBuffer(), wallet.publicKey.toBuffer()],
      program.programId
    );
    console.log(`round: `, round.toBase58());
    const tx = await program.methods
      .placeBet(amountInLamports, betnumber, round_number)
      .accounts({
        player: wallet?.publicKey,
        house: house.publicKey,
        global,
        round,
        vault,
        bet,
        systemProgram: SystemProgram.programId,
      })
      .rpc({
        skipPreflight: true,
      })
      .then(confirm)
      .then(log);
    console.log('Success');
  };

  const handleBetUnder = async () => {
    console.log(`Betting UNDER with ${solAmountUnder} SOL`);
    if (!program || !wallet) return;

    // Assuming solAmountUnder is a string representing the SOL amount,
    // convert it to a BigNumber representing lamports.
    // 1 SOL = 1,000,000,000 lamports
    const lamportsPerSol = new BN(1_000_000_000);
    const amountInSol = new BN(solAmountUnder); // This might need parsing if solAmountUnder is not already a BN compatible format
    const amountInLamports = amountInSol.mul(lamportsPerSol);

    // console.log("Program:", program);
    // console.log("Methods available:", program?.methods);

    const betnumber = 0;

    const [global] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('global'), house.publicKey.toBuffer()],
      program.programId
    );
    const globalAccount = await program.account.global.fetch(global);
    // console.log(globalAccount.round)

    const _roundBN = new BN((globalAccount.round as number).toString());
    const _roundBuffer = _roundBN.toArrayLike(Buffer, 'le', 8);
    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('round'), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const roundAccount = await program.account.round.fetch(round);
    // console.log(`roundAccount: `, roundAccount);

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), round.toBuffer()],
      program.programId
    );

    let round_number = new BN((roundAccount.round as number).toString());

    // This should be the player's public key or similar identifier
    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('bet'), round.toBuffer(), wallet.publicKey.toBuffer()],
      program.programId
    );
    console.log(`round: `, round.toBase58());
    const tx = await program.methods
      .placeBet(amountInLamports, betnumber, round_number)
      .accounts({
        player: wallet?.publicKey,
        house: house.publicKey,
        global,
        round,
        vault,
        bet,
      })
      .rpc({
        skipPreflight: true,
      })
      .then(confirm)
      .then(log);
    console.log('Success');
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
              <span style={{ display: 'block', fontSize: '2.5rem' }}>{42}</span>
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
              paddingBottom: '20%',
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
              paddingBottom: '20%',
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
            <button onClick={handleBetOver} className="button">
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
