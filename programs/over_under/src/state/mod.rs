use anchor_lang::prelude::*;

/// There are 3 Accounts possible: Global, representing the global state of the game.
/// /// Round, representing a round of the game.
/// Bet, representing a bet placed by a player.
/// 
/// The Global account stores the current round number, the previous round number, and the previous round's randomly generated number, and the bump used to generate the global PDA.
/// The Round account stores its round number, the randomly generated number of the round, the players that placed a bet in the round, and the bump used to generate the round PDA.
/// A bet account stores the player's bet, the amount the player bet in SOL, the round the bet was placed in, and the bump used to generate the bet PDA.


// #[account]
#[account]
pub struct Global {
    pub auth: Pubkey, // the pubkey of the signer of init global
    pub round: u64, // to store the global round
    pub number: u8, // to store the random number of the previous round
    pub bump: u8, // the bump used to generate the global PDA
}

impl Global {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;
}

#[account]
pub struct Round {
    pub round: u64, // the round number
    pub number: u8, // the random number of the round
    pub bets: Vec<Pubkey>, // the players that placed a bet in the round
    pub outcome: u8, // the outcome of the user's bet vs the number drawn. evaluated and updated in resolve round
    pub bump: u8, // the bump used to generate the round PDA
}

impl Round { 
    pub const LEN: usize = 8+8+1+4+(32*100)+1+1;

    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.round.to_le_bytes().to_vec();
        s.extend_from_slice(&self.number.to_le_bytes());
        s.extend_from_slice(&self.outcome.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());
        s        
    }
}

#[account]
pub struct Bet {
    pub player: Pubkey, // the player who placed the bet
    pub bet: u8, // the player's bet, true if the player bet over, false if the player bet under
    pub amount: u64, // the amount the player bet in SOL
    pub round: u64, // the round the bet was placed in
    pub payout: u64, // the payout of the bet, calculated in resolve round
    pub bump : u8 // the bump used to generate the bet PDA
}

impl Bet {
    pub const LEN: usize = 8+32+1+8+8+1+1;
}

