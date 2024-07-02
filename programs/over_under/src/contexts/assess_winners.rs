use anchor_lang::prelude::*;

use crate::state::{Global, Round};

#[derive(Accounts)]
pub struct AssessWinnersC<'info> {
    // signer
    #[account(mut, address = global.auth)]
    pub thread: Signer<'info>,

    // house
    #[account(mut)]
    pub house: SystemAccount<'info>,

    // global account
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round account
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump = round.bump)]
    pub round: Box<Account<'info, Round>>,

    // vault account
    #[account(seeds = [b"vault", round.key().as_ref()], bump = round.vault_bump)]
    pub vault: SystemAccount<'info>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    // system program
    pub system_program: Program<'info, System>,
}
