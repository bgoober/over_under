// "Over / Under" is a betting game that allows users to bet on the outcome of the next random number, between 0 and 100, inclusive of 0 and 100.
// In round 1, a random number is generated. In subsequent rounds, a new random number is generated.
// Users bet on the outcome of the next random number, and whether that number will be higher or lower than the previous round's number.
// If the number is the same as the last round, the house wins the pot.
// Losers pay winners, and the house takes a cut of the winnings.

use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod errors;
use errors::Error;
mod state;
use state::*;

declare_id!("4NE4QusNajaeH8NcYXhC56jFAroM52SKkseutCuNyUBc");

#[program]
pub mod over_under {

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
        msg!("test play round instruction");
        // Verify the provided signature
        ctx.accounts.verify_ed25519_signature(&sig)?;
        msg!("Signature: {:?}", sig);

        // Play the round, which calculates the roll, updates the round number,
        // the outcome of the round, and updates global state
        ctx.accounts.play_round(&ctx.bumps, &sig)?;
        msg!("play_round Signature: {:?}", sig);

        Ok(())
    }

    pub fn assess_winners(ctx: Context<AssessWinnersC>) -> Result<()> {
        if ctx.accounts.round.outcome == 3 || ctx.accounts.round.number == 101 {
            return Err(Error::RoundNotYetPlayed.into());
        } else {
            let mut total_winners_pot = 0;
            let mut winner_accounts = Vec::new();

            let vault = ctx.accounts.vault.lamports();

            for account in ctx.remaining_accounts.iter() {
                let _account_key = account.key();
                let data = account.try_borrow_mut_data()?;

                //Deserialize the data from the account and save it in an Account variable
                let account_to_write =
                    Bet::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

                if account_to_write.bet == ctx.accounts.round.outcome {
                    total_winners_pot += account_to_write.amount;
                    winner_accounts.push((account.key(), account_to_write));
                }
            }

            // Apply collected changes outside the previous loop
            for (account, account_to_write) in winner_accounts.iter_mut() {
                let payout = (account_to_write.amount as u64 / total_winners_pot) * vault; // Ensure correct division
                account_to_write.payout = payout;

                // Find the account by account_key to serialize data back
                if let Some(account) = ctx
                    .remaining_accounts
                    .iter()
                    .find(|a| a.key() == account.key())
                {
                    let mut data = account.try_borrow_mut_data()?;
                    let _ = account_to_write.try_serialize(&mut data.as_mut());
                }
            }
        }
        Ok(())
    }

    pub fn payout(ctx: Context<PayC>) -> Result<()> {
        ctx.accounts.payout()?;
        Ok(())
    }

    pub fn close_round(ctx: Context<CloseRoundC>) -> Result<()> {
        ctx.accounts.close_round()?;
        Ok(())
    }

    pub fn close_bets1(ctx: Context<Close1BetC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets2(ctx: Context<Close2BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets3(ctx: Context<Close3BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets4(ctx: Context<Close4BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets5(ctx: Context<Close5BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets6(ctx: Context<Close6BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets7(ctx: Context<Close7BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets8(ctx: Context<Close8BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets9(ctx: Context<Close9BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }

    pub fn close_bets10(ctx: Context<Close10BetsC>) -> Result<()> {
        let _ctx = ctx;
        Ok(())
    }
}
