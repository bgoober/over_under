use std::collections::BTreeMap;

use anchor_lang::prelude::*;

use crate::{state::Bet, state::Global, state::Round};


#[derive(Accounts)]
pub struct ResolveBetC<'info> {
    // thread is signer
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut)]
    pub player: SystemAccount<'info>,

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

    #[account(mut, close = player, seeds = [b"bet", vault.key().as_ref()], bump = bet.bump)]
    pub bet: Account<'info, Bet>,

    #[account(address = solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

impl <'info> ResolveBetC<'info> {
    pub fn resolve_bet(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        // if the self.bet.round is the same as the round.number, then continue, else error
        // require!((self.bet.round == self.global.round), Error::RoundNotOver);
        Ok(())

    }
}