use good_lp::Constraint;
use ndarray::{Axis, Zip};

use crate::mip_story_builder::{
    mip_model::{ConstraintGenerator, StoryExpressions, StoryParameters},
    mip_utils::sum_optional_expressions,
};

pub struct TransitionConstraintGenerator {}

impl ConstraintGenerator for TransitionConstraintGenerator {
    fn generate(
        &self,
        story_expressions: &StoryExpressions,
        story_parameters: &StoryParameters,
    ) -> Vec<Constraint> {
        let mut edge_implies_transition = edge_implies_transition(story_expressions);
        let mut transition_implies_edge = transition_implies_edge(story_expressions);

        edge_implies_transition.append(&mut transition_implies_edge);

        edge_implies_transition
    }
}

fn edge_implies_transition(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    let (_, _, total_observations) = story_expressions.scene_edges.dim();

    let sum_of_edge_count = story_expressions
        .scene_edges
        .map_axis(Axis(2), sum_optional_expressions);

    Zip::from(&sum_of_edge_count)
        .and(&story_expressions.scene_transitions)
        .map_collect(|optional_edge_count, optional_transition| {
            match (optional_edge_count, optional_transition) {
                (Some(edge_count), Some(transition)) => Some(
                    transition
                        .clone()
                        .geq(edge_count.clone() / (total_observations as f64)),
                ),
                _ => None,
            }
        })
        .into_iter()
        .filter_map(|x| x)
        .collect()
}

fn transition_implies_edge(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    let (_, _, total_observations) = story_expressions.scene_edges.dim();

    let sum_of_edge_count = story_expressions
        .scene_edges
        .map_axis(Axis(2), sum_optional_expressions);

    Zip::from(&sum_of_edge_count)
        .and(&story_expressions.scene_transitions)
        .map_collect(|optional_edge_count, optional_transition| {
            match (optional_edge_count, optional_transition) {
                (Some(edge_count), Some(transition)) => {
                    Some(transition.clone().leq(edge_count.clone()))
                }
                _ => None,
            }
        })
        .into_iter()
        .filter_map(|x| x)
        .collect()
}
