use anchor_lang::prelude::*;
use solana_program::instruction;
use std::collections::BTreeMap;

use crate::state::Round;

use crate::state::Global;


// in the InitRound context, we initialize a round that is a pda of the global account.
// then, we derive a vault that is a pda of the round account, which users will place their SOL bets into
#[derive(Accounts)]
#[instruction(_round: u64)]
pub struct RoundC<'info> {
    // thread
    #[account(mut)]
    pub thread: Signer<'info>,

    // the pubkey of the signer of init global
    #[account(mut)]
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        mut,
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round pda of the global account
    #[account(init, payer = thread, seeds = [b"round", global.key().as_ref(), _round.to_le_bytes().as_ref()], space = Round::LEN, bump)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> RoundC<'info> {
    // create round function to create a round
    pub fn init(&mut self, _round: u64, bumps: &BTreeMap<String, u8>) -> Result<()> {
        self.round.set_inner(Round {
            round: _round,
            number: 0,
            outcome: 0, // 0 for false, 1 for true, 2 for tie
            bump: bumps["round"]
        });
        Ok(())
    }
}