use core::ops::Add;
use good_lp::Constraint;
use itertools::iproduct;
use ndarray::{s, Axis};
use std::ops::Div;

use crate::mip_story_builder::{
    mip_model::{ConstraintGenerator, StoryExpressions, StoryParameters},
    mip_utils::{sum_expressions, sum_optional_expressions},
};

pub struct ReachabilityConstraintGenerator {}

impl ConstraintGenerator for ReachabilityConstraintGenerator {
    fn generate(
        &self,
        story_expressions: &StoryExpressions,
        story_parameters: &StoryParameters,
    ) -> Vec<Constraint> {
        let unreachability_propogates = unreachability_propogates(story_expressions);
        let final_scenes_have_fixed_outcomes = final_scenes_have_fixed_outcomes(story_expressions);
        let reachability_implies_transition = reachability_implies_transition(story_expressions);
        let all_scenes_reachable_from_first = all_scenes_reachable_from_first(story_expressions);
        let reachability_propogates = reachability_propogates(story_expressions);

        let constraints = vec![
            //unreachability_propogates,
            reachability_propogates,
            //final_scenes_have_fixed_outcomes,
            reachability_implies_transition,
            all_scenes_reachable_from_first,
        ];

        constraints.into_iter().flatten().collect()
    }
}

fn reachability_propogates(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    let (num_scenes, scenes_plus_outcomes, num_outcomes) = story_expressions.reachability.dim();

    let scene_reaches_outcome = story_expressions
        .reachability
        .map_axis(Axis(1), sum_optional_expressions);

    iproduct!(0..num_scenes, 0..scenes_plus_outcomes, 0..num_outcomes)
        .into_iter()
        .filter_map(|(from, to, final)| {
            let to_reaches_outcome = scene_reaches_outcome[[to, outcome]]
                .unwrap()
                .clone()
                .div((scenes_plus_outcomes + 1) as f64);

            let transition_reaches_outcome =
                story_expressions.reachability[[from, to, outcome]].clone();
            
            match (to_reaches_outcome, transition_reaches_outcome) {
                (Some(trans), Some(to)) => Some(to.leq(trans)),
                _ => None

            }
        })
        .collect()
}

fn unreachability_propogates(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    let (num_scenes, scenes_plus_outcomes, num_outcomes) = story_expressions.reachability.dim();

    let scene_reaches_outcome = story_expressions
        .reachability
        .map_axis(Axis(1), sum_optional_expressions);

    iproduct!(0..num_scenes, 0..scenes_plus_outcomes, 0..num_outcomes)
        .into_iter()
        .filter_map(|(from, to, outcome)| {
            let to_reaches_outcome = scene_reaches_outcome[[to, outcome]].clone();
            let transition_reaches_outcome =
                story_expressions.reachability[[from, to, outcome]].clone();
            match (to_reaches_outcome, transition_reaches_outcome) {
                (Some(to), Some(transition)) => Some(to.geq(transition)),
                _ => None
            } 
        })
        .collect()
}

fn transition_implies_reachability(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    


    Zip::from(story_expressions.reachability)
        .and(story_expressions.scene_transitions)
        .map_collect(
            |reachability, optional_transition| match optional_transition {
                Some(transition) => transition.clone().leq(reachability.clone()),
                _ => None,
            },
        )
        .into_iter()
        .filter_map(|x| x)
        .collect()
}

fn final_scenes_have_fixed_outcomes(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    let (_, _, num_final_scenes) = story_expressions.outcome_transition_reachability.dim();
    let (num_scenes, _) = story_expressions.scene_transitions.dim();

    iproduct!(0..num_scenes, 0..num_final_scenes, 0..num_final_scenes)
        .into_iter()
        .map(|(k, i, j)| {
            let optional_transition =
                story_expressions.scene_transitions[[k, i + num_scenes]].clone();

            let expression =
                story_expressions.outcome_transition_reachability[[k, i + num_scenes, j]].clone();

            match optional_transition {
                Some(transition) if i == j => expression.geq(transition),
                _ => expression.leq(0.1),
            }
        })
        .collect()
}

fn all_scenes_reachable_from_first(story_expressions: &StoryExpressions) -> Vec<Constraint> {
    story_expressions
        .outcome_transition_reachability
        .map_axis(Axis(1), sum_expressions)
        .slice(s![0, ..])
        .into_iter()
        .map(|expression| expression.clone().geq(0.9))
        .collect()
}
