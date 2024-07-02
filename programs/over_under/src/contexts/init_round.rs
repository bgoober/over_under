use anchor_lang::prelude::*;
use std::collections::BTreeMap;

use crate::state::Round;

use crate::state::Global;


// in the InitRound context, we initialize a round that is a pda of the global account.
// then, we derive a vault that is a pda of the round account, which users will place their SOL bets into
#[derive(Accounts)]
#[instruction(_round: u64)]
pub struct RoundC<'info> {
    // signer
    #[account(mut, constraint = thread.key() == global.auth)]
    pub thread: Signer<'info>,

    #[account()]
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    // round pda of the global account
    #[account(init, payer = thread, seeds = [b"round", global.key().as_ref(), _round.to_le_bytes().as_ref()], space = Round::LEN, bump)]
    pub round: Box<Account<'info, Round>>,

    // vault pda of the round account
    #[account(seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl <'info> RoundC<'info> {
    // create round function to create a round
    pub fn init(&mut self, _round: u64, bumps: &BTreeMap<String, u8>) -> Result<()> {
        self.round.set_inner(Round {
            round: _round,
            number: 101, 
            outcome: 3,
            bets: Vec::with_capacity(10),
            players: Vec::with_capacity(10),
            randomness_account: Pubkey::default(),
            bump: *bumps.get("round").unwrap(),
            vault_bump: *bumps.get("vault").unwrap(),
        });

        msg!("round.round: {}", self.round.round);
        msg!("round.number: {}", self.round.number);
        msg!("round.outcome: {}", self.round.outcome);
        msg!("round.bets.len(): {}", self.round.bets.len());
        
        Ok(())
    }
}