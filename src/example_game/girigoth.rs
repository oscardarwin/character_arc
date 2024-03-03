use crate::model::CharacterArc;
use crate::{GameDecision, GameEntity};
use petgraph::graph::Graph;

enum GirigothState {
    GeneralHunting,
    HuntingRattlebug,
    Dead,
}

impl CharacterArc<GirigothState, GameDecision> for GameEntity::Girigoth {
    fn initial_state() -> GirigothState {
        GirigothState::GeneralHunting
    }

    fn arc_graph() -> Graph<GirigothState, GameDecision> {
        let mut graph = Graph::<GirigothState, GameDecision>::new();
        let s1 = graph.add_node(GirigothState::GeneralHunting);
        let s2 = graph.add_node(GirigothState::HuntingRattlebug);
        let s3 = graph.add_node(GirigothState::Dead);

        graph.extend_with_edges(&[
            (s1, s2, GameDecision::ReadTheBook),
            (s2, s3, GameDecision::DestroyTheBook),
        ]);

        graph
    }
}
