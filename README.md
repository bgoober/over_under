# Over / Under

A round-based betting game. Players bet on the outcome of a random number; whether it will be Over or Under (higher or lower), than the previous round's random number. Loser pay winners. If the new random number is the same as the previous round's random number then the House wins.

TODO: 

1. Refactor the contexts to:
    - InitGlobal
        - init_game
            - permanent, never closes
    - InitRound
        - init_round
            - ephemeral, closes after each round, a new round is init'd by a Clockwork thread after each round is closed.
    - BetContext
        - place_bet
            - ephemeral, closes when the 

2. Refactor the accounts/state to:
    - Global