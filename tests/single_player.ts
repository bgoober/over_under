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
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

// Get the keypair from the wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// derive account addresses up here or under describe but before a test

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
    const tx = await program.methods.initGlobal().accounts({ global }).rpc();
  });

  // initRound
  it("Round initialized!", async () => {
    const globalAccount = await program.account.global.fetch(global);

    console.log(
      `global round: ${globalAccount}`,
      globalAccount.round.toString()
    );

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

    // Fetch the round account
    const roundAccount = await program.account.round.fetch(round);
    console.log(`round: ${roundAccount}`, roundAccount.round.toString());

    // Fetch the vault account
    // const vaultAccount = await program.account.vault.fetch(vault);
    // console.log(`vault: ${vaultAccount}`, vaultAccount.round.toString());
  });

  // placeBet
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
    console.log(`round: ${roundAccount}`, roundAccount.round.toString());

    console.log(
      `global round: ${globalAccount}`,
      globalAccount.round.toString()
    );

    let round_number = roundAccount.round.toNumber();

    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), round.toBuffer(), keypair.publicKey.toBuffer()],
      program.programId
    );

    // Assuming BN is already imported
    // Convert the first and third arguments to BN
    const amountBN = new BN(100000000);
    const roundNumberBN = new BN(round_number);

    const tx = await program.methods
      .placeBet(amountBN, 1, roundNumberBN) // Use BN objects for the first and third arguments
      .accounts({
        house: keypair.publicKey,
        global,
        round,
        vault,
        bet,
        player: keypair.publicKey,
      })
      .signers([keypair])
      .rpc()
      .then(confirm)
      .then(log);

    // fetch the bet
    const betAccount = await program.account.bet.fetch(bet);
    const roundAccount2 = await program.account.round.fetch(round);
    console.log(`bet amount: ${betAccount}`, betAccount.amount.toString());
    console.log("bet: ${betAccount}", betAccount.bet.toString());
    // log the round.bets length
    console.log(
      `round2 bets length: ${roundAccount2}`,
      roundAccount2.bets.length
    );
    console.log("round2 bets: ", roundAccount2.bets);
  });

  // play_round
  it("Round played!", async () => {
    // Fetch the global account
    const globalAccount = await program.account.global.fetch(global);

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
    console.log(`round: ${roundAccount}`, roundAccount.round.toString());

    console.log(
      `global round: ${globalAccount}`,
      globalAccount.round.toString()
    );

    // console.log("test1");

    let account = await anchor
      .getProvider()
      .connection.getAccountInfo(round, "confirmed");
    let sig_ix = Ed25519Program.createInstructionWithPrivateKey({
      privateKey: keypair.secretKey,
      message: account.data.subarray(8),
    });

    // console.log("test2");

    const resolve_ix = await program.methods
      .playRound(Buffer.from(sig_ix.data.buffer.slice(16 + 32, 16 + 32 + 64)))
      .accounts({
        thread: keypair.publicKey,
        house: keypair.publicKey,
        global,
        round,
        vault,
        instructionSysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
        systemProgram: SystemProgram.programId,
      })
      .instruction();

    // console.log("test3");
    const tx = new Transaction().add(sig_ix).add(resolve_ix);

    // console.log("test4");

    await sendAndConfirmTransaction(program.provider.connection, tx, [keypair])
      .then(log)
      .catch((error) => console.error("Transaction # Error:", error));

    // console.log("test5");

    // fetch global
    const globalAccount2 = await program.account.global.fetch(global);
    console.log("global number: ", globalAccount2.number.toString());
    console.log("global round: ", globalAccount2.round.toString());
    console.log("round bets: ", roundAccount.bets);
  });

  it("Winners Assessed!", async () => {
    // fetch global
    const globalAccount = await program.account.global.fetch(global);

    // fetch round
    const _roundBN = new BN(globalAccount.round.toString());
    const _roundBuffer = _roundBN.toArrayLike(Buffer, "le", 8);
    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const roundAccount = await program.account.round.fetch(round);

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), round.toBuffer()],
      program.programId
    );
    const remainingAccounts = roundAccount.bets.map((betAccount) => ({
      pubkey: betAccount,
      isSigner: false,
      isWritable: true,
    }));

      const tx = await program.methods
        .assessWinners()
        .accounts({
          house: keypair.publicKey,
          global,
          round,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .remainingAccounts([...remainingAccounts])
        .signers([keypair])
        .rpc()
        .then(confirm)
        .then(log);
    }
)

  it("Payed Out!", async () => {
    // Fetch the global account
    const globalAccount = await program.account.global.fetch(global);

    console.log("global number: ", globalAccount.number.toString());

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

    console.log(
      `global round: `,
      globalAccount.round.toString()
    );
    // log the global.number
    console.log(
      `global number: `,
      globalAccount.number.toString()
    );

    const [bet] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), round.toBuffer(), keypair.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .payout() // Use BN objects for the first and third arguments
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
  });

  it("Round is Closed!", async () => {
    // fetch global
    const globalAccount = await program.account.global.fetch(global);

    console.log("global number: ", globalAccount.number.toString());
    console.log("global round: ", globalAccount.round.toString());

    const _roundBN = new BN(globalAccount.round.toString());

    // Convert to 8-byte Buffer in little-endian for other operations
    const _roundBuffer = _roundBN.toArrayLike(Buffer, "le", 8);

    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), _roundBuffer],
      program.programId
    );

    const tx = await program.methods
      .closeRound() // Use BN objects for the first and third arguments
      .accounts({
        house: keypair.publicKey,
        global,
        round,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc()
      .then(confirm)
      .then(log);
  });
});
