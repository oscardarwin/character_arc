use layout::core::utils::save_to_file;
use layout::{backends::svg::SVGWriter, gv};
use model::ChangeKey;
use petgraph::dot::Dot;
use std::error::Error;

// mod example_game;
mod mip_story_builder;
mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let num_scenes = 6;

    let scene_graph =
        mip_story_builder::build(vec![ChangeKey("decision".to_string())], num_scenes)?;

    let dot = Dot::new(&scene_graph);

    let contents = format!("{}", dot);
    let mut parser = gv::DotParser::new(&contents);

    let graph_viz = parser.process()?;

    let mut gb = gv::GraphBuilder::new();
    gb.visit_graph(&graph_viz);
    let mut vg = gb.get();

    let mut svg = SVGWriter::new();

    vg.do_it(false, true, false, &mut svg);
    let content = svg.finalize();

    let res = save_to_file("output.svg", &content);

    Ok(())
}
