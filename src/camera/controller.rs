use super::Camera;
use crate::camera::CameraView;
use cgmath::Rotation3;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

#[derive(Debug, Default)]
pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_space_pressed: bool,
    is_shift_pressed: bool,
    reset: bool,
    is_mouse_pressed: bool,
    last_mouse_position: Option<PhysicalPosition<f64>>,
    queued_mouse_diff: (f32, f32),
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed: 1.0,
            ..Default::default()
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.is_space_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_shift_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::R => {
                        self.reset = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseInput { state, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                self.is_mouse_pressed = is_pressed;
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.is_mouse_pressed {
                    if let Some(old_position) = self.last_mouse_position {
                        self.queued_mouse_diff.0 += (position.x - old_position.x) as f32;
                        self.queued_mouse_diff.1 += (position.y - old_position.y) as f32;
                    }
                }

                self.last_mouse_position = Some(*position);
                true
            }
            _ => false,
        }
    }

    pub(super) fn update_camera(&mut self, camera: &mut CameraView) {
        let speed = self.speed * 0.1;

        use cgmath::InnerSpace;
        let forward = camera.dir;
        let forward_norm = forward.normalize();

        let right_norm = forward_norm.cross(camera.up).normalize();

        if self.reset {
            // reset the camera
        }

        let camera_rotation_x =
            cgmath::Quaternion::from_angle_x(cgmath::Rad(self.queued_mouse_diff.1 / 500.));
        let camera_rotation_y =
            cgmath::Quaternion::from_angle_y(cgmath::Rad(self.queued_mouse_diff.0 / 500.));

        let new_facing = camera_rotation_x * camera_rotation_y * camera.dir;
        camera.dir = new_facing.normalize();
        self.queued_mouse_diff = (0., 0.);

        if self.is_forward_pressed {
            camera.eye += forward_norm * speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * speed;
        }
        if self.is_right_pressed {
            camera.eye += right_norm * speed;
        }
        if self.is_left_pressed {
            camera.eye -= right_norm * speed;
        }
        if self.is_space_pressed {
            camera.eye += camera.up * speed;
        }
        if self.is_shift_pressed {
            camera.eye -= camera.up * speed;
        }

        // println!("{:?}", camera);
    }
}
