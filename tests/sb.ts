import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  SystemProgram,
  VersionedTransaction,
} from "@solana/web3.js";
import {
  AnchorUtils,
  InstructionUtils,
  Queue,
  Randomness,
  SB_ON_DEMAND_PID,
  sleep,
} from "@switchboard-xyz/on-demand";
import dotenv from "dotenv";
import * as fs from "fs";
import reader from "readline-sync";

(async function () {
  dotenv.config();
  console.clear();
  const { keypair, connection, provider, wallet } = await AnchorUtils.loadEnv();
  
  const payer = wallet.payer;
  // Switchboard sbQueue fixed
  const sbQueue = new PublicKey("FfD96yeXs4cxZshoPPSKhSPgVQxLAJUT3gefgh84m1Di");
  const sbProgramId = SB_ON_DEMAND_PID;
  const sbIdl = await anchor.Program.fetchIdl(sbProgramId, provider);
  const sbProgram = new anchor.Program(sbIdl!, sbProgramId, provider);
  const queueAccount = new Queue(sbProgram, sbQueue);

  // setup
  const path = "sb-randomness/target/deploy/sb_randomness-keypair.json";
  const [_, myProgramKeypair] = await AnchorUtils.initWalletFromFile(path);
  const coinFlipProgramId = myProgramKeypair.publicKey;
  const coinFlipProgram = await myAnchorProgram(provider, coinFlipProgramId);

  const rngKp = Keypair.generate();
const [randomness, ix] = await Randomness.create(sbProgram, rngKp, sbQueue);

const commitIx = await randomness.commitIx(sbQueue);
// Add this instruction to your coinFlip transaction and send it

const revealIx = await randomness.revealIx();
// Execute the reveal instruction, followed by your program's settle_flip function



const revealIx = await randomness.revealIx();
// Execute the reveal instruction, followed by your program's settle_flip function


randomness.serializeIxToFile(
    [revealIx, settleFlipIx],
    "serializedIx.bin"
  );


  const settleFlipIx = await coinFlipProgram.instruction.settleFlip(
    escrowBump,
    {
      accounts: {
        playerState: playerStateAccount,
        randomnessAccountData: randomness.pubkey,
        escrowAccount: escrowAccount,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    }
  );
// Add the revealIx and this instruction together and execute 