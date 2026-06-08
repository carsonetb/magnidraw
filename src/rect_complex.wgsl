struct CameraUniform {
    proj: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct InstanceInput {
    @location(2) transform: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) rounding: vec4<f32>,
    @location(5) border_width: f32,
    @location(6) border_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) sdf_pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) rounding: vec4<f32>,
    @location(4) border_width: f32,
    @location(5) border_color: vec4<f32>,
}

// https://iquilezles.org/articles/distfunctions2d/
fn sd_rounded_box(point: vec2<f32>, box_half_size: vec2<f32>, rounding: vec4<f32>) -> f32 {
    var radius: f32 = 0.0;
    if point.x > 0.0 && point.y > 0.0 {
        radius = rounding.x;
    } else if point.x > 0.0 && point.y <= 0.0 {
        radius = rounding.y;
    } else if point.x <= 0.0 && point.y > 0.0 {
        radius = rounding.z;
    } else {
        radius = rounding.w;
    }

    let q = abs(point) - box_half_size + vec2(radius, radius);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2(0.0, 0.0))) - radius;
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    out.color = instance.color;
    out.size = instance.transform.zw;
    out.rounding = instance.rounding;
    out.border_width = instance.border_width;
    out.border_color = instance.border_color;

    out.sdf_pos = model.position.xy - vec2(0.5, 0.5);

    out.clip_position = camera.proj * vec4<f32>(
        model.position
            * vec3(instance.transform.zw, 0.0)
            + vec3(instance.transform.xy, 0.0),
        1.0
    );

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = in.sdf_pos * in.size;
    let half_size = in.size / 2.0;

    let dist = sd_rounded_box(pixel_pos, half_size, in.rounding);

    var alpha = 1.0 - smoothstep(0.0, 1.0, dist);

    if alpha < 0.0 {
        discard;
    }

    var color = in.color;
    if dist > -in.border_width {
        color = in.border_color;
    }

    return vec4(color.rgb, color.a * alpha);
}
