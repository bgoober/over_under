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
    console.log(`round: `, roundAccount.round.toString());

    console.log(`global round: }`, globalAccount.round.toString());

    let account = await anchor
      .getProvider()
      .connection.getAccountInfo(round, "confirmed");
    let sig_ix = Ed25519Program.createInstructionWithPrivateKey({
      privateKey: keypair.secretKey,
      message: account.data.subarray(8),
    });

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

    const tx = new Transaction().add(sig_ix).add(resolve_ix);

    await sendAndConfirmTransaction(program.provider.connection, tx, [keypair])
      .then(log)
      .catch((error) => console.error("Transaction # Error:", error));
  });
})