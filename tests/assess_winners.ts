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
      });
})