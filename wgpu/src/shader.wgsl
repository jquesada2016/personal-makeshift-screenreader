struct VertexInput {
  @location(0) position: vec2<f32>,
  @location(1) color: vec3<f32>,
  @location(2) transform_0: vec4<f32>,
  @location(3) transform_1: vec4<f32>,
  @location(4) transform_2: vec4<f32>,
  @location(5) transform_3: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let transform = mat4x4(
        input.transform_0,
        input.transform_1,
        input.transform_2,
        input.transform_3,
    );

    var output = VertexOutput();

    output.position = transform * vec4(input.position, 0.0, 1.0);
    output.color = input.color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(input.color, 1.0);
}