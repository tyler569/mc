use crate::mesh::{Dimension, Face, Vertex};
use cgmath::Vector3;
use std::fmt::Formatter;

fn point_with(p: &Vector3<f32>, dimension: Dimension, value: f32) -> Vector3<f32> {
    match dimension {
        Dimension::X => Vector3::new(value, p.y, p.z),
        Dimension::Y => Vector3::new(p.x, value, p.z),
        Dimension::Z => Vector3::new(p.x, p.y, value),
    }
}

fn dir_get(p: &Vector3<f32>, dimension: Dimension) -> f32 {
    match dimension {
        Dimension::X => p.x,
        Dimension::Y => p.y,
        Dimension::Z => p.z,
    }
}

fn point_demux(
    dimension: Dimension,
    constant: f32,
    points: [Vector3<f32>; 2],
    indices: (usize, usize),
) -> Vector3<f32> {
    match dimension {
        Dimension::X => Vector3::new(constant, points[indices.0].y, points[indices.1].z),
        Dimension::Y => Vector3::new(points[indices.0].x, constant, points[indices.1].z),
        Dimension::Z => Vector3::new(points[indices.0].x, points[indices.1].y, constant),
    }
}

pub struct Cuboid {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
}

impl Cuboid {
    pub fn face(&self, dir: Face, uv: &[f32; 4]) -> [Vertex; 6] {
        let flip_triangle = !matches!(dir, Face::Up | Face::North | Face::East);
        let (f1, f2);

        let dim = dir.dimension();
        f1 = point_with(&self.p1, dim, dir_get(&self.p2, dim));
        f2 = self.p2;

        let triangles = if flip_triangle {
            [(0, 0), (1, 0), (0, 1), (0, 1), (1, 0), (1, 1)]
        } else {
            [(0, 0), (0, 1), (1, 0), (1, 0), (0, 1), (1, 1)]
        };

        let constant_value = match dir {
            Face::Down => self.p1.y,
            Face::North => self.p1.z,
            Face::East => self.p1.x,
            Face::Up => self.p2.y,
            Face::South => self.p2.z,
            Face::West => self.p2.x,
        };

        let vertices = triangles.map(|tri| {
            let point = point_demux(dim, constant_value, [self.p1, self.p2], tri);
            Vertex {
                position: point.into(),
                uv: [uv[tri.0 * 2], uv[tri.1 * 2 + 1]],
                normal: dir.into_vec3().into(),
                texture_index: 2.,
            }
        });
        println!("{dir:?} {vertices:?}");
        vertices
    }
}
