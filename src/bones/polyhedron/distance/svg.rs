use crate::bones::Distance;
use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec, parse,
    printer::PrinterContext,
};

impl Distance {
    pub fn graphviz(&self) -> String {
        let mut dot = format!("graph G{{\nlayout=neato\n");
        let colors = vec!["red", "green", "blue"];
        for v in self.vertices() {
            dot.push_str(&format!(
                "\tV{v} [color=\"{}\"];\n",
                colors[self.connections(v).len() % colors.len()]
            ));
        }

        for [v, u] in self.edges() {
            dot.push_str(&format!("\tV{v} -- V{u};\n"));
        }
        dot.push_str("}");
        dot
    }

    pub(in crate::bones::polyhedron) fn svg(&self) -> Option<Vec<u8>> {
        let Ok(graph) = parse(&self.graphviz()) else {
            log::warn!("failed to parse Graphviz");
            return None;
        };
        exec(
            graph,
            &mut PrinterContext::default(),
            vec![
                Format::Svg.into(),
                //CommandArg::Output(format!("{}{}", prefix, filename)),
            ],
        )
        .ok()
    }
}
