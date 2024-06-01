use std::collections::BTreeMap;

use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::state::Global;
use crate::{errors::DiceError, state::Bet};

//pub const HOUSE_EDGE: u16 = 150; // 1.5% House edge

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    // The global account stores the round number and the number of the previous round and we will change these values
    #[account(mut, seeds = [b"global", house.key().as_ref()], bump)]
    pub global: Account<'info, Global>,

    #[account(mut)]
    ///CHECK: This is safe
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is safe
    pub instruction_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(
            ix.program_id,
            ed25519_program::ID,
            DiceError::Ed25519Program
        );
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, DiceError::Ed25519Accounts);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, DiceError::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, DiceError::Ed25519Header);

        // Ensure public keys match
        require_keys_eq!(
            signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,
            self.house.key(),
            DiceError::Ed25519Pubkey
        );

        // Ensure signatures match
        require!(
            &signature
                .signature
                .ok_or(DiceError::Ed25519Signature)?
                .eq(sig),
            DiceError::Ed25519Signature
        );

        // Ensure messages match
        require!(
            &signature
                .message
                .as_ref()
                .ok_or(DiceError::Ed25519Signature)?
                .eq(&*self.bet.to_slice()),
            DiceError::Ed25519Signature
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // Calculate the roll as a number between 0 and 100.
        let roll = lower.wrapping_add(upper).wrapping_rem(101) as u8;

        let total_winning_bets: u64 = self
            .global
            .players
            .iter()
            .filter(|player_info| {
                // Load the player's Bet account
                let player_bet: Bet = Bet::try_from_slice(&player_info.as_ref()).unwrap(); // Note: You might want to handle this unwrap in a way that suits your program

                // Determine if the player won based on their bet and the roll.
                player_bet.bet && roll > self.global.number
                    || !player_bet.bet && roll < self.global.number
            })
            .map(|player_info| {
                let player_bet: Bet = Bet::try_from_slice(&player_info.as_ref()).unwrap();
                player_bet.amount
            })
            .sum();

        for player_info in &self.global.players {
            // Load the player's Bet account
            let player_bet: Bet = Bet::try_from_slice(&player_info.as_ref()).unwrap(); // Note: You might want to handle this unwrap in a way that suits your program

            let player_won = player_bet.bet && roll > self.global.number
                || !player_bet.bet && roll < self.global.number;

            let total_bets_amount_sum: u64 = self
                .global
                .players
                .iter()
                .map(|player_info| {
                    let player_bet: Bet = Bet::try_from_slice(&player_info.as_ref()).unwrap();
                    player_bet.amount
                })
                .sum();

            if player_won {
                let payout = (player_bet.amount as u128)
                    .checked_mul(total_bets_amount_sum.into())
                    .ok_or(DiceError::Overflow)?
                    .checked_div(total_winning_bets as u128)
                    .ok_or(DiceError::Overflow)? as u64;

                let accounts = Transfer {
                    from: self.vault.to_account_info(),
                    to: self.player.to_account_info(),
                };

                let seeds = [
                    b"vault",
                    &self.house.key().to_bytes()[..],
                    &[*bumps.get("vault").ok_or(DiceError::BumpError)?],
                ];
                let signer_seeds = &[&seeds[..]][..];

                let ctx = CpiContext::new_with_signer(
                    self.system_program.to_account_info(),
                    accounts,
                    signer_seeds,
                );
                transfer(ctx, payout)?;
            }
        }

        self.global.number = roll;

        Ok(())
    }
}

// TODO: put the resolve_bet function on an internal timer using the slot number to call the function, then init a new round.
