use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::state::{Global, Round};

use crate::errors::Error;

#[derive(Accounts)]
pub struct CloseRoundC<'info> {
    // signer
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = global.house)]
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
    pub round: Box<Account<'info, Round>>,

    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    // system program
    pub system_program: Program<'info, System>,
}

impl<'info> CloseRoundC<'info> {
    pub fn close_round(&mut self) -> Result<()> {
        // Check if the round can be closed based on its outcome, number, or bets.
        if self.round.outcome == 3 || self.round.number == 101 {
            return Err(Error::RoundStillOngoing.into());
        }

        // Check if the current round matches the global round.
        if self.round.round != self.global.round {
            return Err(Error::RoundMismatch.into());
        }

        // If the vault has a balance, transfer it to the house.
        if self.vault.lamports() > 0 {
            let from_account_info = self.vault.to_account_info();
            let to_account_info = self.house.to_account_info();
            let amount = self.vault.lamports();
            let cpi_accounts = Transfer {
                from: from_account_info.clone(),
                to: to_account_info,
            };

            let cpi_program = self.system_program.to_account_info();
            let seeds = &[
                b"vault",
                self.round.to_account_info().key.as_ref(),
                &[self.round.vault_bump],
            ];
            let signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer);
            transfer(cpi_ctx, amount)?;
        }

        self.global.round += 1;
        self.global.number = self.round.number;

        Ok(())
    }
}
