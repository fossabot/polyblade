use std::collections::{HashMap, HashSet};

use super::{Cycles, Shape};
use crate::polyhedron::{shape::Cycle, VertexId};

impl Shape {
    pub fn split_vertex(&mut self, v: VertexId) -> Vec<[usize; 2]> {
        let sc = self.cycles.sorted_connections(v);
        let edges = self.distance.split_vertex(v, sc);
        self.cycles = Cycles::from(&self.distance);
        edges
    }

    pub fn contract_edges(&mut self, edges: Vec<[VertexId; 2]>) {
        self.distance.contract_edges(edges);
        self.recompute();
    }

    pub fn kis(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        let edges = self.distance.edges().collect();
        // let mut cycles = self.cycles.clone();
        if let Some(degree) = degree {
            self.cycles
                .iter()
                .collect::<Vec<_>>()
                .retain(|c| c.len() == degree);
        }
        for cycle in self.cycles.iter() {
            let v = self.distance.insert();
            // let mut vpos = Vec3::zero();

            for &u in cycle.iter() {
                self.distance.connect([v, u]);
                //vpos += self.positions[&u];
            }

            //self.positions.insert(v, vpos / cycle.len() as f32);
        }

        self.recompute();
        edges
    }

    pub fn chamfer(&mut self) {
        let originals = self.edges().collect::<Vec<_>>();
        for cycle in self.cycles.iter() {
            let mut new_face = vec![];
            for &v in cycle.iter() {
                let u = self.distance.insert();
                new_face.push(u);
                self.distance.connect([v, u]);
            }
            for i in 0..new_face.len() {
                self.distance
                    .connect([new_face[i], new_face[(i + 1) % new_face.len()]]);
            }
        }
        for edge in originals {
            self.distance.disconnect(edge);
        }
        self.recompute();
    }
}
