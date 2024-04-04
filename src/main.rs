use layout::core::utils::save_to_file;
use layout::{backends::svg::SVGWriter, gv};
use model::{ChangeKey, SceneGraph};
use petgraph::dot::Dot;
use std::error::Error;

// mod example_game;
mod mip_story_builder;
mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let num_scenes = 6;
    let num_outcomes = 3;

    let scene_graph = mip_story_builder::build(
        vec![ChangeKey("d1".to_string()), ChangeKey("d2".to_string())],
        num_scenes,
        num_outcomes,
    )?;

    visualise_scene_graph(&scene_graph)?;

    Ok(())
}

fn visualise_scene_graph(scene_graph: &SceneGraph) -> Result<(), Box<dyn Error>> {
    let dot = Dot::new(&scene_graph);

    let contents = format!("{}", dot);

    //println!("{:?}", &dot);

    let mut parser = gv::DotParser::new(&contents);

    let graph_viz = parser.process()?;

    //println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));

    //let res = save_to_file("output.viz", gv::dump_ast(&graph_viz));
    // gv::dump_ast(&graph_viz);

    let mut gb = gv::GraphBuilder::new();
    gb.visit_graph(&graph_viz);
    let mut vg = gb.get();

    let mut svg = SVGWriter::new();

    vg.do_it(false, true, false, &mut svg);
    let content = svg.finalize();

    let res = save_to_file("output.svg", &content);

    Ok(())
}
