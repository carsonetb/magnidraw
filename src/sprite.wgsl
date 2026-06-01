struct CameraUniform {
    proj: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct InstanceInput {
    @location(2) transform: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) uv_rect: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    out.uv = model.position.xy * instance.uv_rect.zw + instance.uv_rect.xy;
    out.color = instance.color;

    out.clip_position = camera.proj * vec4(
        model.position * vec3(instance.transform.zw, 0.0) + vec3(instance.transform.xy, 0.0), 1.0
    );

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(t_diffuse, s_diffuse, in.uv);

    let final_color = texture_color * in.color;

    if final_color.a < 0.01 {
        discard;
    }

    return final_color;
}
