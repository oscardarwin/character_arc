use crate::model::CharacterArc;
use crate::{GameDecision, GameEntity};
use petgraph::graph::Graph;

enum StrangeBookState {
    Idle,
    GirigothSpy,
}

impl CharacterArc<StrangeBookState, GameDecision> for GameEntity::Rattlebug {
    fn initial_state() -> StrangeBookState {
        StrangeBookState::Idle
    }

    fn arc_graph() -> Graph<StrangeBookState, GameDecision> {
        let mut graph = Graph::<StrangeBookState, GameDecision>::new();
        let s1 = graph.add_node(StrangeBookState::Idle);
        let s2 = graph.add_node(StrangeBookState::GirigothSpy);

        graph.extend_with_edges(&[(s1, s2, GameDecision::ReadTheBook)]);

        graph
    }
}
