use anchor_lang::prelude::*;

use crate::state::{Global, Round};

#[derive(Accounts)]
pub struct AssessWinnersC<'info> {
    // signer
    #[account(mut)]
    pub thread: Signer<'info>,

    // house
    #[account(mut, constraint = house.key() == global.auth.key())]
    pub house: SystemAccount<'info>,

    // global account
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round account
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump)]
    pub round: Account<'info, Round>,

    // vault account
    #[account(seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    // system program
    pub system_program: Program<'info, System>,
}
