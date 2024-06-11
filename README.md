# Character Arc

Use very short future Monte carlo to analyse future moves.





# A Grammar System for teaching Boss Mechanics

Prepare some abilities and then the computer finds teamwork based puzzles that need to be completed to succeed.

Basically about identifying interactions. I.e. specific combinations of moves that serve as the "key" to the puzzle.


Define a move space for the game
Define a resources names
Define trigger names

Define a probability distribution on the move space.
Randomly select a set of moves from the move space

compute an optimal policy with no restrictions on moves
compute a set of optimal policies with:
    - only partial synergy
    - non-even number of contributions

game is good if:
    - moves are distinct
    - free policy wins
    - free policy has high synergy (preference for reaction based moves)
    - free policy has even number of player contributions
    - free policy has some variance in possible moves
    - all restricted policies lose


Specify a fixed number of moves linked to resources, 

Choose a spread out set of starting moves

Run a genetic algorithm to mutate the starting moves and evalute based on the criteria above.



## How well will this work?

-- metrics to gain some idea of how much of the search space has been covered.





Explore the move space




# Approximate the meta in Card Games

specify a significant percentage e.g. 0.7. We say W_{i,j} = 0.4 min(0, ((0.7 - 0.5) * 2)) if D_i beats D_j 70% of the time.

Can we create a graph that loosely approximates the larger card graph? 

I.e. partition the decks into P_1, ..., P_N and produce a matrix V_{i,j} such that 

V_{i,j} = P_{r \in P_i, s \in P_j}( D_r beats D_s )


Let V_{i,j} be a random variable chosen by uniformly randomly choosing a decks in s \in P_i and r \in P_j respectively and emitting W_{i,j}.

Then we want Var(V_{i,j}) to be as small as possible and E[V_{i,i}] to be close to 0.
