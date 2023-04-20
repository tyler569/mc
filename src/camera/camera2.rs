use crate::uniform::Uniform;
use cgmath::{vec3, Vector3};
use std::f32::consts::PI;
use std::time::Instant;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::WindowEvent;

struct Camera {
    initial_position: Vector3<f32>,
    position: Vector3<f32>,
    horizontal_angle: f32,
    vertical_angle: f32,
    fov: f32,
    speed: f32,
    mouse_speed: f32,
    last_time: Instant,
    mouse_capture: bool,

    last_mouse_position: Option<PhysicalPosition<f32>>,
    cursor_x_delta: f32,
    cursor_y_delta: f32,

    matrix: Uniform<[[f32; 4]; 4]>,
}

impl Camera {
    fn new(device: &wgpu::Device) -> Self {
        Self {
            initial_position: vec3(0., 0., -5.),
            position: vec3(0., 0., 5.),
            horizontal_angle: 0.,
            vertical_angle: 0.,
            fov: 45.,
            speed: 5.,
            mouse_speed: 0.07,
            last_time: Instant::now(),
            mouse_capture: true,

            last_mouse_position: None,
            cursor_x_delta: 0.,
            cursor_y_delta: 0.,

            matrix: Uniform::identity(device),
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => true,
            _ => false,
        }
    }

    fn update(&mut self, event: &WindowEvent) {
        let now = Instant::now();
        let delta_time = now - self.last_time;

        self.horizontal_angle += delta_time.as_secs_f32() * self.cursor_x_delta * self.mouse_speed;
        self.vertical_angle += delta_time.as_secs_f32() * self.cursor_y_delta * self.mouse_speed;

        let direction = vec3(
            self.vertical_angle.cos() * self.horizontal_angle.sin(),
            self.vertical_angle.cos(),
            self.vertical_angle.cos() * self.horizontal_angle.cos(),
        );
        let right = vec3(
            self.horizontal_angle.sin() - PI / 2.,
            0.,
            self.horizontal_angle.cos() - PI / 2.,
        );
        let up = right.cross(direction);
    }
}
