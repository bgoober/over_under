use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{Bet, Global, Round};

#[derive(Accounts)]
pub struct PayC<'info> {
    // signer
    #[account(mut, constraint = thread.key() == global.auth.key())]
    pub thread: Signer<'info>,

    #[account()]
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

    #[account(
        seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,

    #[account(mut,
        seeds = [b"vault", round.key().as_ref()],
        bump = round.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet1.player.key().as_ref()],
        bump, close = player1
    )]
    pub bet1: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet2.player.key().as_ref()],
        bump, close = player2
    )]
    pub bet2: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet3.player.key().as_ref()],
        bump, close = player3
    )]
    pub bet3: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet4.player.key().as_ref()],
        bump, close = player4
    )]
    pub bet4: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet5.player.key().as_ref()],
        bump, close = player5
    )]
    pub bet5: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet6.player.key().as_ref()],
        bump, close = player6
    )]
    pub bet6: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet7.player.key().as_ref()],
        bump, close = player7
    )]
    pub bet7: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet8.player.key().as_ref()],
        bump, close = player8
    )]
    pub bet8: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet9.player.key().as_ref()],
        bump, close = player9
    )]
    pub bet9: Option<Account<'info, Bet>>,

    #[account(mut,
        seeds = [b"bet", round.key().as_ref(), bet10.player.key().as_ref()],
        bump, close = player10
    )]
    pub bet10: Option<Account<'info, Bet>>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayC<'info> {
    pub fn payout(&mut self) -> Result<()> {
        let bet_accounts = [
            self.bet1.as_ref(),
            self.bet2.as_ref(),
            self.bet3.as_ref(),
            self.bet4.as_ref(),
            self.bet5.as_ref(),
            self.bet6.as_ref(),
            self.bet7.as_ref(),
            self.bet8.as_ref(),
            self.bet9.as_ref(),
            self.bet10.as_ref(),
        ];

        msg!("global auth key: {}", self.global.auth.key());


        for bet_option in bet_accounts.iter() {
            if let Some(bet) = bet_option {
                if bet.payout > 0 {
                    // Assuming `bet.payout` is the amount to transfer back to the bet account
                    let from_account_info = self.vault.to_account_info();
                    let to_account_info = bet.to_account_info();
                    let amount = bet.payout;
                    let cpi_accounts = Transfer {
                        from: from_account_info.clone(),
                        to: to_account_info,
                    };

                    let cpi_program = self.system_program.to_account_info();
                    let seeds = &[b"vault", self.round.to_account_info().key.as_ref()];

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

        Ok(())
    }
}
