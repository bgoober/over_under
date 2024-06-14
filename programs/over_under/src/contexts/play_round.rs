use std::collections::BTreeMap;

use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{ prelude::*, system_program::{transfer, Transfer}};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::{
    __private::__idl::__cpi_client_accounts_idl_close_account::IdlCloseAccount, errors::Error, state::{Bet, Global, Round}
};

#[derive(Accounts)]
pub struct PlayRoundC<'info> {
    #[account(mut)]
    thread: Signer<'info>,

    // the pubkey of the signer of init global
    pub house: SystemAccount<'info>,

    // global account which is a pda of the program ID and the house pubkey
    #[account(
        seeds = [b"global", house.key().as_ref()],
        bump
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub bet: Account<'info, Bet>,

    // round the player is placing a bet in,
    #[account(seeds = [b"round", global.key().as_ref(), global.round.to_le_bytes().as_ref()], bump)]
    pub round: Account<'info, Round>,

    // vault pda of the round account
    #[account(mut, seeds = [b"vault", round.key().as_ref()], bump, close = house)]
    pub vault: SystemAccount<'info>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: This is safe
    pub instruction_sysvar: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlayRoundC<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, Error::Ed25519Program);
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, Error::Ed25519Accounts);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, Error::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, Error::Ed25519Header);

        // Ensure public keys match
        require_keys_eq!(
            signature.public_key.ok_or(Error::Ed25519Pubkey)?,
            self.house.key(),
            Error::Ed25519Pubkey
        );

        // Ensure signatures match
        require!(
            &signature.signature.ok_or(Error::Ed25519Signature)?.eq(sig),
            Error::Ed25519Signature
        );

        // Ensure messages match
        require!(
            &signature
                .message
                .as_ref()
                .ok_or(Error::Ed25519Signature)?
                .eq(&self.round.to_slice()),
            Error::Ed25519Signature
        );

        Ok(())
    }

    pub fn play_round(&mut self, _bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        let roll = self.calculate_roll(sig);
        self.round.number = roll;
        self.update_round_outcome();
        self.calculate_winners(); // updates the round.winners vector
        self.payout_winners(); // pays out the winners
        self.update_global_state();


        Ok(())
    }

    pub fn calculate_roll(&self, sig: &[u8]) -> u8 {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // produce a number 0-100
        lower.wrapping_add(upper).wrapping_rem(101) as u8
    }

    pub fn update_round_outcome(&mut self) {
        if self.round.number > self.global.number {
            self.round.outcome = 1;
        } else if self.round.number < self.global.number {
            self.round.outcome = 0;
        } else {
            self.round.outcome = 2;
        }
    }

    pub fn calculate_winners(&mut self) -> Result<()> {

        let pot = self.vault.lamports();

        // a winners pot must be created that is equal to the total sum of each player's bet.amount who won the game
        let mut winners_pot: u64 = 0;

        // iterate through each account in the remaining account
        // compare the players bet to the outcome of the round, and if they won, add their bet.amount to the winners_pot
        for account in ctx.remaining_accounts.iter() {
            let _account_key = account.key();
            let data = account.try_borrow_mut_data();

            //Deserialize the data from the account and save it in an Account variable
            let account_to_write = Bet::try_deserialize(&mut data.unwrap().as_ref())
                .expect("Error Deserializing Bet Account Data");

            if account_to_write.bet == self.round.outcome {
                winners_pot += account_to_write.amount;
                let payout = (account_to_write.amount / winners_pot) * pot;
                self.round.winners.push((_account_key, payout));
            }

            // close the Bet account
            account_to_write.close();


            let account = AccountsClose {
                account: account_to_write.to_account_info(),
                destination: self.bet.player.key(),
            };

        }

        Ok(())
    }

    pub fn payout_winners(&mut self) -> Result<()> {
        // iterate through each winner in the round.winners array and transfer the payout to the winner's pubkey
        for &(ref winner_pubkey, amount) in self.round.winners.iter() {
            let cpi_program = self.system_program.to_account_info();
    
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: winner_pubkey.to_account_info(), // Use the winner's pubkey
            };
    
            let seeds = &[
                b"vault",
                self.round.key().as_ref(),
                &[self.round.bump],
            ];
    
            let signer_seeds = &[&seeds[..]];
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    
            // Transfer the amount won by the winner
            transfer(cpi_ctx, amount)?;
        }

        // close the vault account
        self.vault.close();
    
        Ok(())
    }


    pub fn update_global_state(&mut self) {
        self.global.number = self.round.number;
        self.global.round += 1;
    }
}
// TODO: put the resolve_bet function on an internal timer using the slot number to call the function, then init a new round.
