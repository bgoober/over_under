import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { OverUnder } from "../target/types/over_under";
import { PublicKey } from '@solana/web3.js';

describe("over_under", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OverUnder as Program<OverUnder>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initGlobal().rpc();
    console.log("Your transaction signature", tx);
  });
});
