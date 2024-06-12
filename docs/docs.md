## Accounts

1. Global - Stores the Global state of the game; the current Round number, and the random number from the previous Round.
    - round: u64
    - number: u64
    - bump: u8
2. Round - Stores the state of each Round of gameplay; compares itself to, then updates Global at the end of each Round.
    - round: u64
    - number: u64
    - outcome: u8 | 0 == false (lower/under), 1 == true (over/higher), 2 == same, 3 == round is being played/not-yet-concluded
    - bump: u8
3. Bet - Stores the bets made by each player; 1 per player per Round.
    - bet: u8 | 0 or 1
    - amount: u64 | made in SOL
    - round: u64
    - bump: u8

## Contexts

1. InitGlobalC - Initializes the Global Account. Signed by the House pubkey.
    - Accounts:
        - House: Signer
        - Global (init)
            - PDA of Program + House pubkey
        - SystemProgram
        
2. InitRoundC - Initializes a new Round Account. Signed by the House or a Thread.
    - Accounts:
        - House or thread: Signer
        - Global PDA
        - Round (init)
            - PDA of Global + _round u64
        - vault (init)
            - PDA of Round
        - SystemProgram

3. InitBetC - Initializes a new Bet account for the user; 1 per player. Signed by the player.
    - Accounts:
        - player: Signer
        - Global PDA
        - Round PDA
        - vault PDA 
        - Bet Account
            - PDA of Round + player's pubkey
        - SystemProgram

4. PlayRoundC - Takes an existing round, and generates a random number for that round, and updates the Global Account. Signed by a thread.
    - Accounts:
        - thread: Signer
        - house 
        - Global (mut)
        - Round (mut)

5. ResolveBetC - Resolves Bet accounts after a Round has been played. Signed by a thread, or the player.
    - Accounts:
        - player/thread: Signer
        - house
        - Global
        - Round 
        - Bet (close = player)




