// Tree structure of Object:
//
// * position: [f32; 3]
// * triangles: Vec<Triangle>
//   - vertices: Vertex
//     * position: [f32; 3]
//     * color: [f32; 3]
//   - normal: [f32; 3]
// * collision: bool

pub mod object {
    mod gmlib;
    use gmlib::matrix::*;

    pub struct Vertex {
        position: Vec3,
        color: Vec4,
    }

    pub struct Triangle {
        vertices: [Vertex; 3],
        normal: Vec3,
    }

    pub struct Object {
        position: Vec3,
        triangles: Vec<Triangle>,
        collision: bool,
    }

    impl Object {
        pub fn rotate(&mut self, axis: Vec3, angle: f32, offset: Vec3) {
            for i in 0..self.triangles.len() {
                self.triangles[i].vertices[0].position =
                    Quaternion::rotate_offset(self.triangles[i].vertices[0].position, axis, angle, offset);
                self.triangles[i].vertices[1].position =
                    Quaternion::rotate_offset(self.triangles[i].vertices[1].position, axis, angle, offset);
                self.triangles[i].vertices[2].position =
                    Quaternion::rotate_offset(self.triangles[i].vertices[2].position, axis, angle, offset);
            }
        }
    }
}

