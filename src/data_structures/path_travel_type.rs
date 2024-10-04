use crate::prelude::*;

pub enum PathTravelType {
    OneWay,
    Cycle,
    GoBackAlongPath,
}

impl PathTravelType {
    pub fn apply_to_path<T: Numeric>(&self, mut path_vertices: Vec<T>) -> Vec<T> {
        if path_vertices.is_empty() {
            path_vertices
        } else {
            match self {
                Self::OneWay => path_vertices,
                Self::Cycle => {
                    path_vertices.push(*path_vertices.first().unwrap());
                    path_vertices
                }
                Self::GoBackAlongPath => {
                    let last_vertice = *path_vertices.last().unwrap();
                    path_vertices.pop();
                    let mut vertices_without_tail = path_vertices.clone();
                    vertices_without_tail.reverse();
                    path_vertices.push(last_vertice);
                    path_vertices.append(&mut vertices_without_tail);
                    path_vertices
                }
            }
        }
    }
}
