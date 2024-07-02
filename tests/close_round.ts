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
  
    it("Round is Closed!", async () => {
        // fetch global
        const globalAccount = await program.account.global.fetch(global);
    
        console.log("old global round: ", globalAccount.round.toString());
        console.log("old global number: ", globalAccount.number.toString());
    
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
    
        // fetch global
        const globalAccount2 = await program.account.global.fetch(global);
        console.log("new global round: ", globalAccount2.round.toString());
        console.log("new global number: ", globalAccount2.number.toString());
      });
})