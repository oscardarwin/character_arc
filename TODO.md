# Game Plan

## What makes a good story?

minimum story length
    - new variable restricted to available scene decisions
    - 

a fixed example

## Fixed Variables

e_k entities
d_n decisions
x_j state
N: number of scenes
e_{k, s, r, d}: entity k can take decision d transitioning from s to r.


## Free Variables

E_{m, k, s} = 1 if Entity k is in scene m in state s
G_{m, n, d} = 1 if Scene n follows Scene m with Decision d. 

R_{m, o} = 1 if outcome o is reachable from scene m.

## Constraints

x == 1 and y == 1 => z == 1

z + 1 >= x + y


for all scenes m <= n:
*DAG property of story*
*can be set explicitly by variable construction
sum_d G_{m, n, d} == 0

*At most one next scene per decision*
for all scenes m, decisions d
sum_n G_{m, n, d} <= 1

*Minimum and maximum branching for each scene*
for all scenes m:
2 < sum_{n, d} G_{m, n, d} < 5

*Reachability of end states*
for each outcome o:
R_{0, o} == 1

*Reachability Propogation*

*if R_{n, o} == 1 and sum_d G_{m, n, d} >= 1 then R_{m, o} == 1*
1 + R_{m, o} >= sum_d G_{m, n, d} / D + R_{n, o} 

*a node with no reachable neighbours is unreachable*
AND_n G_{m, n, d} == 1 and 1 - R_{n, o} == 1 then R_{m, o} == 0

R_{m, o} == 0 AND T_{m, n} == 1 => R_{n, o} == 0
R_{m, o} == 1 AND T_{m, n} == 1 => R_{n, 0} == 1


(2 - R_{n, o}) >= 1 - R_{m, o} + T_{m, n}
1 + R_{m, o} >= R_{n, o} + T_{m, n}

T_{m,n} == 1 R_{m,o} == 0 => R_{n,o} == 0

P_{m, n, o} + 1 >= T_{m, n} + R_{n, o}




* Propogations *
When outcome o propogates between i and j.





2 - R_{m, o} >= 1 / N sum_n (T_{m, n} + (1 - R_{n, o}))

*outcome scenes have the right outcomes*

*initial scene can reach all outcomes*




for all scenes m, entities k
sum_s E_{m, k, s} == 1


for all scenes m, entities k, states s:

E_{m, }


- 
- Decisions must be foreshadowed
- Each character must take roughly the same frequency of Decisions
- The story must end in tragedy or victory

