import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { OverUnder } from "../target/types/over_under";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

describe("over_under", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OverUnder as Program<OverUnder>;

  const house = new PublicKey("4QPAeQG6CTq2zMJAVCJnzY9hciQteaMkgBmcyGL7Vrwp");

  const [global] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global"), house.toBuffer()],
    program.programId
  );

  it("Global initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initGlobal().accounts({ global }).rpc();
    console.log("Your transaction signature", tx);
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
      .accounts({ house, global, round, vault })
      .rpc();
    console.log("Your transaction signature", tx);

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
      [Buffer.from("bet"), round.toBuffer(), house.toBuffer()],
      program.programId
    );

    // Assuming BN is already imported
    // Convert the first and third arguments to BN
    const amountBN = new BN(100000000);
    const roundNumberBN = new BN(round_number);

    const tx = await program.methods
      .placeBet(amountBN, 1, roundNumberBN) // Use BN objects for the first and third arguments
      .accounts({ house, global, round, vault, bet })
      .rpc();
    console.log("Your transaction signature", tx);

 
    // fetch the bet
    const betAccount = await program.account.bet.fetch(bet);
    console.log(`bet amount: ${betAccount}`, betAccount.amount.toString());
    console.log('bet: ${betAccount}', betAccount.bet.toString())
  });

  // play_round
  
});
