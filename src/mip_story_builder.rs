use crate::model::{ChangeKey, Scene, SceneGraph};
use core::ops::Add;
use good_lp::{
    default_solver, Constraint, Expression, ProblemVariables, ResolutionError, Solution,
    SolverModel, Variable, VariableDefinition,
};
use itertools::iproduct;
use ndarray::{s, Array, Array2, Array3, ArrayView, ArrayView1, Axis, Dim, Dimension, Zip};
use petgraph::data::{Element, FromElements};
use std::error::Error;

mod constraints;
mod mip_model;
mod mip_utils;

use constraints::branching::BranchingConstraintGenerator;
use mip_model::{ConstraintGenerator, StoryExpressions, StoryParameters, StoryVariables};

use self::constraints::{
    reachability::ReachabilityConstraintGenerator, transitions::TransitionConstraintGenerator,
};

struct StoryGenerationResult {
    solution: Box<dyn Solution>,
    story_variables: StoryVariables,
}

fn generate_from_mip(
    num_scenes: usize,
    num_decisions: usize,
    num_final_scenes: usize,
) -> Result<StoryGenerationResult, ResolutionError> {
    let mut variables = ProblemVariables::new();

    let parameters = StoryParameters {
        num_scenes,
        num_final_scenes,
        num_decisions,
        min_branching: 2,
        max_branching: 6,
    };

    let mut mip_variables = ProblemVariables::new();

    let story_variables = StoryVariables::new(&parameters, &mut mip_variables);

    let lin_sum: Expression = story_variables
        .scene_edges
        .iter()
        .filter_map(|e: &Option<Variable>| *e)
        .map(|variable| Expression::from(variable))
        .sum();

    let mut model = mip_variables.minimise(lin_sum).using(default_solver);

    let story_expressions = StoryExpressions::from(&story_variables);

    // minimum branching
    let constraint_generators: Vec<Box<dyn ConstraintGenerator>> = vec![
        //Box::new(BranchingConstraintGenerator {}),
        //Box::new(TransitionConstraintGenerator {}),
        Box::new(ReachabilityConstraintGenerator {}),
    ];

    for constraint_generator in constraint_generators.into_iter() {
        let constraints = constraint_generator.generate(&story_expressions, &parameters);

        for constraint in constraints {
            model.add_constraint(constraint);
        }
    }

    // reachability
    model.solve().map(|solution| StoryGenerationResult {
        solution: Box::new(solution),
        story_variables,
    })
}

fn optional_to_f64(optional: &Option<f64>) -> f64 {
    match optional {
        Some(x) => f64::clone(x),
        None => -1.0,
    }
}

fn print_solution(story_variables: &StoryVariables, solution: &Box<dyn Solution>) {
    let fixed_variables = story_variables.get_solution_values(solution);
    let (n, m, o) = fixed_variables.scene_edges.dim();
    let (_, _, num_outcomes) = fixed_variables.outcome_transition_reachability.dim();

    println!("reachability:");
    println!(
        "{:#?}",
        Array3::from_shape_fn((num_outcomes, n, m), |(oi, ni, mi)| {
            fixed_variables.outcome_transition_reachability[[ni, mi, oi]]
        })
    );

    println!("transitions:");
    println!(
        "{:#?}",
        fixed_variables.scene_transitions.map(optional_to_f64)
    );
    println!("edges:");

    let edges = Array3::from_shape_fn((o, n, m), |(oi, ni, mi)| {
        fixed_variables.scene_edges[[ni, mi, oi]]
    });

    println!("{:#?}", edges.map(optional_to_f64));
}

pub fn build(
    change_keys: Vec<ChangeKey>,
    num_scenes: usize,
    num_outcomes: usize,
) -> Result<SceneGraph, Box<dyn Error>> {
    let StoryGenerationResult {
        solution,
        story_variables,
    } = generate_from_mip(
        num_scenes,
        change_keys.len().try_into().unwrap(),
        num_outcomes,
    )?;

    print_solution(&story_variables, &solution);

    //println!("{:#?}", story_variables);

    // create a list of scenes
    let scenes = (0..num_scenes + num_outcomes)
        .into_iter()
        .map(|i| Element::Node {
            weight: Scene {
                name: format!("scene-{}", i),
                story_entities: vec![],
            },
        });

    let scene_transitions = story_variables
        .scene_edges
        .indexed_iter()
        .to_owned()
        .filter_map(|item| match item {
            (id, Some(variable)) => Some((id, solution.value(*variable))),
            (_, None) => None,
        })
        .filter(|(_, value)| value.ge(&0.9))
        .map(|((from_id, to_id, decision_id), _)| Element::Edge {
            source: from_id,
            target: to_id,
            weight: change_keys.get(decision_id).unwrap().clone(),
        });

    let scene_graph = SceneGraph::from_elements(scenes.chain(scene_transitions));
    Ok(scene_graph)
}
