import { useEffect, useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { OverUnder } from "../utils/over_under";
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import idl from "./over_under.json"

import { web3 } from "@coral-xyz/anchor";
import { IDLData, IDLType } from "@/utils/idl";

import { ConnectionProvider } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

import type { AppProps } from "next/app";
import dynamic from "next/dynamic";


const PROGRAM = IDLData.metadata.address;
const programID = new PublicKey(PROGRAM);

export interface Wallet {
  publicKey: anchor.web3.PublicKey;
}

type ProgramProps = {
  connection: Connection;
  wallet?: Wallet;
};

export const useProgram = ({ connection, wallet }: ProgramProps) => {
  const [program, setProgram] = useState<anchor.Program<anchor.Idl>>();

  useEffect(() => {
    updateProgram();
  }, [connection, wallet]);

  const updateProgram = () => {
    if (!wallet) return
    const provider = new anchor.AnchorProvider(connection, wallet as anchor.Wallet, {
      preflightCommitment: "recent",
      commitment: "processed",
    });
    const program = new anchor.Program(idl as any, programID, provider);
    setProgram(program);
  };

  return {
    program,
  };
};