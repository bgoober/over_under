use anchor_lang::prelude::*;

use crate::state::Bet;

/// DOCS:
/// In the interest of automation, because there are 10 players maximum in each round, and we store each Bet pda and player pubkey;
/// we will have 10 different CloseBetC contexts, so an automated process can decide exactly which context to use and how many accounts
/// to close for any given round.

#[derive(Accounts)]
pub struct Close1BetC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet.player)]
    pub player: AccountInfo<'info>,

    #[account(mut, has_one = player, close = player)]
    pub bet: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close2BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close3BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close4BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close5BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close6BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,

    #[account(mut, address = bet6.player)]
    pub player6: AccountInfo<'info>,

    #[account(mut, close = player6)]
    pub bet6: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close7BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,

    #[account(mut, address = bet6.player)]
    pub player6: AccountInfo<'info>,

    #[account(mut, close = player6)]
    pub bet6: Account<'info, Bet>,

    #[account(mut, address = bet7.player)]
    pub player7: AccountInfo<'info>,

    #[account(mut, close = player7)]
    pub bet7: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close8BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,

    #[account(mut, address = bet6.player)]
    pub player6: AccountInfo<'info>,

    #[account(mut, close = player6)]
    pub bet6: Account<'info, Bet>,

    #[account(mut, address = bet7.player)]
    pub player7: AccountInfo<'info>,

    #[account(mut, close = player7)]
    pub bet7: Account<'info, Bet>,

    #[account(mut, address = bet8.player)]
    pub player8: AccountInfo<'info>,

    #[account(mut, close = player8)]
    pub bet8: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close9BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,

    #[account(mut, address = bet6.player)]
    pub player6: AccountInfo<'info>,

    #[account(mut, close = player6)]
    pub bet6: Account<'info, Bet>,

    #[account(mut, address = bet7.player)]
    pub player7: AccountInfo<'info>,

    #[account(mut, close = player7)]
    pub bet7: Account<'info, Bet>,

    #[account(mut, address = bet8.player)]
    pub player8: AccountInfo<'info>,

    #[account(mut, close = player8)]
    pub bet8: Account<'info, Bet>,

    #[account(mut, address = bet9.player)]
    pub player9: AccountInfo<'info>,

    #[account(mut, close = player9)]
    pub bet9: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct Close10BetsC<'info> {
    #[account(mut)]
    pub thread: Signer<'info>,

    #[account(mut, address = bet1.player)]
    pub player1: AccountInfo<'info>,

    #[account(mut, close = player1)]
    pub bet1: Account<'info, Bet>,

    #[account(mut, address = bet2.player)]
    pub player2: AccountInfo<'info>,

    #[account(mut, close = player2)]
    pub bet2: Account<'info, Bet>,

    #[account(mut, address = bet3.player)]
    pub player3: AccountInfo<'info>,

    #[account(mut, close = player3)]
    pub bet3: Account<'info, Bet>,

    #[account(mut, address = bet4.player)]
    pub player4: AccountInfo<'info>,

    #[account(mut,  close = player4)]
    pub bet4: Account<'info, Bet>,

    #[account(mut, address = bet5.player)]
    pub player5: AccountInfo<'info>,

    #[account(mut, close = player5)]
    pub bet5: Account<'info, Bet>,

    #[account(mut, address = bet6.player)]
    pub player6: AccountInfo<'info>,

    #[account(mut, close = player6)]
    pub bet6: Account<'info, Bet>,

    #[account(mut, address = bet7.player)]
    pub player7: AccountInfo<'info>,

    #[account(mut, close = player7)]
    pub bet7: Account<'info, Bet>,

    #[account(mut, address = bet8.player)]
    pub player8: AccountInfo<'info>,

    #[account(mut, close = player8)]
    pub bet8: Account<'info, Bet>,

    #[account(mut, address = bet9.player)]
    pub player9: AccountInfo<'info>,

    #[account(mut, close = player9)]
    pub bet9: Account<'info, Bet>,

    #[account(mut, address = bet10.player)]
    pub player10: AccountInfo<'info>,

    #[account(mut, close = player10)]
    pub bet10: Account<'info, Bet>,
}
