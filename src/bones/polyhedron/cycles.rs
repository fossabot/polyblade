use crate::bones::VertexId;
use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone)]
pub struct Cycle(Vec<VertexId>);

#[derive(Default, Debug, Clone)]
pub struct Cycles {
    // Circular lists of Vertex Ids representing faces
    cycles: Vec<Cycle>,
}

impl Cycles {
    pub fn new(cycles: Vec<Vec<VertexId>>) -> Self {
        Self {
            cycles: cycles.into_iter().map(Cycle).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.cycles.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cycle> {
        self.cycles.iter()
    }
}

impl Index<usize> for Cycle {
    type Output = VertexId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index % self.0.len()]
    }
}

impl Index<usize> for Cycles {
    type Output = Cycle;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cycles[index % self.cycles.len()]
    }
}
impl IndexMut<usize> for Cycle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index % self.0.len();
        &mut self.0[i]
    }
}

impl Cycle {
    pub fn from(vertices: Vec<VertexId>) -> Self {
        Self(vertices)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn delete(&mut self, v: VertexId) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter_map(|u| {
                use std::cmp::Ordering::*;
                match v.cmp(&u) {
                    Equal => None,
                    Less => Some(u - 1),
                    Greater => Some(u),
                }
            })
            .collect::<Vec<_>>();
    }

    pub fn replace(&mut self, old: VertexId, new: VertexId) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter_map(|v| {
                if v == new {
                    None
                } else if v == old {
                    Some(new)
                } else {
                    Some(v)
                }
            })
            .collect();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }

    pub fn contains(&self, v: &VertexId) -> bool {
        self.0.contains(v)
    }
}

impl From<Vec<[VertexId; 2]>> for Cycle {
    fn from(mut edges: Vec<[VertexId; 2]>) -> Self {
        let mut first = false;
        let mut face = vec![edges[0][0]];
        while !edges.is_empty() {
            let v = if first {
                *face.first().unwrap()
            } else {
                *face.last().unwrap()
            };
            if let Some(i) = edges.iter().position(|e| e.contains(&v)) {
                let next = if edges[i][0] == v {
                    edges[i][1]
                } else {
                    edges[i][0]
                };
                if !face.contains(&next) {
                    face.push(next);
                }
                edges.remove(i);
            } else {
                first ^= true;
            }
        }
        Self(face)
    }
}
impl Cycles {
    pub fn delete(&mut self, v: VertexId) {
        for cycle in &mut self.cycles {
            cycle.delete(v);
        }
    }

    /// Replace all occurrence of one vertex with another
    pub fn replace(&mut self, old: VertexId, new: VertexId) {
        for cycle in &mut self.cycles {
            cycle.replace(old, new);
        }
    }
}
