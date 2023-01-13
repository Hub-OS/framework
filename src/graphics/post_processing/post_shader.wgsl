struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@location(0) v_in: vec2<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(v_in, 0.0, 1.0);
    output.uv = v_in * 0.5 + 0.5;
    output.uv.y = 1.0 - output.uv.y;

    return output;
}

// example
@group(1) @binding(0)
var txture: texture_2d<f32>;
@group(1) @binding(1)
var smplr: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(txture, smplr, uv);
}
