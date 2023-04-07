use cgmath::SquareMatrix;
use std::collections::HashMap;
use wgpu::util::DeviceExt;

use crate::vertex_struct;

vertex_struct! {
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        uv: [f32; 2],
        texture_index: u32,
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    position: cgmath::Matrix4<f32>,
    buffer: Option<wgpu::Buffer>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Face {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            position: cgmath::Matrix4::identity(),
            buffer: None,
        }
    }

    const FACE_OFFSETS: [[f32; 18]; 6] = [
        [
            0., 0., 0., 0., 1., 0., 1., 0., 0., 0., 1., 0., 1., 1., 0., 1., 0., 0.,
        ], // Front
        [
            1., 0., 0., 1., 1., 0., 1., 0., 1., 1., 1., 0., 1., 1., 1., 1., 0., 1.,
        ], // Right
        [
            1., 0., 1., 1., 1., 1., 0., 0., 1., 1., 1., 1., 0., 1., 1., 0., 0., 1.,
        ], // Back
        [
            0., 0., 1., 0., 1., 1., 0., 0., 0., 0., 1., 1., 0., 1., 0., 0., 0., 0.,
        ], // Left
        [
            0., 1., 0., 0., 1., 1., 1., 1., 0., 0., 1., 1., 1., 1., 1., 1., 1., 0.,
        ], // Top
        [
            0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 1., 1., 0., 0., 1., 0., 1.,
        ], // Bottom
    ];

    const UV_OFFSETS: [[f32; 12]; 6] = [
        [1., 1., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1.], // Front
        [1., 1., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1.], // Right
        [1., 1., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1.], // Back
        [1., 1., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1.], // Left
        [1., 1., 1., 0., 0., 1., 1., 0., 0., 0., 0., 1.], // Top
        [1., 1., 0., 1., 1., 0., 1., 0., 0., 1., 0., 0.], // Bottom
    ];

    const NORMALS: [[f32; 3]; 6] = [
        [-1., 0., 0.], // Front
        [1., 0., 0.],  // Back
        [0., 0., -1.], // Right
        [0., 0., 1.],  // Left
        [0., 1., 0.],  // Top
        [0., -1., 0.], // Bottom
    ];

    pub fn emit_face(&mut self, face: Face, position: (f32, f32, f32), texture_index: u32) {
        let offsets = &Self::FACE_OFFSETS[face as usize];
        let uvs = &Self::UV_OFFSETS[face as usize];
        let normal = &Self::NORMALS[face as usize];

        for (o, u) in offsets.chunks(3).zip(uvs.chunks(2)) {
            self.vertices.push(Vertex {
                position: [position.0 + o[0], position.1 + o[1], position.2 + o[2]],
                uv: [u[0], u[1]],
                normal: *normal,
                texture_index,
            })
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.position = cgmath::Matrix4::identity();
    }

    pub fn set_position(&mut self, matrix: cgmath::Matrix4<f32>) {
        self.position = matrix;
    }

    pub fn texture_demo() -> Self {
        let mut mesh = Self::new();

        let to_f = |x, y, z| (x as f32, y as f32, z as f32);

        for x in 0..16 {
            for y in 0..16 {
                let position = to_f(x * 2, y * 2, 0);
                let texture_index = y * 16 + x;
                mesh.emit_face(Face::North, position, texture_index);
                mesh.emit_face(Face::East, position, texture_index);
                mesh.emit_face(Face::South, position, texture_index);
                mesh.emit_face(Face::West, position, texture_index);
                mesh.emit_face(Face::Up, position, texture_index);
                mesh.emit_face(Face::Down, position, texture_index);
            }
        }

        mesh
    }

    pub fn default() -> Self {
        let mut mesh = Self::new();

        let mut chunk = HashMap::<(i32, i32, i32), u32>::new();
        for x in 0..16 {
            for y in 0..8 {
                for z in 0..16 {
                    chunk.insert((x, y, z), 1);
                }
            }
        }
        chunk.insert((4, 8, 4), 10);
        chunk.insert((4, 9, 4), 10);

        let to_f = |x, y, z| (x as f32, y as f32, z as f32);

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = chunk[&(x, y, z)];

                    if block != 0 && (x - 1 < 0 || chunk[&(x - 1, y, z)] == 0) {
                        mesh.emit_face(Face::West, to_f(x, y, z), block)
                    }
                    if block != 0 && (x + 1 > 15 || chunk[&(x + 1, y, z)] == 0) {
                        mesh.emit_face(Face::East, to_f(x, y, z), block)
                    }
                    if block != 0 && (z - 1 < 0 || chunk[&(x, y, z - 1)] == 0) {
                        mesh.emit_face(Face::South, to_f(x, y, z), block)
                    }
                    if block != 0 && (z + 1 > 15 || chunk[&(x, y, z + 1)] == 0) {
                        mesh.emit_face(Face::North, to_f(x, y, z), block)
                    }
                    if block != 0 && (y - 1 < 0 || chunk[&(x, y - 1, z)] == 0) {
                        mesh.emit_face(Face::Down, to_f(x, y, z), block)
                    }
                    if block != 0 && (y + 1 > 15 || chunk[&(x, y + 1, z)] == 0) {
                        mesh.emit_face(Face::Up, to_f(x, y, z), block)
                    }
                }
            }
        }

        mesh
    }

    pub fn build(&mut self, device: &wgpu::Device) {
        self.buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&self.vertices),
            }),
        );
    }

    pub fn render(&self) {}
}
