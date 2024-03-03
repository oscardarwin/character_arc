use crate::model::{ChangeKey, Scene, SceneGraph};
use core::ops::Add;
use good_lp::{
    default_solver, Expression, ProblemVariables, ResolutionError, Solution, SolverModel, Variable,
    VariableDefinition,
};
use ndarray::{s, Array3, Axis};
use petgraph::{
    data::{Element, FromElements},
    graph::Node,
};
use std::error::Error;

struct StoryGenerationResult {
    solution: Box<dyn Solution>,
    scene_edges: Array3<Variable>,
}

fn generate_from_mip(
    num_scenes: usize,
    num_decisions: usize,
) -> Result<StoryGenerationResult, ResolutionError> {
    let mut variables = ProblemVariables::new();

    let scene_edges: Array3<Variable> = Array3::from_shape_fn(
        (num_scenes, num_scenes, num_decisions),
        |(from_scene_index, to_scene_index, change_key_index)| {
            variables.add(VariableDefinition::new().binary().name(format!(
                "scene-{from_scene_index}-to-scene-{to_scene_index}-with-change-{change_key_index}"
            )))
        },
    );

    let lin_sum: Expression = scene_edges
        .map(|variable| Expression::from(*variable))
        .into_iter()
        .sum();

    let mut model = variables.minimise(lin_sum).using(default_solver);

    for i in 0..num_scenes {
        let edges_for_scene = scene_edges.slice(s![i, .., ..]);

        let atleast_one_edge_per_scene = edges_for_scene
            .mapv(Expression::from)
            .into_iter()
            .reduce(|e_1, e_2| e_1.add(e_2))
            .unwrap()
            .geq(1);

        model.add_constraint(atleast_one_edge_per_scene);
    }

    model.solve().map(|solution| StoryGenerationResult {
        solution: Box::new(solution),
        scene_edges,
    })
}

pub fn build(change_keys: Vec<ChangeKey>, num_scenes: usize) -> Result<SceneGraph, Box<dyn Error>> {
    let StoryGenerationResult {
        solution,
        scene_edges,
    } = generate_from_mip(num_scenes, change_keys.len())?;

    // create a list of scenes
    let scenes = (0..num_scenes).into_iter().map(|_| Element::Node {
        weight: Scene {
            story_entities: vec![],
        },
    });

    let scene_transitions = scene_edges
        .indexed_iter()
        .to_owned()
        .map(|(id, variable)| (id, solution.value(*variable)))
        .filter(|(_, value)| value.ge(&0.9))
        .map(|((from_id, to_id, decision_id), _)| Element::Edge {
            source: from_id,
            target: to_id,
            weight: change_keys.get(decision_id).unwrap().clone(),
        });

    let scene_graph = SceneGraph::from_elements(scenes.chain(scene_transitions));
    Ok(scene_graph)
}
