use anchor_lang::prelude::*;

use crate::state::{Global, Round};

use crate::errors::Error;

#[derive(Accounts)]
pub struct CloseRoundC<'info> {
    // signer
    #[account(mut, address = global.auth)]
    pub thread: Signer<'info>,

    #[account(mut)]
    pub house: SystemAccount<'info>,

    // global as ref
    #[account(mut,
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round to close
    #[account(mut, close = house,
        seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
        bump = round.bump
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
        } else if self.round.round == self.global.round {
            self.global.round += 1;
            self.global.number = self.round.number;

            self.round.close(self.house.to_account_info())?;
        } else {
            return Err(Error::RoundMismatch.into());
        }

        Ok(())
    }
}
