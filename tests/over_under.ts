import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { OverUnder } from "../target/types/over_under";
import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

describe("over_under", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OverUnder as Program<OverUnder>;

  const house = new PublicKey("4QPAeQG6CTq2zMJAVCJnzY9hciQteaMkgBmcyGL7Vrwp");

  const [global] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global"), house.toBuffer()],
    program.programId,
  )

  it("Global initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initGlobal().accounts({global}).rpc();
    console.log("Your transaction signature", tx);
  });

  // initRound
  it("Round initialized!", async () => {
    const globalAccount = await program.account.global.fetch(global);

    console.log(`global round: ${globalAccount}`, globalAccount.round.toString());

    const _round = new BN(globalAccount.round.toString());

    const [round] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("round"), global.toBuffer(), Buffer.from(_round.toString())],
      program.programId,
    )

    const [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), round.toBuffer()],
      program.programId,
    )

   const tx = await program.methods.initRound(_round).accounts({global, house, round, vault}).rpc();
   console.log("Your transaction signature", tx);
  });
});
