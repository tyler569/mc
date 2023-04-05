use wgpu::SurfaceConfiguration;

#[derive(Debug)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub dir: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            // +z is out of the screen
            eye: (0.0, 0.0, 5.0).into(),
            // have it look at the origin
            dir: -cgmath::Vector3::unit_z(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self, config: &SurfaceConfiguration) -> cgmath::Matrix4<f32> {
        let aspect = config.width as f32 / config.height as f32;

        let view = cgmath::Matrix4::look_to_rh(self.eye, self.dir, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
