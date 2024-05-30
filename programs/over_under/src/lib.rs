// "Over / Under" is a betting game that allows users to bet on the outcome of the next random number, between 0 and 100, inclusive of 0 and 100.
// In round 1, a random number is generated. In subsequent rounds, a new random number is generated.
// Users bet on the outcome of the next random number, and whether that number will be higher or lower than the previous round's number.
// If the number is the same as the last round, the house wins the pot.
// Losers pay winners, and the house takes a cut of the winnings.

use anchor_lang::prelude::*;
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use std::collections::BTreeMap;
use crate::{state::Bet, errors::DiceError};

mod contexts;
use contexts::*;
mod state;
mod errors;

declare_id!("4z3ZzM7rVH8D2mBuL81TuYBtAxMrWdDziKf8Z34tLxr");

#[program]
pub mod over_under {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, bet: bool, amount: u64) -> Result<()> {
        ctx.accounts.create_bet(&ctx.bumps, seed, bet, amount)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }
}
