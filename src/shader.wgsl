struct Camera {
    position: vec3<f32>,
    angle_horizontal: f32,
    angle_vertical: f32,
    scale_factor: f32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) camera_position: vec3<f32>,
    @location(3) camera_matrix_column_1: vec3<f32>,
    @location(4) camera_matrix_column_2: vec3<f32>,
    @location(5) camera_matrix_column_3: vec3<f32>,
	@location(6) depth_factor: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var render_distance: f32 = 1000.0;

	var camera_matrix = mat3x3f(
		model.camera_matrix_column_1,
		model.camera_matrix_column_2,
		model.camera_matrix_column_3,
	);

    var result = camera_matrix * (model.position - model.camera_position);

    var factor: f32;
    if result.z > 0.0 {
        factor = 1.0 / (model.depth_factor * result.z);
        result.z = result.z / render_distance;
    } else {
        factor = 1.0;
    }

    result.x *= factor;
    result.y *= factor;

    out.color = model.color;
	out.clip_position = vec4<f32>(result, 1.0);

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.1);
}
