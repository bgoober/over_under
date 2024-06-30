use anchor_lang::prelude::*;

use crate::state::Bet;

#[derive(Accounts)]
pub struct CloseBetC<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, address = bet.player)]    
    pub player: AccountInfo<'info>,
    
    #[account(mut, has_one = player, close = player)]
    pub bet: Account<'info, Bet>
}
