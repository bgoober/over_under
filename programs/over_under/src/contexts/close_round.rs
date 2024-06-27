use anchor_lang::prelude::*;

use crate::state::{Global, Round};

use crate::errors::Error;

#[derive(Accounts)]
pub struct CloseRoundC<'info> {
    // signer
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, constraint = house.key() == global.auth.key())]
    pub house: SystemAccount<'info>,

    // global as ref
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round to close
    #[account(mut, close = house,
        seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    // system program
    pub system_program: Program<'info, System>,
}

impl<'info> CloseRoundC<'info> {
    pub fn close_round(&mut self) -> Result<()> {
        // check if the self.round.outcome is 3, or if the self.round.bets.len() is 0, or the self.round.number is 101, and if any of these are true, return an error that the round is still ongoing and can't be closed, else close the round
        if self.round.outcome == 3 || self.round.number == 101 {
            return Err(Error::RoundStillOngoing.into());
        } else {
            self.round.close(self.house.to_account_info())?;
        }

        Ok(())
    }
}
