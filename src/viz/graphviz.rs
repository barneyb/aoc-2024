use crate::timing::Timing;
use crate::viz::viz_file_stem;
use petgraph::dot::Dot;
use petgraph::visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::process::Command;

/// I render the passed [Dot] to a GraphViz file, named for the current
/// executable, and uses `dot` to filter it into a PDF version of the same.
pub fn render_dot<G>(dot: &Dot<G>)
where
    G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: Debug,
    G::NodeWeight: Debug,
{
    write_and_render(|f| write!(f, "{:?}", dot))
}

/// I render the passed unweighted adjacency-list graph to a GraphViz file,
/// named for the current executable, and uses `dot` to filter it into a PDF
/// version of the same.
pub fn render_unweighted<N>(graph: &HashMap<N, Vec<N>>)
where
    N: Debug + Eq + Hash,
{
    write_and_render(|f| {
        let mut index = HashMap::new();
        writeln!(f, "digraph {{")?;
        for (n, _) in graph {
            let i = index.len();
            index.insert(n, i);
            writeln!(f, "{i} [label=\"{n:?}\"]")?;
        }
        for (n, es) in graph {
            let i = index.get(n).unwrap();
            for e in es {
                writeln!(f, "{i} -> {}", index.get(e).unwrap())?;
            }
        }
        writeln!(f, "}}")
    })
}

/// I render the passed weighted adjacency-list graph to a GraphViz file, named
/// for the current executable, and uses `dot` to filter it into a PDF version
/// of the same.
pub fn render_weighted<N, W>(graph: &HashMap<N, Vec<(N, W)>>)
where
    N: Debug,
    W: Debug,
{
    write_and_render(|f| {
        writeln!(f, "digraph {{")?;
        for (n, es) in graph {
            for (e, w) in es {
                writeln!(f, "\"{n:?}\" -> \"{e:?}\" [label=\"{w:?}\"]")?;
            }
        }
        writeln!(f, "}}")
    })
}

/// I am the most low-level function, providing direct access to emit whatever
/// graphviz content you wish, directly to the output file, named for the
/// current executable. Afterward, `dot` is used to filter it into a PDF version
/// of the same.
pub fn write_and_render<F>(emit_content: F)
where
    F: Fn(&mut BufWriter<File>) -> std::io::Result<()>,
{
    const PLUGIN: &str = "dot";
    const FORMAT: &str = "pdf";
    let (filename_gv, filename_render) = make_filenames(FORMAT);
    Timing::ad_hoc(&format!("Wrote graph to '{filename_gv}'"), || {
        let f = File::create(&filename_gv).expect(&format!("Unable to create '{filename_gv}'"));
        let mut f = BufWriter::new(f);
        emit_content(&mut f).expect("Unable to write data");
    });
    let exit_code = Timing::ad_hoc(&format!("Rendered '{filename_render}'"), || {
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
