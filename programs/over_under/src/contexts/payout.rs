use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{Bet, Global, Round};

#[derive(Accounts)]
pub struct PayC<'info> {
    #[account(mut, constraint = player.key() == bet.player.key())]
    pub player: Signer<'info>,

    #[account(constraint = house.key() == global.auth.key())]
    pub house: SystemAccount<'info>,

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
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut, close = player,
        seeds = [b"bet", round.key().as_ref(), player.key().as_ref()],
        bump, has_one = player
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayC<'info> {
    pub fn payout(&mut self) -> Result<()> {
        if self.bet.payout > 0 && self.player.key() == self.bet.player.key() {
            let amount = self.bet.payout;
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let seeds = &[
                b"vault",
                self.round.to_account_info().key.as_ref(),
                &[self.round.vault_bump],
            ];
    
            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, amount)?;
        }

        self.bet.close(self.player.to_account_info())?;
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct PayC<'info> {
//     #[account(mut, constraint = player.key() == bet.player.key())]
//     pub player: Signer<'info>,

//     /// CHECK this is safe
//     #[account(mut)]
//     pub player2: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player3: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player4: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player5: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player6: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player7: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player8: Option<AccountInfo<'info>>,
//     #[account(mut)]
//     pub player9: Option<AccountInfo<'info>>,

//     #[account(constraint = house.key() == global.auth.key())]
//     pub house: SystemAccount<'info>,

//     #[account(
//         seeds = [b"global", house.key().as_ref()],
//         bump
//     )]
//     pub global: Account<'info, Global>,

//     #[account(
//         seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub round: Account<'info, Round>,

//     #[account(mut,
//         seeds = [b"vault", round.key().as_ref()],
//         bump
//     )]
//     pub vault: SystemAccount<'info>,

//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player.key().as_ref()],
//         bump, 
//     )]
//     pub bet: Account<'info, Bet>,

//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player2.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet2: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player3.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet3: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player4.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet4: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player5.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet5: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player6.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet6: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player7.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet7: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player8.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet8: Option<Account<'info, Bet>>,
//     #[account(mut,
//         seeds = [b"bet", round.key().as_ref(), player9.clone().unwrap().key().as_ref()],
//         bump, 
//     )]
//     pub bet9: Option<Account<'info, Bet>>,

//     pub system_program: Program<'info, System>,
// }

// impl<'info> PayC<'info> {
//     pub fn payout(&mut self) -> Result<()> {
//         let players = [
//             (self.player.to_account_info(), Some(&self.bet)),
//             (self.player2.clone().unwrap().to_account_info(), self.bet2.as_ref()),
//             (self.player3.clone().unwrap().to_account_info(), self.bet3.as_ref()),
//             (self.player4.clone().unwrap().to_account_info(), self.bet4.as_ref()),
//             (self.player5.clone().unwrap().to_account_info(), self.bet5.as_ref()),
//             (self.player6.clone().unwrap().to_account_info(), self.bet6.as_ref()),
//             (self.player7.clone().unwrap().to_account_info(), self.bet7.as_ref()),
//             (self.player8.clone().unwrap().to_account_info(), self.bet8.as_ref()),
//             (self.player9.clone().unwrap().to_account_info(), self.bet9.as_ref()),
//         ];

//         let cpi_program = self.system_program.to_account_info();
//         let from_account_info = self.vault.to_account_info();
//         let seeds = &[
//             b"vault",
//             self.round.to_account_info().key.as_ref(),
//             &[self.round.vault_bump],
//         ];
//         let signer_seeds = &[&seeds[..]];

//         for (player_account_info, bet_option) in players.iter() {
//             if let Some(bet) = bet_option {
//                 if bet.payout > 0 && player_account_info.key == &bet.player.key() {
//                     let amount = bet.payout;
//                     let cpi_accounts = Transfer {
//                         from: from_account_info.clone(),
//                         to: player_account_info.clone(),
//                     };
//                     let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer_seeds);
//                     transfer(cpi_ctx, amount)?;
//                     bet.close(player_account_info.clone())?;
//                 }
//             }
//         }

//         Ok(())
//     }
// }