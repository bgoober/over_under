use std::collections::BTreeMap;

use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::{
    errors::Error,
    state::{Bet, Global, Round},
};

#[derive(Accounts)]
pub struct PlayRoundC<'info> {
    #[account(mut)]
    thread: Signer<'info>,

    // the pubkey of the signer of init global
    #[account(mut)]
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub bet: Account<'info, Bet>,

    // round the player is placing a bet in,
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is safe
    pub instruction_sysvar: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlayRoundC<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, Error::Ed25519Program);
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, Error::Ed25519Accounts);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, Error::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, Error::Ed25519Header);

        // Ensure public keys match
        require_keys_eq!(
            signature.public_key.ok_or(Error::Ed25519Pubkey)?,
            self.house.key(),
            Error::Ed25519Pubkey
        );

        // Ensure signatures match
        require!(
            &signature.signature.ok_or(Error::Ed25519Signature)?.eq(sig),
            Error::Ed25519Signature
        );

        // Ensure messages match
        require!(
            &signature
                .message
                .as_ref()
                .ok_or(Error::Ed25519Signature)?
                .eq(&self.round.to_slice()),
            Error::Ed25519Signature
        );

        Ok(())
    }

    pub fn play_round(&mut self, _bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        let roll = self.calculate_roll(sig);
        self.round.number = roll;
        self.update_round_outcome();
        self.update_global_state();
        // + calculate winners will be in the lib.rs file

        let mut winners_pot: u64 = 0;
        let mut winners: Vec<(Pubkey, u64)> = Vec::new();

        // Perform the transfer if the round outcome is 2
        if self.round.outcome == 2 {
            let cpi_program = self.system_program.to_account_info();

            let pot = self.vault.lamports();
            // cpi transfer from vault to house
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.house.to_account_info(),
            };

            let seeds = &[
                b"vault",
                self.round.key().as_ref(),
                //&[*bumps.get("round").unwrap()],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx: CpiContext<Transfer> =
                CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, pot)?;
        } else {
            // Iterate over the bets
            for bet_account_pubkey in self.round.bets.iter() {

                // Deserialize the Bet account data
              let bet = Bet::try_deserialize(&mut bet_account_pubkey.as_ref()).expect("Error deserializing access pda");

                if bet.bet == self.round.outcome {
                    winners_pot += bet.amount;
                    winners.push((bet_account_pubkey.key(), bet.amount));
                }
            }

        // Make a cpi transfer to each winner
        for (winner_betkey, amount) in winners.iter() {
            let cpi_program = self.system_program.to_account_info();

            let bet = Bet::try_deserialize(&mut winner_betkey.as_ref()).expect("Error deserializing access pda");

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: bet.player.to_account_info(),
            };

            let seeds = &[
                b"vault",
                self.round.key().as_ref(),
                //&[*bumps.get("round").unwrap()],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx: CpiContext<Transfer> =
                CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            // Perform the transfer
            transfer(cpi_ctx, *amount)?;

            // // Close the Bet account
            // close_account(CloseAccountContext {
            //     account: winner_pubkey.to_account_info(),
            //     destination: self.house.to_account_info(),
            //     owner: cpi_program,
            // })?;
        }

        //     // Close the Round account
        //     close_account(CloseAccountContext {
        //         account: self.round.to_account_info(),
        //         destination: self.house.to_account_info(),
        //         owner: self.system_program.to_account_info(),
        //     })?;
        // }
    }
    Ok(())
    }

    pub fn calculate_roll(&self, sig: &[u8]) -> u8 {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // produce a number 0-100
        lower.wrapping_add(upper).wrapping_rem(101) as u8
    }

    pub fn update_round_outcome(&mut self) {
        if self.round.number > self.global.number {
            self.round.outcome = 1;
        } else if self.round.number < self.global.number {
            self.round.outcome = 0;
        } else {
            self.round.outcome = 2;
        }
    }

    pub fn update_global_state(&mut self) {
        self.global.number = self.round.number;
        self.global.round += 1;
    }

    // iterate over self.round.bets vector
    // calculate the winners (internally) by comparing self.round.outcome vs each accounts bet.bet
    // calculate the round the winners_pot, each winner's payout,
    // then transfer the payout to the winner's bet from the round's vault
    // close the bet to the owner of the bet
    // first check if the round.outcome == 2, and if it is, send all the vault's balance to the house, if not, assess winners and payout equal to
    // // the players perctentage of the total winning_bets amount * the vault's balance
    // pub fn calculate_winners(&mut self) -> Result<()> {
    // let mut winners_pot: u64 = 0;
    // let mut winners: Vec<(Pubkey, u64)> = Vec::new();

    // // Perform the transfer if the round outcome is 2
    // if self.round.outcome == 2 { Ok({
    //     let cpi_program = self.system_program.to_account_info();

    //     let pot = self.vault.lamports();
    //     // cpi transfer from vault to house
    //     let cpi_accounts = Transfer {
    //         from: self.vault.to_account_info(),
    //         to: self.house.to_account_info(),
    //     };

    //     let seeds = &[
    //         b"vault",
    //         self.round.key().as_ref(),
    //         //&[*bumps.get("round").unwrap()],
    //     ];

    //     let signer_seeds = &[&seeds[..]];

    //     let cpi_ctx: CpiContext<Transfer> =
    //         CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    //     transfer(cpi_ctx, pot)?;
    // }) } else {
    //     // Iterate over the bets
    //     for bet in self.round.bets.iter() {
    //         if bet.bet == self.round.outcome {
    //             winners_pot += bet.amount;
    //             winners.push((bet.player, bet.amount));
    //         }
    //     }

    //     // Make a cpi transfer to each winner
    //     for (winner_pubkey, amount) in winners.iter() {
    //         let cpi_program = self.system_program.to_account_info();

    //         let cpi_accounts = Transfer {
    //             from: self.vault.to_account_info(),
    //             to: winner_pubkey.to_account_info(),
    //         };

    //         let seeds = &[
    //             b"vault",
    //             self.round.key().as_ref(),
    //             //&[*bumps.get("round").unwrap()],
    //         ];

    //         let signer_seeds = &[&seeds[..]];

    //         let cpi_ctx: CpiContext<Transfer> =
    //             CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    //         // Perform the transfer
    //         transfer(cpi_ctx, *amount)?;

    //         // // Close the Bet account
    //         // close_account(CloseAccountContext {
    //         //     account: winner_pubkey.to_account_info(),
    //         //     destination: self.house.to_account_info(),
    //         //     owner: cpi_program,
    //         // })?;
    //     }

    //     //     // Close the Round account
    //     //     close_account(CloseAccountContext {
    //     //         account: self.round.to_account_info(),
    //     //         destination: self.house.to_account_info(),
    //     //         owner: self.system_program.to_account_info(),
    //     //     })?;
    //     // }

    //     Ok(())
    // }
}
