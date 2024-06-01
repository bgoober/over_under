use anchor_lang::prelude::*;
use crate::state::Global;

#[derive(Accounts)]
pub struct Initialize<'info> {


    #[account(mut)]
    pub house: Signer<'info>,


    #[account(mut, seeds = [b"vault", house.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,


    #[account(init_if_needed, payer = house, seeds = [b"global", house.key().as_ref()], space = Global::LEN, bump)]
    pub global: Account<'info, Global>,


    pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.global.set_inner(Global {
            round: 1,
            number: 50,
            bump: self.global.bump,
        });
        Ok(())
    }
}
