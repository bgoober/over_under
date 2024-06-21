use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use std::collections::BTreeMap;

use crate::state::Bet;

use crate::state::Round;

//use crate::errors::Error;
use crate::state::Global;

use crate::errors::Error;

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct BetC<'info> {
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
    pub global: Box<Account<'info, Global>>,
    
    // round the player is placing a bet in,
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump)]
    pub round: Box<Account<'info, Round>>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    // bet account to store the bet which is a pda of the round account
    #[account(init, payer = player, seeds = [b"bet", round.key().as_ref(), player.key().as_ref()], space = Bet::LEN, bump)]
    pub bet: Box<Account<'info, Bet>>,

    // system program to transfer SOL
    pub system_program: Program<'info, System>,
}

impl<'info> BetC<'info> {
    // create bet function to create a bet
    pub fn init(&mut self, amount: u64, bet: u8, round: u64, bumps: &BTreeMap<String, u8>) -> Result<()> {
        
        if self.round.outcome != 3 || self.round.number != 101 {
            return Err(Error::RoundAlreadyPlayed.into());
        } else {
        self.bet.set_inner(Bet {
            player: self.player.key(),
            amount,
            bet: bet,
            round,
            payout: 0,
            bump: *bumps.get("bet").unwrap(),
        });

        self.round.bets.push(self.bet.key());

        Ok(())
    }
}

    // upate the round.players with the player's pubkey
    // pub fn update_round_players(&mut self) -> Result<()> {
    //     if !self.round.players.contains(&self.player.key) {
    //         self.round.players.push(*self.player.key);
    //     }
    //     Ok(())
    // }

    // deposit to vault function
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(ctx, amount)
    }
}
