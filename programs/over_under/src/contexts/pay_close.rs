use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PayCloseC<'info> {
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

    #[account(mut)]
    pub bet: Account<'info, Bet>,

    // round the player is placing a bet in,
    #[account(mut, seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump, close = house)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}