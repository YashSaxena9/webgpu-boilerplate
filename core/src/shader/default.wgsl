
@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

struct ScreenSize {
    width: f32,
    height: f32
}

@group(0) @binding(0)
var<uniform> screen_size: ScreenSize;

@group(0) @binding(1)
var<uniform> time: f32;

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(
        frag_coord.x / screen_size.width,
        frag_coord.y / screen_size.height,
        abs((time * 255) % 255),
        1.0
    );
}
