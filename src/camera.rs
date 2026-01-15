pub mod camera {
    use std::f32::consts::PI;

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Camera {
        pub position: [f32; 3],
        pub angle_h: f32,
        pub angle_v: f32,
        pub depth_factor: f32,
    }

    impl Camera {
        pub fn new(position: [f32; 3], angle_h: f32, angle_v: f32, depth_factor: f32) -> Self {
            Self {
                position,
                angle_h,
                angle_v,
                depth_factor,
            }
        }

        pub fn matrix(&self) -> [[f32; 3]; 3] {
            let sin_h = self.angle_h.sin();
            let cos_h = self.angle_h.cos();
            let sin_v = self.angle_v.sin();
            let cos_v = self.angle_v.cos();

            [
                [cos_h,         0.0,    -sin_h       ],
                [sin_h * sin_v, cos_v,  cos_h * sin_v],
                [sin_h * cos_v, -sin_v, cos_h * cos_v],
            ]
        }

        pub fn direction_h(&self) -> [f32; 3] {
            [-self.angle_h.sin(), 0.0, self.angle_h.cos()]
        }
    }
}


