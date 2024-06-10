use std::collections::BTreeMap;

use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_instruction_sysvar::{Ed25519InstructionSignatures, InstructionSysvar};
use solana_program::{sysvar::instructions::load_instruction_at_checked, ed25519_program, hash::hash};

use crate::{state::Bet, errors::OUError, state::Global, state::Round};


pub const HOUSE_EDGE: u16 = 150; // 1.5% House edge

#[derive(Accounts)]
pub struct PlayRoundC<'info> {
    #[account(mut)]
    thread: Signer<'info>,

    // the pubkey of the signer of init global
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

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
    pub instruction_sysvar: AccountInfo<'info>,

    
    pub system_program: Program<'info, System>
}

impl<'info> PlayRoundC<'info> {

    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction 
        let ix = load_instruction_at_checked(
            0, 
            &self.instruction_sysvar.to_account_info()
        )?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, OUError::Ed25519Program);
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, OUError::Ed25519Accounts);
        
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, OUError::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, OUError::Ed25519Header);
        
        // Ensure public keys match
        require_keys_eq!(signature.public_key.ok_or(OUError::Ed25519Pubkey)?, self.house.key(), OUError::Ed25519Pubkey);

        // Ensure signatures match
        require!(&signature.signature.ok_or(OUError::Ed25519Signature)?.eq(sig), OUError::Ed25519Signature);

        // Ensure messages match
        require!(&signature.message.as_ref().ok_or(OUError::Ed25519Signature)?.eq(&self.bet.to_slice()), OUError::Ed25519Signature);

        Ok(())
    }

    pub fn play_round(&mut self, bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8;16] = [0;16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);
        
        let roll = lower
            .wrapping_add(upper)
            .wrapping_rem(100) as u8 + 1;

        // updat the round number to roll
        self.round.number = roll;        

        // if self.global.number < roll {

        // if the round.number is greater than global.number, round.outcome is 1. If round.number < global.number then round.outcome is 0. 
        // If round.number == global.number, then round.outcome is 2.
        if self.round.number > self.global.number {
            self.round.outcome = 1;
        } else if self.round.number < self.global.number {
            self.round.outcome = 0;
        } else {
            self.round.outcome = 2;
        }


        self.global.number = roll;
        self.global.round += 1;

        

        Ok(())
    }
}
// TODO: put the resolve_bet function on an internal timer using the slot number to call the function, then init a new round.
