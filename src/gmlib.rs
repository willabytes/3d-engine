//
// Feature list:
//
// * Vec2
//   - from([f32; 2]) -> Vec2
//   - norm(self) -> Vec2
//   - to_array(self) -> [f32; 2]
//   - Scalar product: Vec2 * Vec2 -> f32
//   - Addition: Vec2 + Vec2 -> Vec2
//   - Subtraction: Vec2 - Vec2 -> Vec2
//   - Scalar multiplication: Vec2 * f32 -> Vec2
//   - Scalar division: Vec2 / f32 -> Vec2
//   - Negation: -Vec2 -> Vec2
//   - Implements std::fmt::Display
//
// * Vec3
//   - from([f32; 3]) -> Vec3
//   - norm(self) -> Vec3
//   - to_array(self) -> [f32; 3]
//   - Scalar product: Vec3 * Vec3 -> f32
//   - Cross product: Vec3 % Vec3 -> Vec3
//   - Addition: Vec3 + Vec3 -> Vec3
//   - Subtraction: Vec3 - Vec3 -> Vec3
//   - Scalar multiplication: Vec3 * f32 -> Vec3
//   - Scalar division: Vec3 / f32 -> Vec3
//   - Negation: -Vec3 -> Vec3
//   - Implements std::fmt::Display
//
// * Vec4
//   - from([f32; 4]) -> Vec4
//   - norm(self) -> Vec4
//   - to_array(self) -> [f32; 4]
//   - Scalar product: Vec4 * Vec4 -> f32
//   - Addition: Vec4 + Vec4 -> Vec4
//   - Subtraction: Vec4 - Vec4 -> Vec4
//   - Scalar multiplication: Vec4 * f32 -> Vec4
//   - Scalar division: Vec4 / f32 -> Vec4
//   - Negation: -Vec4 -> Vec4
//   - Implements std::fmt::Display
//
// * Mat2x2
//   - from([f32; 4]) -> Mat2x2
//   - pow(mut self, exponent: u32) -> Mat2x2
//   - determinant(&self) -> f32
//   - inverse(&self) -> Self
//   - to_array(self) -> [[f32; 2]; 2]
//   - Addition: Mat2x2 + Mat2x2 -> Mat2x2
//   - Subtraction: Mat2x2 - Mat2x2 -> Mat2x2
//   - Matrix-scalar multiplication: Mat2x2 * f32 -> Mat2x2
//   - Matrix-scalar division: Mat2x2 / f32 -> Mat2x2
//   - Matrix-matrix multiplication: Mat2x2 * Mat2x2 -> Mat2x2
//   - Matrix-vector multiplication: Mat2x2 * Vec2 -> Vec2
//   - Negation: -Mat2x2 -> Mat2x2
//   - Implements std::fmt::Display
//   - Unit matrix constant: UNIT_MAT2X2
//
// * Mat3x3
//   - from([f32; 9]) -> Mat3x3
//   - pow(mut self, exponent: u32) -> Mat3x3
//   - determinant(&self) -> f32
//   - inverse(&self) -> Self
//   - to_array(self) -> [[f32; 3]; 3]
//   - Addition: Mat3x3 + Mat3x3 -> Mat3x3
//   - Subtraction: Mat3x3 - Mat3x3 -> Mat3x3
//   - Matrix-scalar multiplication: Mat3x3 * f32 -> Mat3x3
//   - Matrix-scalar division: Mat3x3 / f32 -> Mat3x3
//   - Matrix-matrix multiplication: Mat3x3 * Mat3x3 -> Mat3x3
//   - Matrix-vector multiplication: Mat3x3 * Vec3 -> Vec3
//   - Negation: -Mat3x3 -> Mat3x3
//   - Implements std::fmt::Display
//   - Unit matrix constant: UNIT_MAT3X3
//
// * Mat4x4
//   - from([f32; 16]) -> Mat4x4
//   - pow(mut self, exponent: u32) -> Mat4x4
//   - determinant(&self) -> f32
//   - inverse(&self) -> Mat4x4
//   - to_array(self) -> [[f32; 4]; 4]
//   - Addition: Mat4x4 + Mat4x4 -> Mat4x4
//   - Subtraction: Mat4x4 - Mat4x4 -> Mat4x4
//   - Matrix-scalar multiplication: Mat4x4 * f32 -> Mat4x4
//   - Matrix-scalar division: Mat4x4 / f32 -> Mat4x4
//   - Matrix-matrix multiplication: Mat4x4 * Mat4x4 -> Mat4x4
//   - Matrix-vector multiplication: Mat4x4 * Vec4 -> Vec4
//   - Negation: -Mat4x4 -> Mat4x4
//   - Implements std::fmt::Display
//   - Unit matrix constant: UNIT_MAT4X4
//

pub mod matrix {
    #[derive(Debug, Clone, Copy)]
    pub struct Vec2 {
        pub x_1: f32,
        pub x_2: f32,
    }

    impl Vec2 {
        pub fn from(values: [f32; 2]) -> Self {
            Self {
                x_1: values[0],
                x_2: values[1],
            }
        }

        pub fn norm(self) -> Self {
            self / f32::sqrt(
                self.x_1 * self.x_1 +
                self.x_2 * self.x_2
            )
        }

        pub fn to_array(self) -> [f32; 2] {
            [self.x_1, self.x_2]
        }
    }

    // Scalar product
    impl std::ops::Mul<Vec2> for Vec2 {
        type Output = f32;

        fn mul(self, rhs: Vec2) -> f32 {
            self.x_1 * rhs.x_1 +
            self.x_2 * rhs.x_2
        }
    }

    impl std::ops::Add<Vec2> for Vec2 {
        type Output = Self;

        fn add(self, rhs: Vec2) -> Self {
            Self {
                x_1: self.x_1 + rhs.x_1,
                x_2: self.x_2 + rhs.x_2,
            }
        }
    }

    impl std::ops::Sub<Vec2> for Vec2 {
        type Output = Self;

        fn sub(self, rhs: Vec2) -> Self {
            Self {
                x_1: self.x_1 - rhs.x_1,
                x_2: self.x_2 - rhs.x_2,
            }
        }
    }

    // Scalar multiplication
    impl std::ops::Mul<f32> for Vec2 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 * rhs,
                x_2: self.x_2 * rhs,
            }
        }
    }

    // Scalar division
    impl std::ops::Div<f32> for Vec2 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 / rhs,
                x_2: self.x_2 / rhs,
            }
        }
    }

    // Vec2 negation
    impl std::ops::Neg for Vec2 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_1: -self.x_1,
                x_2: -self.x_2,
            }
        }
    }

    impl std::fmt::Display for Vec2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}\n{}",
                self.x_1,
                self.x_2,
            )
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Vec3 {
        pub x_1: f32,
        pub x_2: f32,
        pub x_3: f32,
    }

    impl Vec3 {
        pub fn from(values: [f32; 3]) -> Self {
            Self {
                x_1: values[0],
                x_2: values[1],
                x_3: values[2],
            }
        }

        pub fn magnitude(self) -> f32 {
            f32::sqrt(
                self.x_1 * self.x_1 +
                self.x_2 * self.x_2 +
                self.x_3 * self.x_3
            )
        }

        pub fn normalize(self) -> Self {
            self / self.magnitude()
        }

        pub fn to_array(self) -> [f32; 3] {
            [self.x_1, self.x_2, self.x_3]
        }
    }

    // Scalar product
    impl std::ops::Mul<Vec3> for Vec3 {
        type Output = f32;

        fn mul(self, rhs: Vec3) -> f32 {
            self.x_1 * rhs.x_1 +
            self.x_2 * rhs.x_2 +
            self.x_3 * rhs.x_3
        }
    }

    // Cross product
    impl std::ops::Rem<Vec3> for Vec3 {
        type Output = Self;

        fn rem(self, rhs: Vec3) -> Self {
            Self {
                x_1: self.x_3 * rhs.x_2 - self.x_2 * rhs.x_3,
                x_2: self.x_1 * rhs.x_3 - self.x_3 * rhs.x_1,
                x_3: self.x_2 * rhs.x_1 - self.x_1 * rhs.x_2,
            }
        }
    }
    
    impl std::ops::Add<Vec3> for Vec3 {
        type Output = Self;

        fn add(self, rhs: Vec3) -> Self {
            Self {
                x_1: self.x_1 + rhs.x_1,
                x_2: self.x_2 + rhs.x_2,
                x_3: self.x_3 + rhs.x_3,
            }
        }
    }

    impl std::ops::Sub<Vec3> for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Vec3) -> Self {
            Self {
                x_1: self.x_1 - rhs.x_1,
                x_2: self.x_2 - rhs.x_2,
                x_3: self.x_3 - rhs.x_3,
            }
        }
    }

    // Scalar multiplication
    impl std::ops::Mul<f32> for Vec3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 * rhs,
                x_2: self.x_2 * rhs,
                x_3: self.x_3 * rhs,
            }
        }
    }

    // Scalar division
    impl std::ops::Div<f32> for Vec3 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 / rhs,
                x_2: self.x_2 / rhs,
                x_3: self.x_3 / rhs,
            }
        }
    }

    // Vec3 negation
    impl std::ops::Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_1: -self.x_1,
                x_2: -self.x_2,
                x_3: -self.x_3,
            }
        }
    }

    impl std::fmt::Display for Vec3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}\n{}\n{}",
                self.x_1,
                self.x_2,
                self.x_3,
            )
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Vec4 {
        pub x_1: f32,
        pub x_2: f32,
        pub x_3: f32,
        pub x_4: f32,
    }

    impl Vec4 {
        pub fn from(values: [f32; 4]) -> Self {
            Self {
                x_1: values[0],
                x_2: values[1],
                x_3: values[2],
                x_4: values[3],
            }
        }

        pub fn norm(self) -> Self {
            self / f32::sqrt(
                self.x_1 * self.x_1 +
                self.x_2 * self.x_2 +
                self.x_3 * self.x_3 +
                self.x_4 * self.x_4
            )
        }

        pub fn to_array(self) -> [f32; 4] {
            [self.x_1, self.x_2, self.x_3, self.x_4]
        }
    }

    // Scalar product
    impl std::ops::Mul<Vec4> for Vec4 {
        type Output = f32;

        fn mul(self, rhs: Vec4) -> f32 {
            self.x_1 * rhs.x_1 +
            self.x_2 * rhs.x_2 +
            self.x_3 * rhs.x_3 +
            self.x_4 * rhs.x_4
        }
    }

    impl std::ops::Add<Vec4> for Vec4 {
        type Output = Self;

        fn add(self, rhs: Vec4) -> Self {
            Self {
                x_1: self.x_1 + rhs.x_1,
                x_2: self.x_2 + rhs.x_2,
                x_3: self.x_3 + rhs.x_3,
                x_4: self.x_4 + rhs.x_4,
            }
        }
    }

    impl std::ops::Sub<Vec4> for Vec4 {
        type Output = Self;

        fn sub(self, rhs: Vec4) -> Self {
            Self {
                x_1: self.x_1 - rhs.x_1,
                x_2: self.x_2 - rhs.x_2,
                x_3: self.x_3 - rhs.x_3,
                x_4: self.x_4 - rhs.x_4,
            }
        }
    }

    // Scalar multiplication
    impl std::ops::Mul<f32> for Vec4 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 * rhs,
                x_2: self.x_2 * rhs,
                x_3: self.x_3 * rhs,
                x_4: self.x_4 * rhs,
            }
        }
    }

    // Scalar division
    impl std::ops::Div<f32> for Vec4 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_1: self.x_1 / rhs,
                x_2: self.x_2 / rhs,
                x_3: self.x_3 / rhs,
                x_4: self.x_4 / rhs,
            }
        }
    }

    // Vec4 negation
    impl std::ops::Neg for Vec4 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_1: -self.x_1,
                x_2: -self.x_2,
                x_3: -self.x_3,
                x_4: -self.x_4,
            }
        }
    }

    impl std::fmt::Display for Vec4 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}\n{}\n{}\n{}",
                self.x_1,
                self.x_2,
                self.x_3,
                self.x_4,
            )
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Mat2x2 {
        pub x_11: f32, pub x_12: f32,
        pub x_21: f32, pub x_22: f32,
    }    

    impl Mat2x2 {
        pub fn from(values: [f32; 4]) -> Self {
            Self {
                x_11: values[0], x_12: values[1],
                x_21: values[2], x_22: values[3],
            }
        }

        pub fn pow(mut self, exponent: u32) -> Self {
            if exponent == 0 {
                UNIT_MAT2X2
            } else {
                for i in 0..(exponent - 1) { self = self * self; }
                
                self
            }
        }

        pub fn determinant(&self) -> f32 {
            self.x_11 * self.x_22 -
            self.x_12 * self.x_21
        }

        pub fn inverse(&self) -> Self {
            let determinant = self.determinant();

            Mat2x2 {
                x_11: self.x_22  / determinant,
                x_12: -self.x_12 / determinant,
                x_21: -self.x_21 / determinant,
                x_22: self.x_11  / determinant,
            }
        }

        pub fn to_array(self) -> [[f32; 2]; 2] {
            [
                [self.x_11, self.x_12],
                [self.x_21, self.x_22],
            ]
        }
    }

    // Mat2x2 addition
    impl std::ops::Add<Mat2x2> for Mat2x2 {
        type Output = Self;

        fn add(self, rhs: Mat2x2) -> Self {
            Self {
                x_11: self.x_11 + rhs.x_11,
                x_12: self.x_12 + rhs.x_12,
                x_21: self.x_21 + rhs.x_21,
                x_22: self.x_22 + rhs.x_22,
            }
        }
    }

    // Mat2x2 subtraction
    impl std::ops::Sub<Mat2x2> for Mat2x2 {
        type Output = Self;

        fn sub(self, rhs: Mat2x2) -> Self {
            Self {
                x_11: self.x_11 - rhs.x_11,
                x_12: self.x_12 - rhs.x_12,
                x_21: self.x_21 - rhs.x_21,
                x_22: self.x_22 - rhs.x_22,
            }
        }
    }

    // Mat2x2-f32 multiplication
    impl std::ops::Mul<f32> for Mat2x2 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 * rhs,
                x_12: self.x_12 * rhs,
                x_21: self.x_21 * rhs,
                x_22: self.x_22 * rhs,
            }
        }
    }

    // Mat2x2-f32 division
    impl std::ops::Div<f32> for Mat2x2 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 / rhs,
                x_12: self.x_12 / rhs,
                x_21: self.x_21 / rhs,
                x_22: self.x_22 / rhs,
            }
        }
    }


    // Mat2x2-Mat2x2 multiplication
    impl std::ops::Mul<Mat2x2> for Mat2x2 {
        type Output = Self;

        fn mul(self, rhs: Mat2x2) -> Self {
            Self {
                x_11: self.x_11 * rhs.x_11 + self.x_12 * rhs.x_21,
                x_12: self.x_11 * rhs.x_12 + self.x_12 * rhs.x_22,
                x_21: self.x_21 * rhs.x_11 + self.x_22 * rhs.x_21,
                x_22: self.x_21 * rhs.x_12 + self.x_22 * rhs.x_22,
            }
        }
    }

    // Mat2x2-Vec2 multiplication
    impl std::ops::Mul<Vec2> for Mat3x3 {
        type Output = Vec2;

        fn mul(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x_1: self.x_11 * rhs.x_1 + self.x_12 * rhs.x_2,
                x_2: self.x_21 * rhs.x_1 + self.x_22 * rhs.x_2,
            }
        }
    }

    // Mat2x2 negation
    impl std::ops::Neg for Mat2x2 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_11: -self.x_11,
                x_12: -self.x_12,
                x_21: -self.x_21,
                x_22: -self.x_22,
            }
        }
    }

    impl std::fmt::Display for Mat2x2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}   {}\n{}   {}",
                self.x_11,
                self.x_12,
                self.x_21,
                self.x_22,
            )
        }
    }

    pub const UNIT_MAT2X2: Mat2x2 = Mat2x2 {
        x_11: 1.0, x_12: 0.0,
        x_21: 0.0, x_22: 1.0,
    };


    #[derive(Debug, Clone, Copy)]
    pub struct Mat3x3 {
        pub x_11: f32, pub x_12: f32, pub x_13: f32,
        pub x_21: f32, pub x_22: f32, pub x_23: f32,
        pub x_31: f32, pub x_32: f32, pub x_33: f32,
    }    

    impl Mat3x3 {
        pub fn from(values: [f32; 9]) -> Self {
            Self {
                x_11: values[0], x_12: values[1], x_13: values[2],
                x_21: values[3], x_22: values[4], x_23: values[5],
                x_31: values[6], x_32: values[7], x_33: values[8],
            }
        }

        pub fn pow(mut self, exponent: u32) -> Self {
            if exponent == 0 {
                UNIT_MAT3X3
            } else {
                for i in 0..(exponent - 1) { self = self * self; }
                
                self
            }
        }

        pub fn determinant(&self) -> f32 {
            self.x_11 * self.x_22 * self.x_33 +
            self.x_12 * self.x_23 * self.x_31 +
            self.x_13 * self.x_21 * self.x_32 -
            self.x_11 * self.x_23 * self.x_32 -
            self.x_12 * self.x_21 * self.x_33 -
            self.x_13 * self.x_22 * self.x_31
        }

        pub fn inverse(&self) -> Self {
            let temp_11 = self.x_22 * self.x_33 - self.x_23 * self.x_32;
            let temp_21 = self.x_23 * self.x_31 - self.x_21 * self.x_33;
            let temp_31 = self.x_21 * self.x_32 - self.x_22 * self.x_31;

            let det = self.x_11 * temp_11 + self.x_12 * temp_21 + self.x_13 * temp_31;

            Self {
                x_11: (self.x_22 * self.x_33 - self.x_23 * self.x_32) / det,
                x_12: (self.x_13 * self.x_32 - self.x_12 * self.x_33) / det,
                x_13: (self.x_12 * self.x_23 - self.x_13 * self.x_22) / det,
                x_21: (self.x_23 * self.x_31 - self.x_21 * self.x_33) / det,
                x_22: (self.x_11 * self.x_33 - self.x_13 * self.x_31) / det,
                x_23: (self.x_13 * self.x_21 - self.x_11 * self.x_23) / det,
                x_31: (self.x_21 * self.x_32 - self.x_22 * self.x_31) / det,
                x_32: (self.x_12 * self.x_31 - self.x_11 * self.x_32) / det,
                x_33: (self.x_11 * self.x_22 - self.x_12 * self.x_21) / det,
            }
        }

        pub fn to_array(self) -> [[f32; 3]; 3] {
            [
                [self.x_11, self.x_12, self.x_13],
                [self.x_21, self.x_22, self.x_23],
                [self.x_31, self.x_32, self.x_33],
            ]
        }
    }

    // Mat3x3 addition
    impl std::ops::Add<Mat3x3> for Mat3x3 {
        type Output = Self;

        fn add(self, rhs: Mat3x3) -> Self {
            Self {
                x_11: self.x_11 + rhs.x_11,
                x_12: self.x_12 + rhs.x_12,
                x_13: self.x_13 + rhs.x_13,
                x_21: self.x_21 + rhs.x_21,
                x_22: self.x_22 + rhs.x_22,
                x_23: self.x_23 + rhs.x_23,
                x_31: self.x_31 + rhs.x_31,
                x_32: self.x_32 + rhs.x_32,
                x_33: self.x_33 + rhs.x_33,
            }
        }
    }

    // Mat3x3 subtraction
    impl std::ops::Sub<Mat3x3> for Mat3x3 {
        type Output = Self;

        fn sub(self, rhs: Mat3x3) -> Self {
            Self {
                x_11: self.x_11 - rhs.x_11,
                x_12: self.x_12 - rhs.x_12,
                x_13: self.x_13 - rhs.x_13,
                x_21: self.x_21 - rhs.x_21,
                x_22: self.x_22 - rhs.x_22,
                x_23: self.x_23 - rhs.x_23,
                x_31: self.x_31 - rhs.x_31,
                x_32: self.x_32 - rhs.x_32,
                x_33: self.x_33 - rhs.x_33,
            }
        }
    }

    // Mat3x3-f32 multiplication
    impl std::ops::Mul<f32> for Mat3x3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 * rhs,
                x_12: self.x_12 * rhs,
                x_13: self.x_13 * rhs,
                x_21: self.x_21 * rhs,
                x_22: self.x_22 * rhs,
                x_23: self.x_23 * rhs,
                x_31: self.x_31 * rhs,
                x_32: self.x_32 * rhs,
                x_33: self.x_33 * rhs,
            }
        }
    }

    // Mat3x3-f32 division
    impl std::ops::Div<f32> for Mat3x3 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 / rhs,
                x_12: self.x_12 / rhs,
                x_13: self.x_13 / rhs,
                x_21: self.x_21 / rhs,
                x_22: self.x_22 / rhs,
                x_23: self.x_23 / rhs,
                x_31: self.x_31 / rhs,
                x_32: self.x_32 / rhs,
                x_33: self.x_33 / rhs,
            }
        }
    }


    // Mat3x3-Mat3x3 multiplication
    impl std::ops::Mul<Mat3x3> for Mat3x3 {
        type Output = Self;

        fn mul(self, rhs: Mat3x3) -> Self {
            Self {
                x_11: self.x_11 * rhs.x_11 + self.x_12 * rhs.x_21 + self.x_13 * rhs.x_31,
                x_12: self.x_11 * rhs.x_12 + self.x_12 * rhs.x_22 + self.x_13 * rhs.x_32,
                x_13: self.x_11 * rhs.x_13 + self.x_12 * rhs.x_23 + self.x_13 * rhs.x_33,
                x_21: self.x_21 * rhs.x_11 + self.x_22 * rhs.x_21 + self.x_23 * rhs.x_31,
                x_22: self.x_21 * rhs.x_12 + self.x_22 * rhs.x_22 + self.x_23 * rhs.x_32,
                x_23: self.x_21 * rhs.x_13 + self.x_22 * rhs.x_23 + self.x_23 * rhs.x_33,
                x_31: self.x_31 * rhs.x_11 + self.x_32 * rhs.x_21 + self.x_33 * rhs.x_31,
                x_32: self.x_31 * rhs.x_12 + self.x_32 * rhs.x_22 + self.x_33 * rhs.x_32,
                x_33: self.x_31 * rhs.x_13 + self.x_32 * rhs.x_23 + self.x_33 * rhs.x_33,
            }
        }
    }

    // Mat3x3-Vec3 multiplication
    impl std::ops::Mul<Vec3> for Mat3x3 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Vec3 {
            Vec3 {
                x_1: self.x_11 * rhs.x_1 + self.x_12 * rhs.x_2 + self.x_13 * rhs.x_3,
                x_2: self.x_21 * rhs.x_1 + self.x_22 * rhs.x_2 + self.x_23 * rhs.x_3,
                x_3: self.x_31 * rhs.x_1 + self.x_32 * rhs.x_2 + self.x_33 * rhs.x_3,
            }
        }
    }

    // Mat3x3 negation
    impl std::ops::Neg for Mat3x3 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_11: -self.x_11,
                x_12: -self.x_12,
                x_13: -self.x_13,
                x_21: -self.x_21,
                x_22: -self.x_22,
                x_23: -self.x_23,
                x_31: -self.x_31,
                x_32: -self.x_32,
                x_33: -self.x_33,
            }
        }
    }

    impl std::fmt::Display for Mat3x3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}   {}   {}\n{}   {}   {}\n{}   {}   {}",
                self.x_11,
                self.x_12,
                self.x_13,
                self.x_21,
                self.x_22,
                self.x_23,
                self.x_31,
                self.x_32,
                self.x_33,
            )
        }
    }

    pub const UNIT_MAT3X3: Mat3x3 = Mat3x3 {
        x_11: 1.0, x_12: 0.0, x_13: 0.0,
        x_21: 0.0, x_22: 1.0, x_23: 0.0,
        x_31: 0.0, x_32: 0.0, x_33: 1.0,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Mat4x4 {
        pub x_11: f32, pub x_12: f32, pub x_13: f32, pub x_14: f32,
        pub x_21: f32, pub x_22: f32, pub x_23: f32, pub x_24: f32,
        pub x_31: f32, pub x_32: f32, pub x_33: f32, pub x_34: f32,
        pub x_41: f32, pub x_42: f32, pub x_43: f32, pub x_44: f32,
    }    

    impl Mat4x4 {
        pub fn from(values: [f32; 16]) -> Self {
            Self {
                x_11: values[0],  x_12: values[1],  x_13: values[2],  x_14: values[3],
                x_21: values[4],  x_22: values[5],  x_23: values[6],  x_24: values[7],
                x_31: values[6],  x_32: values[7],  x_33: values[8],  x_34: values[11],
                x_41: values[12], x_42: values[13], x_43: values[14], x_44: values[15],
            }
        }

        pub fn pow(mut self, exponent: u32) -> Self {
            if exponent == 0 {
                UNIT_MAT4X4
            } else {
                for i in 0..(exponent - 1) { self = self * self; }
                
                self
            }
        }

        pub fn determinant(&self) -> f32 {
            self.x_11 * Mat3x3::from([
                self.x_22, self.x_23, self.x_24,
                self.x_32, self.x_33, self.x_34,
                self.x_42, self.x_43, self.x_44,
            ]).determinant() -
            self.x_21 * Mat3x3::from([
                self.x_12, self.x_13, self.x_14,
                self.x_32, self.x_33, self.x_34,
                self.x_42, self.x_43, self.x_44,
            ]).determinant() +
            self.x_31 * Mat3x3::from([
                self.x_12, self.x_13, self.x_14,
                self.x_22, self.x_23, self.x_24,
                self.x_42, self.x_43, self.x_44,
            ]).determinant() -
            self.x_41 * Mat3x3::from([
                self.x_12, self.x_13, self.x_14,
                self.x_22, self.x_23, self.x_24,
                self.x_32, self.x_33, self.x_34,
            ]).determinant()
        }

        // Slow as tar shit. https://semath.info/src/inverse-cofactor-ex4.html
        pub fn inverse(&self) -> Self {
            let determinant_self = self.determinant();

            Self {
                x_11: Mat3x3 {
                    x_11: self.x_22, x_12: self.x_23, x_13: self.x_24,
                    x_21: self.x_32, x_22: self.x_33, x_23: self.x_34,
                    x_31: self.x_42, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / determinant_self,
                x_12: Mat3x3 {
                    x_11: self.x_12, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_32, x_22: self.x_33, x_23: self.x_34,
                    x_31: self.x_42, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / -determinant_self,
                x_13: Mat3x3 {
                    x_11: self.x_12, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_22, x_22: self.x_23, x_23: self.x_24,
                    x_31: self.x_42, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / determinant_self,
                x_14: Mat3x3 {
                    x_11: self.x_12, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_22, x_22: self.x_23, x_23: self.x_24,
                    x_31: self.x_32, x_32: self.x_33, x_33: self.x_34,
                }.determinant() / -determinant_self,
                x_21: Mat3x3 {
                    x_11: self.x_21, x_12: self.x_23, x_13: self.x_24,
                    x_21: self.x_31, x_22: self.x_33, x_23: self.x_34,
                    x_31: self.x_41, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / -determinant_self,
                x_22: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_31, x_22: self.x_33, x_23: self.x_34,
                    x_31: self.x_41, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / determinant_self,
                x_23: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_21, x_22: self.x_23, x_23: self.x_24,
                    x_31: self.x_41, x_32: self.x_43, x_33: self.x_44,
                }.determinant() / -determinant_self,
                x_24: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_13, x_13: self.x_14,
                    x_21: self.x_21, x_22: self.x_23, x_23: self.x_24,
                    x_31: self.x_31, x_32: self.x_33, x_33: self.x_34,
                }.determinant() / determinant_self,
                x_31: Mat3x3 {
                    x_11: self.x_21, x_12: self.x_22, x_13: self.x_24,
                    x_21: self.x_31, x_22: self.x_32, x_23: self.x_34,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_44,
                }.determinant() / determinant_self,
                x_32: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_14,
                    x_21: self.x_31, x_22: self.x_32, x_23: self.x_34,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_44,
                }.determinant() / -determinant_self,
                x_33: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_14,
                    x_21: self.x_21, x_22: self.x_22, x_23: self.x_24,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_44,
                }.determinant() / determinant_self,
                x_34: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_14,
                    x_21: self.x_21, x_22: self.x_22, x_23: self.x_24,
                    x_31: self.x_31, x_32: self.x_32, x_33: self.x_34,
                }.determinant() / -determinant_self,
                x_41: Mat3x3 {
                    x_11: self.x_21, x_12: self.x_22, x_13: self.x_23,
                    x_21: self.x_31, x_22: self.x_32, x_23: self.x_33,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_43,
                }.determinant() / -determinant_self,
                x_42: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_13,
                    x_21: self.x_31, x_22: self.x_32, x_23: self.x_33,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_43,
                }.determinant() / determinant_self,
                x_43: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_13,
                    x_21: self.x_21, x_22: self.x_22, x_23: self.x_23,
                    x_31: self.x_41, x_32: self.x_42, x_33: self.x_43,
                }.determinant() / -determinant_self,
                x_44: Mat3x3 {
                    x_11: self.x_11, x_12: self.x_12, x_13: self.x_13,
                    x_21: self.x_21, x_22: self.x_22, x_23: self.x_23,
                    x_31: self.x_31, x_32: self.x_32, x_33: self.x_33,
                }.determinant() / determinant_self,
            }
        }

        pub fn to_array(self) -> [[f32; 4]; 4] {
            [
                [self.x_11, self.x_12, self.x_13, self.x_14],
                [self.x_21, self.x_22, self.x_23, self.x_24],
                [self.x_31, self.x_32, self.x_33, self.x_34],
                [self.x_41, self.x_42, self.x_43, self.x_44],
            ]
        }
    }

    // Mat4x4 addition
    impl std::ops::Add<Mat4x4> for Mat4x4 {
        type Output = Mat4x4;

        fn add(self, rhs: Mat4x4) -> Self {
            Self {
                x_11: self.x_11 + rhs.x_11,
                x_12: self.x_12 + rhs.x_12,
                x_13: self.x_13 + rhs.x_13,
                x_14: self.x_14 + rhs.x_14,
                x_21: self.x_21 + rhs.x_21,
                x_22: self.x_22 + rhs.x_22,
                x_23: self.x_23 + rhs.x_23,
                x_24: self.x_24 + rhs.x_24,
                x_31: self.x_31 + rhs.x_31,
                x_32: self.x_32 + rhs.x_32,
                x_33: self.x_33 + rhs.x_33,
                x_34: self.x_34 + rhs.x_34,
                x_41: self.x_41 + rhs.x_41,
                x_42: self.x_42 + rhs.x_42,
                x_43: self.x_43 + rhs.x_43,
                x_44: self.x_44 + rhs.x_44,
            }
        }
    }

    // Mat4x4 subtraction
    impl std::ops::Sub<Mat4x4> for Mat4x4 {
        type Output = Mat4x4;

        fn sub(self, rhs: Mat4x4) -> Self {
            Self {
                x_11: self.x_11 - rhs.x_11,
                x_12: self.x_12 - rhs.x_12,
                x_13: self.x_13 - rhs.x_13,
                x_14: self.x_14 - rhs.x_14,
                x_21: self.x_21 - rhs.x_21,
                x_22: self.x_22 - rhs.x_22,
                x_23: self.x_23 - rhs.x_23,
                x_24: self.x_24 - rhs.x_24,
                x_31: self.x_31 - rhs.x_31,
                x_32: self.x_32 - rhs.x_32,
                x_33: self.x_33 - rhs.x_33,
                x_34: self.x_34 - rhs.x_34,
                x_41: self.x_41 - rhs.x_41,
                x_42: self.x_42 - rhs.x_42,
                x_43: self.x_43 - rhs.x_43,
                x_44: self.x_44 - rhs.x_44,
            }
        }
    }

    // Mat4x4-f32 multiplication
    impl std::ops::Mul<f32> for Mat4x4 {
        type Output = Mat4x4;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 * rhs,
                x_12: self.x_12 * rhs,
                x_13: self.x_13 * rhs,
                x_14: self.x_14 * rhs,
                x_21: self.x_21 * rhs,
                x_22: self.x_22 * rhs,
                x_23: self.x_23 * rhs,
                x_24: self.x_24 * rhs,
                x_31: self.x_31 * rhs,
                x_32: self.x_32 * rhs,
                x_33: self.x_33 * rhs,
                x_34: self.x_34 * rhs,
                x_41: self.x_41 * rhs,
                x_42: self.x_42 * rhs,
                x_43: self.x_43 * rhs,
                x_44: self.x_44 * rhs,
            }
        }
    }

    // Mat4x4-f32 division
    impl std::ops::Div<f32> for Mat4x4 {
        type Output = Mat4x4;

        fn div(self, rhs: f32) -> Self {
            Self {
                x_11: self.x_11 / rhs,
                x_12: self.x_12 / rhs,
                x_13: self.x_13 / rhs,
                x_14: self.x_14 / rhs,
                x_21: self.x_21 / rhs,
                x_22: self.x_22 / rhs,
                x_23: self.x_23 / rhs,
                x_24: self.x_24 / rhs,
                x_31: self.x_31 / rhs,
                x_32: self.x_32 / rhs,
                x_33: self.x_33 / rhs,
                x_34: self.x_34 / rhs,
                x_41: self.x_41 / rhs,
                x_42: self.x_42 / rhs,
                x_43: self.x_43 / rhs,
                x_44: self.x_44 / rhs,
            }
        }
    }


    // Mat4x4-Mat4x4 multiplication
    impl std::ops::Mul<Mat4x4> for Mat4x4 {
        type Output = Self;

        fn mul(self, rhs: Mat4x4) -> Self {
            Self {
                x_11: self.x_11 * rhs.x_11 + self.x_12 * rhs.x_21 + self.x_13 * rhs.x_31 + self.x_14 * rhs.x_41,
                x_12: self.x_11 * rhs.x_12 + self.x_12 * rhs.x_22 + self.x_13 * rhs.x_32 + self.x_14 * rhs.x_42,
                x_13: self.x_11 * rhs.x_13 + self.x_12 * rhs.x_23 + self.x_13 * rhs.x_33 + self.x_14 * rhs.x_43,
                x_14: self.x_11 * rhs.x_14 + self.x_12 * rhs.x_24 + self.x_13 * rhs.x_34 + self.x_14 * rhs.x_44,
                x_21: self.x_21 * rhs.x_11 + self.x_22 * rhs.x_21 + self.x_23 * rhs.x_31 + self.x_24 * rhs.x_41,
                x_22: self.x_21 * rhs.x_12 + self.x_22 * rhs.x_22 + self.x_23 * rhs.x_32 + self.x_24 * rhs.x_42,
                x_23: self.x_21 * rhs.x_13 + self.x_22 * rhs.x_23 + self.x_23 * rhs.x_33 + self.x_24 * rhs.x_43,
                x_24: self.x_21 * rhs.x_14 + self.x_22 * rhs.x_24 + self.x_23 * rhs.x_34 + self.x_24 * rhs.x_44,
                x_31: self.x_31 * rhs.x_11 + self.x_32 * rhs.x_21 + self.x_33 * rhs.x_31 + self.x_34 * rhs.x_41,
                x_32: self.x_31 * rhs.x_12 + self.x_32 * rhs.x_22 + self.x_33 * rhs.x_32 + self.x_34 * rhs.x_42,
                x_33: self.x_31 * rhs.x_13 + self.x_32 * rhs.x_23 + self.x_33 * rhs.x_33 + self.x_34 * rhs.x_43,
                x_34: self.x_31 * rhs.x_14 + self.x_32 * rhs.x_24 + self.x_33 * rhs.x_34 + self.x_34 * rhs.x_44,
                x_41: self.x_41 * rhs.x_11 + self.x_42 * rhs.x_21 + self.x_43 * rhs.x_31 + self.x_44 * rhs.x_41,
                x_42: self.x_41 * rhs.x_12 + self.x_42 * rhs.x_22 + self.x_43 * rhs.x_32 + self.x_44 * rhs.x_42,
                x_43: self.x_41 * rhs.x_13 + self.x_42 * rhs.x_23 + self.x_43 * rhs.x_33 + self.x_44 * rhs.x_43,
                x_44: self.x_41 * rhs.x_14 + self.x_42 * rhs.x_24 + self.x_43 * rhs.x_34 + self.x_44 * rhs.x_44,
            }
        }
    }

    // Mat4x4-Vec4 multiplication
    impl std::ops::Mul<Vec4> for Mat4x4 {
        type Output = Vec4;

        fn mul(self, rhs: Vec4) -> Vec4 {
            Vec4 {
                x_1: self.x_11 * rhs.x_1 + self.x_12 * rhs.x_2 + self.x_13 * rhs.x_3 + self.x_14 * rhs.x_4,
                x_2: self.x_21 * rhs.x_1 + self.x_22 * rhs.x_2 + self.x_23 * rhs.x_3 + self.x_24 * rhs.x_4,
                x_3: self.x_31 * rhs.x_1 + self.x_32 * rhs.x_2 + self.x_33 * rhs.x_3 + self.x_34 * rhs.x_4,
                x_4: self.x_41 * rhs.x_1 + self.x_42 * rhs.x_2 + self.x_43 * rhs.x_3 + self.x_44 * rhs.x_4,
            }
        }
    }

    // Mat4x4 negation
    impl std::ops::Neg for Mat4x4 {
        type Output = Self;

        fn neg(self) -> Self {
            Self {
                x_11: -self.x_11,
                x_12: -self.x_12,
                x_13: -self.x_13,
                x_14: -self.x_14,
                x_21: -self.x_21,
                x_22: -self.x_22,
                x_23: -self.x_23,
                x_24: -self.x_24,
                x_31: -self.x_31,
                x_32: -self.x_32,
                x_33: -self.x_33,
                x_34: -self.x_34,
                x_41: -self.x_41,
                x_42: -self.x_42,
                x_43: -self.x_43,
                x_44: -self.x_44,
            }
        }
    }

    impl std::fmt::Display for Mat4x4 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}   {}   {}   {}\n{}   {}   {}   {}\n{}   {}   {}   {}\n{}   {}   {}   {}\n",
                self.x_11,
                self.x_12,
                self.x_13,
                self.x_14,
                self.x_21,
                self.x_22,
                self.x_23,
                self.x_24,
                self.x_31,
                self.x_32,
                self.x_33,
                self.x_34,
                self.x_41,
                self.x_42,
                self.x_43,
                self.x_44,
            )
        }
    }

    pub const UNIT_MAT4X4: Mat4x4 = Mat4x4 {
        x_11: 1.0, x_12: 0.0, x_13: 0.0, x_14: 0.0,
        x_21: 0.0, x_22: 1.0, x_23: 0.0, x_24: 0.0,
        x_31: 0.0, x_32: 0.0, x_33: 1.0, x_34: 0.0,
        x_41: 0.0, x_42: 0.0, x_43: 0.0, x_44: 1.0,
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Quaternion {
        pub r: f32,
        pub i: f32,
        pub j: f32,
        pub k: f32,
    }

    impl std::ops::Mul<Self> for Quaternion {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            Self {
                r: self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
                i: self.r * rhs.i + self.i * rhs.r - self.j * rhs.k + self.k * rhs.j,
                j: self.r * rhs.j + self.i * rhs.k + self.j * rhs.r - self.k * rhs.i,
                k: self.r * rhs.k - self.i * rhs.j + self.j * rhs.i + self.k * rhs.r,

            }
        }
    } 

    impl Quaternion {
        pub fn conjugate(self) -> Self {
            Self {
                r: self.r,
                i: -self.i,
                j: -self.j,
                k: -self.k,
            }
        }

        pub fn rotate(point: Vec3, axis: Vec3, angle: f32) -> Vec3 {
            let rotation = axis.normalize() * (0.5 * angle).sin();

            let rotation = Quaternion {
                r: (0.5 * angle).cos(),
                i: rotation.x_1,
                j: rotation.x_3,
                k: rotation.x_2,
            };

            let point = Quaternion {
                r: 0.0,
                i: point.x_1,
                j: point.x_2,
                k: point.x_3,
            };

            let result = rotation * point * rotation.conjugate();

            Vec3 {
                x_1: result.i,
                x_2: result.j,
                x_3: result.k,
            }
        }

        pub fn rotate_offset(point: Vec3, axis: Vec3, offset: Vec3, angle: f32) -> Vec3 {
            Self::rotate(point - offset, axis, angle) + offset
        }

        // Multiplication for right-handed quaternion
        pub fn right_hand_mul(lhs: Quaternion, rhs: Quaternion) -> Self {
            Self {
                r: lhs.r * rhs.r - lhs.i * rhs.i - lhs.j * rhs.j - lhs.k * rhs.k,
                i: lhs.r * rhs.i + lhs.i * rhs.r + lhs.j * rhs.k - lhs.k * rhs.j,
                j: lhs.r * rhs.j - lhs.i * rhs.k + lhs.j * rhs.r + lhs.k * rhs.i,
                k: lhs.r * rhs.k + lhs.i * rhs.j - lhs.j * rhs.i + lhs.k * rhs.r,
            }
        }
    }
}
