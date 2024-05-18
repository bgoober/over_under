// "Over / Under" is a betting game that allows users to bet on the outcome of the next random number, between 0 and 100, inclusive of 0 and 100.
// In round 1, a random number is generated. In subsequent rounds, a new random number is generated.
// Users bet on the outcome of the next random number, and whether that number will be higher or lower than the previous round's number.
// If the number is the same as the last round, the house wins the pot.
// Losers pay winners, and the house takes a cut of the winnings.

use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use rand::Rng;

declare_id!("4z3ZzM7rVH8D2mBuL81TuYBtAxMrWdDziKf8Z34tLxr");

#[program]
pub mod over_under {
    use super::*;

    const house: &'static str = "house pubkey";

    // init_game creates the game with its base settings
    pub fn init_global(ctx: Context<Global>) -> Result<()> {
        ctx.accounts.global.set_inner(GlobalAccount {
            round_number: 1,
            random_number: 50,
        });

        Ok(())
    }

    // init_round allows the program to create a new round at the end of each round
    pub fn init_round(ctx: Context<Round>) -> Result<()> {
        ctx.accounts.round.set_inner(RoundAccount {
            pot: ctx.accounts.round.pot.key(),
            round_number: ctx.accounts.global.round_number + 1,
            random_number: ctx.accounts.global.random_number,
            players: Vec::new(),
            outcome: "".to_string(),
            payout: Vec::new(),
        });

        // generate a random number between 0 and 100, inclusive of 0 and 100, and set it to the round's random_number
        let mut rng = rand::rngs::OsRng;
        let random_number = rng.gen_range(0..101);

        ctx.accounts.round.random_number = random_number;

        // compare the random_number to the global random_number: if higher, set outcome to "over", if lower, set outcome to "under", if same, set outcome to "same"
        if random_number > ctx.accounts.global.random_number {
            ctx.accounts.round.outcome = "over".to_string();
        } else if random_number < ctx.accounts.global.random_number {
            ctx.accounts.round.outcome = "under".to_string();
        } else {
            ctx.accounts.round.outcome = "same".to_string();
        }

        // increment the global round_number
        ctx.accounts.global.round_number += 1;

        // iterate over the round's players and set their winner status based on the outcome. If the player's bet matches the outcome, set winner to true, else set winner to false.

        let mut new_players = Vec::new();
        let mut new_payout = Vec::new();
        
        for player in ctx.accounts.round.players.iter() {
            let mut new_player = player.clone();
            if (new_player.bet == true && ctx.accounts.round.outcome == "over")
                || (new_player.bet == false && ctx.accounts.round.outcome == "under")
            {
                new_player.winner = true;
                new_payout.push(new_player.clone());
            } else {
                new_player.winner = false;
            }
            new_players.push(new_player);
        }
        
        ctx.accounts.round.players = new_players;
        ctx.accounts.round.payout = new_payout;
        
        for player in ctx.accounts.round.players.iter() {
            calculate_winnings_and_update_payout(player, &mut ctx.accounts.round);
        }

        fn calculate_winnings_and_update_payout(player: &Player, round: &mut RoundAccount) {
            if player.winner {
                let total_bet_amount: u64 = round.players.iter().map(|player| player.bet_amount).sum();
                let winnings =
                    total_bet_amount * (player.bet_amount as f64 / total_bet_amount as f64) as u64;
                if winnings > 0 {
                    round.payout.push(Player {
                        player: player.player,
                        bet: player.bet,
                        bet_amount: player.bet_amount,
                        winner: player.winner,
                        winnings,
                    });
                }
            }
        }

        // payout allows the program to payout the winners of each round
        pub fn payout(ctx: Context<Round>) -> Result<()> {
            // pay out the winners of the previous round by reading the RoundAccount's payout Vec and transferring the winnings to the player's account
            let round = ctx.accounts.round.clone();

            for player in ctx.accounts.round.payout.iter() {
                // transfer the player's winnings to the player's account
                ctx.accounts
                    .round
                    .pot
                    .transfer(player.player, player.winnings);
            }

            Ok(())
        }

        // call payout to pay out the winners of the previous round
        payout(ctx)?;

        Ok(())
    }

    // place_bet allows a player to place a bet in a given round
    pub fn place_bet(ctx: Context<PlaceBet>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Global<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, seeds = [b"global"], bump, payer = payer, space = GlobalAccount::LEN)]
    pub global: Account<'info, GlobalAccount>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Round<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, seeds = [b"round"], bump, payer = payer, space = RoundAccount::LEN)]
    pub round: Account<'info, RoundAccount>,
    #[account(mut)]
    pub global: Account<'info, GlobalAccount>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, seeds = [b"player"], bump, payer = payer, space = Player::LEN)]
    player: Account<'info, Player>,
    #[account(mut)]
    pub round: Account<'info, RoundAccount>,
    pub system_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct Payout<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(init, seeds = [b"payout"], bump, payer = payer, space = PayoutAccount::LEN)]
//     pub pay_out: Account<'info, PayoutAccount>,
//     #[account(mut)]
//     pub round: Account<'info, RoundAccount>,
//     pub system_program: AccountInfo<'info>,
// }

#[account]
pub struct GlobalAccount {
    round_number: u128,
    random_number: u128,
}

impl GlobalAccount {
    pub const LEN: usize = 16 + 16; // u128 space + u128 space
}

#[account]
// derive
pub struct RoundAccount {
    // the pot is the address of the round's PDA, to store the bets as the pot, and handle disbursement at the end of the round
    pub pot: Pubkey,
    pub round_number: u128,   // the current round's sequential number
    pub random_number: u128,  // the random number generated for this round
    pub players: Vec<Player>, // the players who have placed bets in this round
    pub outcome: String,      // "over", "under", "same"
    pub payout: Vec<Player>,  // the disbursements made to the winners
}

impl RoundAccount {
    pub const LEN: usize = 8 + 16 + 16 + (4 + (32 + 1 + 8)) + 4 + (4 + (32 + 8));
}

#[account]
pub struct Player {
    pub player: Pubkey,  // the player's public key
    pub bet: bool,       // true or false
    pub bet_amount: u64, // the bet_amount is in lamports of SOL
    pub winner: bool,    // true or false
    pub winnings: u64,   // the winnings is in lamports of SOL for this player
}

impl Player {
    pub const LEN: usize = 32 + 1 + 1 + 8 + 8; // 32 bytes for the player's public key + 1 byte for the bet + 8 bytes for the bet_amount
}

// // #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
// #[account]
// pub struct PayoutAccount {
//     pub player: Pubkey, // the player's public key
//     pub amount: u64, // the amount is in lamports of SOL
// }

// impl PayoutAccount {
//     pub const LEN: usize = 32 + 8; // 32 bytes for the player's public key + 8 bytes for the amount
// }

// bool: 1 byte (although it only requires 1 bit)
// u8/i8: 1 byte
// u16/i16: 2 bytes
// u32/i32: 4 bytes
// u64/i64: 8 bytes
// u128/i128: 16 bytes
// Pubkey: 32 bytes
// Vec<T>: 4 bytes + space for elements (space(T) * number of elements)
// String: 4 bytes + length of string in bytes
// Option<T>: 1 byte + space(T)
// Enum: 1 byte + space of largest variant
