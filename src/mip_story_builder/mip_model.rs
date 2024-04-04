use good_lp::{Constraint, Expression, ProblemVariables, Solution, Variable, VariableDefinition};
use ndarray::{Array2, Array3};

pub struct StoryParameters {
    pub num_scenes: usize,
    pub num_final_scenes: usize,
    pub num_decisions: usize,

    pub min_branching: usize,
    pub max_branching: usize,
}

#[derive(Debug)]
pub struct StoryComputation<T> {
    pub scene_edges: Array3<Option<T>>,
    pub scene_transitions: Array2<Option<T>>,
    pub reachability: Array3<Option<T>>,
}

pub type StoryVariables = StoryComputation<Variable>;

impl StoryVariables {
    pub fn new(parameters: &StoryParameters, mip_variables: &mut ProblemVariables) -> Self {
        let num_scenes = parameters.num_scenes;
        let num_final_scenes = parameters.num_final_scenes;
        let num_decisions = parameters.num_decisions;

        let scene_edges = Array3::from_shape_fn(
            (num_scenes, num_scenes + num_final_scenes, num_decisions),
            |(from_scene, to_scene, _)| {
                (to_scene > from_scene)
                    .then_some(mip_variables.add(VariableDefinition::new().binary()))
            },
        );

        let scene_transitions = Array2::from_shape_fn(
            (num_scenes, num_scenes + num_final_scenes),
            |(from_scene, to_scene)| {
                (to_scene > from_scene)
                    .then_some(mip_variables.add(VariableDefinition::new().binary()))
            },
        );

        let reachability: Array3<Variable> = Array3::from_shape_fn(
            (num_scenes, num_scenes + num_final_scenes, num_final_scenes),
            |(from, to, _)| {
                (to_scene > from_scene)
                    .then_some(mip_variables.add(VariableDefinition::new().binary()))
            },
        );

        StoryVariables {
            scene_edges,
            reachability,
            scene_transitions,
        }
    }

    pub fn get_solution_values(&self, solution: &Box<dyn Solution>) -> StoryComputation<f64> {
        StoryComputation::<f64> {
            scene_edges: self
                .scene_edges
                .map(|ovar| ovar.map(|var| f64::from(solution.value(var.clone())))),
            scene_transitions: self
                .scene_transitions
                .map(|ovar| ovar.map(|var| f64::from(solution.value(var.clone())))),
            reachability: self
                .reachability
                .map(|var| f64::from(solution.value(var.clone()))),
        }
    }
}

pub type StoryExpressions = StoryComputation<Expression>;

fn optional_var_ref_to_expr(variable: &Option<Variable>) -> Option<Expression> {
    variable.map(|var| Expression::from(var.clone()))
}

impl From<&StoryVariables> for StoryExpressions {
    fn from(story_variables: &StoryVariables) -> Self {
        StoryExpressions {
            scene_edges: story_variables.scene_edges.map(optional_var_ref_to_expr),
            scene_transitions: story_variables
                .scene_transitions
                .map(optional_var_ref_to_expr),
            reachability: story_variables.reachability.map(optional_var_ref_to_expr),
        }
    }
}

pub trait ConstraintGenerator {
    fn generate(
        &self,
        story_expressions: &StoryExpressions,
        story_parameters: &StoryParameters,
    ) -> Vec<Constraint>;
}
