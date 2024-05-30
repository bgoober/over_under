use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    pub player: Pubkey, //  the player's public key
    pub bet: bool, // the player's bet, true if the player bet over, false if the player bet under
    pub amount: u64, // the amount the player bet in SOL
    pub seed: u128, // the seed used to generate more than one bet PDA for the same player within the same round
    pub slot: u64, // the slot when the bet was placed
    pub bump : u8 // the bump used to generate the bet PDA
}

impl Bet {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 16 + 8 + 1;

    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.push(self.bet as u8);
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());
        s        
    }
}

#[account]
pub struct Global {
    pub round: u64, // to store the global round
    pub number: u8, // to store the random number of the previous round
    pub bump: u8, // the bump used to generate the global PDA
}

impl Global {
    pub const LEN: usize = 8 + 8 + 1 + 1;
}