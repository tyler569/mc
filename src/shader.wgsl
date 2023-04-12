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
fn vertex_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera * vec4<f32>(vertex.position, 1.);
    out.uv = vertex.uv;
    out.texture_index = vertex.texture_index;
    out.position = out.clip_position.xyz;
    out.normal = vertex.normal;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample_u = floor(in.texture_index / 16.) / 16.;
    let sample_v = floor(in.texture_index % 16.) / 16.;
    let sample = textureSample(t_diffuse, s_diffuse, vec2<f32>(in.uv.x / 16. + sample_u, in.uv.y / 16. + sample_v));
    var darken: f32 = 1.;
    if (in.normal.y < 0.) { darken = 0.6; }
    if (in.normal.x != 0.) { darken = 0.8; }
    if (in.normal.z != 0.) { darken = 0.9; }
    return vec4<f32>(sample.rgb * darken, sample.a);
}