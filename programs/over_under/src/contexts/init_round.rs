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
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(address = global.house)]
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
    /// DOCS: mut must be placed with the vault during initRound or a non-House player will not be able to call placeBet without the House key playing the first bet of each Round (this throws an unkown action undefined error for the player). 
    /// With the mut, any player can call placeBet in a Round with 0 bets. 
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

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
            bump: *bumps.get("round").unwrap(),
            vault_bump: *bumps.get("vault").unwrap(),
        });

        msg!("round.round: {}", self.round.round);
        msg!("round.number: {}", self.round.number);
        msg!("round.outcome: {}", self.round.outcome);
        msg!("round.bets.len(): {}", self.round.bets.len());
        msg!("round.players.len(): {}", self.round.players.len());
        
        Ok(())
    }
}