pub mod camera {
    use crate::gmlib::matrix::*;
    use std::f32::consts::PI;

    pub struct Camera {
        pub position: Vec3,
        pub angle: [f32; 2],
        pub scale_factor: f32,
        pub depth_factor: f32,
    }
    
    impl Camera {
        pub fn direction(&self) -> Vec3 {
            Vec3 {
                x_1: self.angle[0].cos() * self.angle[1].sin(),
                x_2: self.angle[0].sin(),
                x_3: self.angle[0].cos() * self.angle[1].cos(),
            }
        }

        pub fn transform(&self) -> Mat3x3 {
            let direction = self.direction();

            Mat3x3 {
                x_11: 1.0, x_12: 0.0, x_13: -direction.x_1 / direction.x_3,
                x_21: 0.0, x_22: 1.0, x_23: -direction.x_2 / direction.x_3,
                x_31: 0.0, x_32: 0.0, x_33: 1.0 / direction.x_3,
            }
        }
    }
}
