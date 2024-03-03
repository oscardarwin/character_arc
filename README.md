

# Game Plan


Character
    - Arc[]

ArcState

ArcDecision

Arc
    - name: str
    - (State, Decision) -> State

StoryEntity
    - Arc[]
    - tags: Vec<Str>
    - PlayerRelevance: HashMap<Character, Score>

Scene:
    - Vec<StoreEntity>
    - Vec<Decision> 

SceneGraph:
    - Graph<Scene, Decision>

## What makes a good story?

minimum story length
    - new variable restricted to available scene decisions
    - 

How branching is the story ~ 3 outcomes per scene
Drama factor...
    - 

Set of Observations


## Fixed Variables

e_k entities
d_n decisions
x_j state
N: number of scenes
e_{k, s, r, d}: entity k can take decision d transitioning from s to r.


## Free Variables

E_{m, k, s} = 1 if Entity k is in scene m in state s
G_{m, n, d} = 1 if Scene n follows Scene m with Decision d. 

## Constraints

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



for all scenes m, entities k
sum_s E_{m, k, s} == 1


for all scenes m, entities k, states s:

E_{m, }


- 
- Decisions must be foreshadowed
- Each character must take roughly the same frequency of Decisions
- The story must end in tragedy or victory

