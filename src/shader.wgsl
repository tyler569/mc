struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) texture_index: f32,
};

@group(1) @binding(0)
var<uniform> camera: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) texture_index: f32,
    @location(3) normal: vec3<f32>,
};

@vertex
fn vertex_main(@location(0) vertex: u32) -> VertexOutput {
    var out: VertexOutput;

    let x = vertex & 0x3fu;
    let y = (vertex >> 6u) & 0x3fu;
    let z = (vertex >> 12u) & 0x3fu;
    let normal = (vertex >> 18u) & 0x7u;
    let uv = (vertex >> 21u) & 0x3u;
    let texture_index = vertex >> 23u;

    let clip_position = camera * vec4<f32>(f32(x), f32(y), f32(z), 1.);
    var fuv: vec2<f32>;
    switch (uv) {
    case 0u: { fuv = vec2<f32>(0., 0.); }
    case 1u: { fuv = vec2<f32>(0., 1.); }
    case 2u: { fuv = vec2<f32>(1., 0.); }
    case 3u: { fuv = vec2<f32>(1., 1.); }
    default: { fuv = vec2<f32>(0., 0.); }
    }

    var fnormal: vec3<f32>;
    switch (normal) {
    case 0u: { fnormal = vec3<f32>(1., 0., 0.); }
    case 1u: { fnormal = vec3<f32>(-1., 0., 0.); }
    case 2u: { fnormal = vec3<f32>(0., 1., 0.); }
    case 3u: { fnormal = vec3<f32>(0., -1., 0.); }
    case 4u: { fnormal = vec3<f32>(0., 0., 1.); }
    case 5u: { fnormal = vec3<f32>(0., 0., -1.); }
    default: { fnormal = vec3<f32>(0., 0., 1.); }
    }

    out.clip_position = clip_position;
    out.uv = fuv;
    out.normal = fnormal;
    out.texture_index = f32(texture_index);
    out.position = out.clip_position.xyz;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample_u = floor(in.texture_index % 16.) / 16. + in.uv.x / 16.;
    let sample_v = floor(in.texture_index / 16.) / 16. + in.uv.y / 16.;
    let sample = textureSample(t_diffuse, s_diffuse, vec2<f32>(sample_u, sample_v));
    var darken: f32 = 1.;
    if (in.normal.y < 0.) { darken = 0.6; }
    if (in.normal.x != 0.) { darken = 0.8; }
    if (in.normal.z != 0.) { darken = 0.9; }
    return vec4<f32>(sample.rgb * darken, sample.a);
}