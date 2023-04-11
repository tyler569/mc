use std::ops::{Deref, DerefMut};

use wgpu::util::DeviceExt;

pub struct Uniform<T: bytemuck::Pod + bytemuck::Zeroable> {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    value: T,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Uniform<T> {
    pub fn new(device: &wgpu::Device, value: T) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("bind_group_layout"),
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[value]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
            value,
        }
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.value]));
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn set(&mut self, queue: &wgpu::Queue, value: T) {
        self.value = value;
        self.update_buffer(queue);
    }
}

impl Uniform<[[f32; 4]; 4]> {
    pub fn identity(device: &wgpu::Device) -> Self {
        Self::new(device, cgmath::Matrix4::<f32>::from_scale(1.).into())
    }
}
