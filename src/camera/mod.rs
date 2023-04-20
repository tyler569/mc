use crate::uniform::Uniform;
pub use controller::CameraController;
use once_cell::sync::OnceCell;
use std::iter::Once;
use wgpu::SurfaceConfiguration;

mod camera2;
mod controller;

struct CameraView {
    eye: cgmath::Point3<f32>,
    dir: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl CameraView {
    fn matrix(&self, aspect: f32) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_to_rh(self.eye, self.dir, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

pub struct Camera {
    camera_view: CameraView,
    controller: CameraController,
    matrix: Uniform<[[f32; 4]; 4]>,
}

impl Camera {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            camera_view: CameraView {
                // +z is out of the screen
                eye: (0.0, 0.0, -5.0).into(),
                // have it look at the origin
                dir: cgmath::Vector3::unit_z(),
                // which way is "up"
                up: cgmath::Vector3::unit_y(),
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            controller: CameraController::new(0.05),
            matrix: Uniform::identity(device),
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.controller.process_events(event)
    }

    pub fn update(&mut self, config: &SurfaceConfiguration, queue: &wgpu::Queue) {
        self.controller.update_camera(&mut self.camera_view);

        let aspect = config.width as f32 / config.height as f32;
        let matrix = self.camera_view.matrix(aspect);

        self.matrix.set(queue, matrix.into());
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        self.matrix.bind_group()
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.matrix.bind_group_layout()
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
