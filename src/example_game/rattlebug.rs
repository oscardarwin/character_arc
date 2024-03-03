use crate::model::CharacterArc;
use crate::{GameDecision, GameEntity};

enum RattlebugState {
    ScaredAndIdle,
    ActiveGood,
    Evil,
}

impl CharacterArc<RattlebugState, GameDecision> for GameEntity::Rattlebug {
    fn initial_state() -> RattlebugState {
        RattlebugState::ScaredAndIdle
    }

    fn arc_graph() -> Graph<RattlebugState, GameDecision> {
        let mut graph = Graph::<RattlebugState, GameDecision>::new();
        let s1 = graph.add_node(RattlebugState::ScaredAndIdle);
        let s2 = graph.add_node(RattlebugState::ActiveGood);
        let s3 = graph.add_node(RattlebugState::Evil);

        graph.extend_with_edges(&[
            (s1, s2, GameDecision::SeekOutGirigoth),
            (s1, s3, GameDecision::KillSomething),
            (s1, s1, GameDecision::ReadTheBook),
            (s2, s2, GameDecision::ReadTheBook),
            (s2, s3, GameDecision::DestroyTheBook),
            (s1, s3, GameDecision::DestroyTheBook),
        ]);

        graph
    }
}
