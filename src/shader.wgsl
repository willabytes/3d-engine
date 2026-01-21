struct CameraUniform {
    position: vec3<f32>,
    matrix: mat3x3<f32>,
    /*
    matrix_row_1: vec3<f32>,
    matrix_row_2: vec3<f32>,
    matrix_row_3: vec3<f32>,
    */
    //scale_factor: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) camera_position: vec3<f32>,
    @location(3) camera_matrix_row_1: vec3<f32>,
    @location(4) camera_matrix_row_2: vec3<f32>,
    @location(5) camera_matrix_row_3: vec3<f32>,
	@location(6) depth_factor: f32,
    @location(7) normal: vec3<f32>,
    @location(8) light_source: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    /*
	var camera_matrix = mat3x3<f32>(
		camera.matrix_row_1,
		camera.matrix_row_2,
		camera.matrix_row_3,
	);
    */

    var result = camera.matrix * (model.position - camera.position);

	out.clip_position = vec4<f32>(result, result.z * model.depth_factor);

    // With lighting
    /*
    var lighting_factor = dot(model.normal, normalize(model.position - model.light_source));
    if lighting_factor < 0.2 { lighting_factor = 0.2; }
    out.color = model.color * lighting_factor;
    */

    // Without lighting
    out.color = model.color;

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
