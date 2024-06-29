use crate::state::Global;
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct GlobalC<'info> {
    // signer
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account()]
    pub house: SystemAccount<'info>,

    // global
    #[account(init, payer = thread, seeds = [b"global", house.key().as_ref()], space = Global::LEN, bump)]
    pub global: Account<'info, Global>,

    // system program
    pub system_program: Program<'info, System>,
}

impl<'info> GlobalC<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>,) -> Result<()> {
        self.global.set_inner( Global {
            auth: self.thread.key(),
            round: 1,
            number: 50,
            bump: *bumps.get("global").unwrap(),
        });
        Ok(())
    }
}