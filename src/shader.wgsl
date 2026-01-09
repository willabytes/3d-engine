struct Camera {
    position: vec3<f32>,
    angle_horizontal: f32,
    angle_vertical: f32,
    scale_factor: f32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) sin_horizontal: f32,
    @location(3) cos_horizontal: f32,
    @location(4) sin_vertical: f32,
    @location(5) cos_vertical: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var inverse_vertical = mat3x3<f32>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, model.cos_vertical, model.sin_vertical),
        vec3<f32>(0.0, -model.sin_vertical, model.cos_vertical),
    );

    var kek = model.cos_horizontal;

    var inverse_horizontal = mat3x3<f32>(
        vec3<f32>(model.cos_horizontal, 0.0, -model.sin_horizontal),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 0.0),
    );

    var inverse = mat3x3<f32>(
        vec3<f32>(model.cos_horizontal, 0.0, -model.sin_horizontal),
        vec3<f32>(model.sin_horizontal * model.sin_vertical, model.cos_vertical, model.cos_horizontal * model.sin_vertical),
        vec3<f32>(model.sin_horizontal * model.cos_vertical, -model.sin_vertical, model.cos_horizontal * model.cos_vertical),
    );
    

    var result_1 = inverse * model.position;
    var result_2 = result_1 / result_1.z;

    out.color = model.color;
	out.clip_position = vec4<f32>(result_2.x, result_2.y, 0.0, 1.0);
	//out.clip_position = vec4<f32>(result_1.x, result_1.y, 0.0, 1.0);
	//out.clip_position = vec4<f32>(model.position.x, model.position.y, 0.0, 1.0);

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.1);
}
