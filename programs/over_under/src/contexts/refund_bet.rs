use std::collections::BTreeMap;

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{Bet, Global, Round};

use crate::errors::OUError;

#[derive(Accounts)]
pub struct RefundC<'info> {
    // player who is signer
    #[account(mut)]
    pub player: Signer<'info>,

    // the pubkey of the signer of init global
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(

        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round the player is placing a bet in,
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    // bet account to store the bet which is a pda of the round account
    #[account(mut, close = player, seeds = [b"bet", round.key().as_ref()], bump)]
    pub bet: Account<'info, Bet>,

    // system program to transfer SOL
    pub system_program: Program<'info, System>,
}

impl<'info> RefundC<'info> {
    pub fn refund_bet(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [
            b"vault",
            &self.house.key().to_bytes()[..],
            &[*bumps.get("vault").ok_or(OUError::BumpError)?],
        ];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(ctx, self.bet.amount)
    }
}
