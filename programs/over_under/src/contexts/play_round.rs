use std::collections::BTreeMap;

use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::prelude::*;
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::{
    errors::Error,
    state::{Global, Round},
};

#[derive(Accounts)]
pub struct PlayRoundC<'info> {
    #[account(mut)]
    thread: Signer<'info>,

    // the pubkey of the signer of init global
    #[account()]
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round the player is placing a bet in,
    #[account(mut, seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump = round.bump)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump = round.vault_bump)]
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
                .starts_with(&self.round.to_slice()), // making comparison of the round slice to the message signature
            Error::Ed25519Signature
        );

        Ok(())
    }

    pub fn play_round(&mut self, _bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        msg!(&format!("round.bets.len(): {:#?}", self.round.bets));
        if self.round.bets.len() == 0 {
            return Err(Error::NoBetsInRound.into());
        } else {
            let hash = hash(sig).to_bytes();
            let mut hash_16: [u8; 16] = [0; 16];
            hash_16.copy_from_slice(&hash[0..16]);
            let lower = u128::from_le_bytes(hash_16);
            hash_16.copy_from_slice(&hash[16..32]);
            let upper = u128::from_le_bytes(hash_16);

            // produce a number 0-100
            let roll = lower.wrapping_add(upper).wrapping_rem(101) as u8;

            msg!("Roll: {:?}", roll);

            self.round.number = roll;

            {
                if self.round.number > self.global.number {
                    self.round.outcome = 1; // number was higher
                    self.global.number = roll;
                } else if self.round.number < self.global.number {
                    self.round.outcome = 0; // number was lower
                    self.global.number = roll;
                } else if self.round.number == self.global.number {
                    self.round.outcome = 2; // number was the same
                    self.global.number = roll;
                }
            }

            msg!("Round Outcome: {:?}", self.round.outcome);
            msg!("Round Number: {:?}", self.round.number);
            msg!("Global Number: {:?}", self.global.number);
            msg!("Global Round: {:?}", self.global.round);

            Ok(())
        }
    }
}
