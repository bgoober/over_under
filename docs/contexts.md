## Contexts

1. InitGlobalC
    - Initializes the Global Account. Signed by the House pubkey.
    - Accounts:
        - House: Signer
        - Global (init)
            - PDA of Program + House pubkey
        - SystemProgram
        
2. InitRoundC
    - Initializes a new Round Account. Signed by the House or a Thread.
    - Accounts:
        - House or thread: Signer
        - Global PDA
        - Round (init)
            - PDA of Global + _round u64
        - vault (init)
            - PDA of Round
        - SystemProgram

3. InitBetC
    - Initializes a new Bet account for the user; 1 per player. Signed by the player.
    - Accounts:
        - player: Signer
        - Global PDA
        - Round PDA
        - vault PDA 
        - Bet Account
            - PDA of Round + player's pubkey
        - SystemProgram

4. PlayRoundC
    - Takes an existing round, and generates a random number for that round, and updates the Global Account. Signed by a thread.
    - Accounts:
        - thread: Signer
        - house
        - Global
        - Round

5. ResolveBetC


