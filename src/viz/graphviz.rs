use crate::timing::Timing;
use crate::viz::viz_file_stem;
use petgraph::dot::Dot;
use petgraph::visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

/// Renders the passed [Dot] reference to a GraphViz file, named for the current
/// executable, and uses `dot` to filter it into a PDF version of the same. If
/// the same executable renders multiple graphs, they'll be named with a `_N`
/// suffix, starting with `_2`.
pub fn render_dot<G>(dot: &Dot<G>)
where
    G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: Debug,
    G::NodeWeight: Debug,
{
    const PLUGIN: &str = "dot";
    const FORMAT: &str = "pdf";
    let (filename_gv, filename_render) = make_filenames(FORMAT);
    Timing::ad_hoc(&format!("Wrote graph to {filename_gv}"), || {
        let diagram = format!("{:?}", dot);
        let f = File::create(&filename_gv).expect(&format!("Unable to create '{filename_gv}'"));
        let mut f = BufWriter::new(f);
        f.write_all(diagram.as_bytes())
            .expect("Unable to write data");
    });
    let exit_code = Timing::ad_hoc(&format!("Rendered '{filename_gv}' to {FORMAT}"), || {
        Command::new(PLUGIN)
            .arg("-T")
            .arg(FORMAT)
            .arg("-o")
            .arg(&filename_render)
            .arg(&filename_gv)
            .spawn()
            .expect(&format!("Unable to launch {PLUGIN}"))
            .wait()
            .expect(&format!("Unable to wait on {PLUGIN}"))
    });
    assert!(exit_code.success(), "{PLUGIN} didn't succeed: {exit_code}");
}

fn make_filenames(format: &str) -> (String, String) {
    let stem = viz_file_stem();
    (stem.clone() + ".gv", stem + "." + format)
}
