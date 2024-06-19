// "Over / Under" is a betting game that allows users to bet on the outcome of the next random number, between 0 and 100, inclusive of 0 and 100.
// In round 1, a random number is generated. In subsequent rounds, a new random number is generated.
// Users bet on the outcome of the next random number, and whether that number will be higher or lower than the previous round's number.
// If the number is the same as the last round, the house wins the pot.
// Losers pay winners, and the house takes a cut of the winnings.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};
use std::collections::HashMap;

mod contexts;
use contexts::*;
mod errors;
mod state;
use state::*;

declare_id!("4z3ZzM7rVH8D2mBuL81TuYBtAxMrWdDziKf8Z34tLxr");

#[program]
pub mod over_under {

    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn init_global(ctx: Context<GlobalC>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

    pub fn init_round(ctx: Context<RoundC>, _round: u64) -> Result<()> {
        ctx.accounts.init(_round, &ctx.bumps)?;
        Ok(())
    }

    pub fn place_bet(ctx: Context<BetC>, amount: u64, bet: u8, round: u64) -> Result<()> {
        ctx.accounts.init(amount, bet, round, &ctx.bumps)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn play_round(ctx: Context<PlayRoundC>, sig: Vec<u8>) -> Result<()> {
        // Verify the provided signature
        ctx.accounts.verify_ed25519_signature(&sig)?;
    
        // Play the round, which calculates the roll, updates the round number, 
        // the outcome of the round, and updates global state
        ctx.accounts.play_round(&ctx.bumps, &sig)?;
    
        // Get the total SOL amount in the vault
        let pot = ctx.accounts.vault.lamports();
    
        // Initialize the total amount won and the winners
        let mut winners_pot: u64 = 0;
        let mut winners = HashMap::new();
    
        // Get the total number of accounts
        let total_accounts: usize = ctx.remaining_accounts.len();
        msg!("Total Remaining Accounts: {}", total_accounts);
    
        // Calculate the total amount won and the winners
        for account in ctx.remaining_accounts.iter() {
            let _account_key = account.key();
            let data = account.try_borrow_mut_data();
            let account_to_write = Bet::try_deserialize(&mut data.unwrap().as_ref())
                .expect("Error Deserializing Bet Account Data to Calculate Winners");
    
            // If the bet matches the outcome, add the amount to the total won and add the player to the winners
            if account_to_write.bet == ctx.accounts.round.outcome {
                winners_pot += account_to_write.amount;
                winners.insert(account_to_write.player, account_to_write.amount);
            }
        }
    
        // Update the round's winners with the winners and their payouts
        for (pubkey, bet_amount) in &winners {
            let payout = (*bet_amount / winners_pot) * pot;
            ctx.accounts.round.winners.push((*pubkey, payout));
        }
    
        // Get the system program, round, and vault account info
        let system_program_info = ctx.accounts.system_program.to_account_info();
        let cpi_program = system_program_info;
        let round = ctx.accounts.round.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();
    
        // Pay the winners and close the accounts
        for account in ctx.remaining_accounts.iter() {
            let data = account.try_borrow_mut_data()?;
            let account_to_write = Bet::try_deserialize(&mut data.as_ref())
                .expect("Error Deserializing Bet Account Data to Pay Winners");
    
            // If the player is a winner, find their account info
            if let Some((winner_pubkey, payout)) = ctx
                .accounts
                .round
                .winners
                .iter()
                .find(|(pubkey, _)| *pubkey == account_to_write.player)
            {
                // If the winner's account info is found, prepare to transfer the payout
                if let Some(winner_account_info) = ctx
                    .remaining_accounts
                    .iter()
                    .find(|account_info| *account_info.key == *winner_pubkey)
                {
                    // Set up the transfer accounts
                    let cpi_accounts = Transfer {
                        from: vault.to_account_info(),
                        to: winner_account_info.to_account_info(),
                    };
    
                    // Set up the signer seeds
                    let seeds = &[b"vault", round.key.as_ref(), &[ctx.accounts.round.bump]];
                    let signer_seeds = &[&seeds[..]];
    
                    // Create the context for the transfer
                    let cpi_ctx = CpiContext::new_with_signer(
                        cpi_program.clone(),
                        cpi_accounts,
                        signer_seeds,
                    );
    
                    // Perform the transfer
                    transfer(cpi_ctx, *payout)?;
                } 
            }
        }
    
        // Return Ok if everything went well
        Ok(())
    }
                
}
