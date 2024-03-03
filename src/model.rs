use petgraph::{graph::Graph, Directed};
use std::fmt;

#[derive(Clone, Debug)]
pub struct State(pub String);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ChangeKey(pub String);

impl fmt::Display for ChangeKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    from: State,
    to: State,
    change: ChangeKey,
}

#[derive(Clone, Debug)]
pub struct CharacterArc {
    initial_state: State,
    transitions: Vec<Transition>,
}

#[derive(Clone, Debug)]
pub struct StoryEntity {
    name: String,
    arcs: Vec<CharacterArc>,
}

#[derive(Clone, Debug)]
pub struct StoryEntityState(StoryEntity, State);

#[derive(Clone, Debug)]
pub struct Scene {
    pub story_entities: Vec<StoryEntityState>,
}

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scene_info")
    }
}

pub type SceneGraph = Graph<Scene, ChangeKey, Directed, usize>;
