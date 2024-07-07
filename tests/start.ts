// initGlobal + initRound + placeBet -- all as House

import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { OverUnder } from "../target/types/over_under";
import {
  Transaction,
  Ed25519Program,
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import BN from "bn.js";

// use my local keypair for signing
import wallet from "/home/agent/.config/solana/id.json";

// Get the keypair from the wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

describe("over_under", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.OverUnder as Program<OverUnder>;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature, ...block });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(signature);
    return signature;
  };

  const [global] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global"), keypair.publicKey.toBuffer()],
    program.programId
  );

  it("Global initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .initGlobal()
        .accounts({ global, house: keypair.publicKey })
        .rpc()
        .then(confirm)
        .then(log);
    } catch (error) {
      if (error.message.includes("already in use")) {
        // Accept the error and continue
        console.log("Global account already initialized.");
      } else {
        throw error;
      }
    }
  });

  // initRound
  it("Round initialized!", async () => {
    const globalAccount = await program.account.global.fetch(global);

    console.log(`global round: `, globalAccount.round.toString());

    // Use a BN object for operations requiring BN
    const _roundBN = new BN(globalAccount.round.toString());

    // Convert to 8-byte Buffer in little-endian for other operations
    const _roundBuffer = _roundBN.toArrayLike(Buffer, "le", 8);

    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), round.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initRound(_roundBN)
      .accounts({ house: keypair.publicKey, global, round, vault })
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Bet placed!", async () => {
    const globalAccount = await program.account.global.fetch(global);
    // Use a BN object for operations requiring BN
    const _roundBN = new BN(globalAccount.round.toString());

    // Convert to 8-byte Buffer in little-endian for other operations
    const _roundBuffer = _roundBN.toArrayLike(Buffer, "le", 8);
    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), round.toBuffer()],
      program.programId
    );
    const roundAccount = await program.account.round.fetch(round);
    console.log(`round: `, roundAccount.round.toString());

    console.log(`global round: `, globalAccount.round.toString());

    let round_number = roundAccount.round.toNumber();
    console.log(`round number: `, round_number);

    // This should be the player's public key or similar identifier
    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), round.toBuffer(), keypair.publicKey.toBuffer()],
      program.programId
    );
    console.log(`bet: `, bet.toString());

    // Assuming BN is already imported
    // Convert the first and third arguments to BN
    const amountBN = new BN(10);
    const roundNumberBN = new BN(round_number);

    // Assuming solAmountUnder is a string representing the SOL amount,
    // convert it to a BigNumber representing lamports.
    // 1 SOL = 1,000,000,000 lamports
    const lamportsPerSol = new BN(1_000_000_000);
    const amountInLamports = amountBN.mul(lamportsPerSol);

    const tx = await program.methods
      .placeBet(amountInLamports, 1, roundNumberBN) // Use BN objects for the first and third arguments
      .accounts({
        house: keypair.publicKey,
        global,
        round,
        vault,
        bet,
        player: keypair.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc()
      .then(confirm)
      .then(log);

    // fetch the bet
    //   const betAccount = await program.account.bet.fetch(bet);
    //   const roundAccount2 = await program.account.round.fetch(round);
    //   console.log(`bet amount: `, betAccount.amount.toString());
    //   console.log("bet: ", betAccount.bet.toString());
    //   // log the round.bets length
    //   console.log(`round2 bets length: `, roundAccount2.bets.length);
    //   console.log("round2 players: ", roundAccount2.players);
    //   console.log("round2 bets: ", roundAccount2.bets);
  });
});
