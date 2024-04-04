use good_lp::Constraint;
use ndarray::Axis;

use crate::mip_story_builder::{
    mip_model::{ConstraintGenerator, StoryExpressions, StoryParameters},
    mip_utils::sum_optional_expressions,
};

pub struct BranchingConstraintGenerator {}

impl ConstraintGenerator for BranchingConstraintGenerator {
    fn generate(
        &self,
        story_expressions: &StoryExpressions,
        story_parameters: &StoryParameters,
    ) -> Vec<Constraint> {
        let mut greater_than_constraint =
            atleast_n_branches_per_scene(story_expressions, story_parameters);
        let mut less_than_constraint =
            atleast_n_branches_per_scene(story_expressions, story_parameters);

        greater_than_constraint.append(&mut less_than_constraint);

        greater_than_constraint
    }
}

fn atleast_n_branches_per_scene(
    story_expressions: &StoryExpressions,
    story_parameters: &StoryParameters,
) -> Vec<Constraint> {
    let (num_scenes, scenes_and_outcomes, num_observations) = story_expressions.scene_edges.dim();
    let min_branching = story_parameters.min_branching as f64;

    story_expressions
        .scene_transitions
        .map_axis(Axis(1), sum_optional_expressions)
        .into_iter()
        .filter_map(|transition| Some(transition?.geq(min_branching)))
        .collect()
}

fn fewer_than_n_branches_per_scene(
    story_expressions: &StoryExpressions,
    story_parameters: &StoryParameters,
) -> Vec<Constraint> {
    let (num_scenes, scenes_and_outcomes, num_observations) = story_expressions.scene_edges.dim();

    let max_branching_float = story_parameters.max_branching as f64;

    story_expressions
        .scene_edges
        .map_axis(Axis(0), sum_optional_expressions)
        .into_iter()
        .filter_map(|neighbours| Some(neighbours?.leq(max_branching_float.clone())))
        .collect()
}
