use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::{Bet, Global, Round};

// use Transfer and transfer

#[derive(Accounts)]
pub struct PayC<'info> {
    #[account(mut, constraint = player.key() == bet.player.key())]
    pub player: Signer<'info>,

    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    #[account(mut, close = house,
        seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(mut,
        seeds = [b"vault", round.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut, close = player,
        seeds = [b"bet", round.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayC<'info> {
    pub fn pay(&mut self) -> Result<()> {
        // if the round.outcome = 2, then send all lamports in the vault to the house
        // else if the signer is equal to the bet.player key, then transfer the bet.payout to the player
        if self.round.outcome == 2 {
            let amount = self.vault.lamports();
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.house.to_account_info(),
            };

            let seeds = &[b"vault", self.round.to_account_info().key.as_ref(),
            ];

            let signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

            transfer(cpi_ctx, amount)?;

        } else if self.bet.payout > 0 && self.player.key() == self.bet.player.key() {

            let amount = self.bet.payout;
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
    
            let seeds = &[b"vault", self.round.to_account_info().key.as_ref(),
            ];

            let signer = &[&seeds[..]];
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
            transfer(cpi_ctx, amount)?;
    
        }
        Ok(())
    }
}
