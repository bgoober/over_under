use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{state::{Bet, Global, Round}, errors::Error};

#[derive(Accounts)]
pub struct PayC<'info> {
    // signer
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = global.house)]
    pub house: SystemAccount<'info>,

    /// CHECK this is safe
    #[account(mut)]
    pub player1: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player2: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player3: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player4: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player5: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player6: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player7: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player8: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player9: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub player10: Option<AccountInfo<'info>>,

    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    #[account(mut,
        seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [b"vault", round.key().as_ref()],
        bump = round.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account()]
    pub bet1: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet2: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet3: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet4: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet5: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet6: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet7: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet8: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet9: Option<Box<Account<'info, Bet>>>,
    #[account()]
    pub bet10: Option<Box<Account<'info, Bet>>>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayC<'info> {
    pub fn payout(&mut self) -> Result<()> {
        if self.round.outcome == 2 {
            // make a cpi transfer from the vault to the House account for the entire vault lamports
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.house.to_account_info(),
            };

            let cpi_program = self.system_program.to_account_info();
            let seeds = &[
                b"vault",
                self.round.to_account_info().key.as_ref(),
                &[self.round.vault_bump],
            ];
            let signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer);

            let amount = self.vault.lamports();
            transfer(cpi_ctx, amount)?;
        } else if self.round.outcome == 3 || self.round.number == 101 {
            return Err(Error::RoundNotYetPlayed.into());
        } else {
            let player_bets = vec![
                (self.player1.as_ref(), self.bet1.as_ref()),
                (self.player2.as_ref(), self.bet2.as_ref()),
                (self.player3.as_ref(), self.bet3.as_ref()),
                (self.player4.as_ref(), self.bet4.as_ref()),
                (self.player5.as_ref(), self.bet5.as_ref()),
                (self.player6.as_ref(), self.bet6.as_ref()),
                (self.player7.as_ref(), self.bet7.as_ref()),
                (self.player8.as_ref(), self.bet8.as_ref()),
                (self.player9.as_ref(), self.bet9.as_ref()),
                (self.player10.as_ref(), self.bet10.as_ref()),
            ];

            for (player_option, bet_option) in player_bets.iter() {
                if let (Some(player), Some(bet)) = (player_option, bet_option) {
                    if bet.player.key() == player.key() {
                        if bet.payout > 0 {
                            let from_account_info = self.vault.to_account_info();
                            let to_account_info = player.to_account_info();
                            let amount = bet.payout;
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

                            let cpi_ctx = CpiContext::new_with_signer(
                                cpi_program.clone(),
                                cpi_accounts,
                                signer,
                            );
                            transfer(cpi_ctx, amount)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
