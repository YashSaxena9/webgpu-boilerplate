
@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

struct StaticUniform {
    screen_width: f32,
    screen_height: f32
}

struct DynamicUniform {
    elapsed_time: f32,
    delta_time: f32,
    mouse_x: f32,
    mouse_y: f32,
}

@group(0) @binding(0)
var<uniform> static_uniform: StaticUniform;

@group(0) @binding(1)
var<uniform> dynamic_uniform: DynamicUniform;

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(
        frag_coord.x / static_uniform.screen_width, // dynamic_uniform.mouse_x / static_uniform.screen_width,
        frag_coord.y / static_uniform.screen_height, // dynamic_uniform.mouse_y / static_uniform.screen_height,
        0.0,
        1.0
    );
}
