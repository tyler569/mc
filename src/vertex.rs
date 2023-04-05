pub trait VertexFormat {
    fn vertex_format() -> wgpu::VertexFormat;
}

macro_rules! vertex_formats {
    ($( $rust_type:ty => $variant:ident ),* $(,)?) => {
        $(
            impl VertexFormat for $rust_type {
                fn vertex_format() -> wgpu::VertexFormat {
                    wgpu::VertexFormat::$variant
                }
            }
        )*
    }
}

vertex_formats! {
    f32 => Float32,
    [f32; 2] => Float32x2,
    [f32; 3] => Float32x3,
    [f32; 4] => Float32x4,
    u32 => Uint32,
    [u32; 2] => Uint32x2,
    [u32; 3] => Uint32x3,
    [u32; 4] => Uint32x4,
    i32 => Sint32,
    [i32; 2] => Sint32x2,
    [i32; 3] => Sint32x3,
    [i32; 4] => Sint32x4,
}

pub trait Vertex: bytemuck::Pod {}

pub trait VertexLayout {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[macro_export]
macro_rules! default {
    (, $default:expr) => { $default };
    ($option:expr, $default:expr) => { $option };
}

#[macro_export]
macro_rules! vertex_struct {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Vertex:ident $(location: $location:literal)? {
            $(
                $(#[$inner:meta])*
                $field_vis:vis $field:ident: $type:ty,
            )*
        }
    ) => {
        $(#[$outer])*
        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        $vis struct $Vertex {
            $(
                $(#[$inner])*
                $field_vis $field: $type,
            )*
        }

        impl crate::vertex::Vertex for $Vertex {}

        impl crate::vertex::VertexLayout for $Vertex {
            fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
                use once_cell::sync::OnceCell;
                static INSTANCE: OnceCell<wgpu::VertexBufferLayout<'static>> = OnceCell::new();
                INSTANCE.get_or_init(|| {
                    let mut offset = 0;
                    let mut location = crate::default!($($location)?, 0);
                    let mut attributes = vec![];
                    
                    $(
                        attributes.push(wgpu::VertexAttribute {
                            offset: offset as wgpu::BufferAddress,
                            shader_location: location,
                            format: <$type as crate::vertex::VertexFormat>::vertex_format(),
                        });
                        location += 1;
                        offset += std::mem::size_of::<$type>();
                    )*

                    let attributes = attributes.leak();

                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<$Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes
                    }
                }).clone()
            }
        }
    }
}