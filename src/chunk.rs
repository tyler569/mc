use crate::mesh::Mesh;

pub struct Chunk {
    data: [u16; Self::SIZE_X * Self::SIZE_Y * Self::SIZE_Z],
    x: i32,
    y: i32,
    z: i32,

    mesh: Option<Mesh>,
}

impl Chunk {
    pub const SIZE_X: usize = 32;
    pub const SIZE_Y: usize = 32;
    pub const SIZE_Z: usize = 32;

    pub fn new((x, y, z): (i32, i32, i32)) -> Self {
        Self {
            data: [0; Self::SIZE_X * Self::SIZE_Y * Self::SIZE_Z],
            x,
            y,
            z,
            mesh: None,
        }
    }

    pub fn default() -> Self {
        let mut chunk = Self::new((0, 0, 0));
        for y in 0..8 {
            for z in 0..Self::SIZE_Z {
                for x in 0..Self::SIZE_X {
                    chunk.set(x, y, z, (x + z) as u16)
                }
            }
        }
        chunk
    }

    pub fn generate_mesh(&mut self) {
        self.mesh = Some(Mesh::from_chunk(self));
    }

    pub fn indexof(x: usize, y: usize, z: usize) -> usize {
        x + z * Self::SIZE_X + y * Self::SIZE_X * Self::SIZE_Z
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u16 {
        self.data[Self::indexof(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, v: u16) {
        self.data[Self::indexof(x, y, z)] = v;
    }
}
