use nalgebra_glm::{Vec3, Mat4};

pub struct Camera {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            translation: Vec3::new(0.0, 0.0, -500.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 1.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let translation_matrix = nalgebra_glm::translation(&self.translation);
        let scaling_matrix = nalgebra_glm::scaling(&Vec3::new(self.scale, self.scale, self.scale));
        let rotation_matrix_x = nalgebra_glm::rotation(self.rotation.x, &Vec3::x_axis());
        let rotation_matrix_y = nalgebra_glm::rotation(self.rotation.y, &Vec3::y_axis());
        let rotation_matrix_z = nalgebra_glm::rotation(self.rotation.z, &Vec3::z_axis());

        let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

        scaling_matrix * rotation_matrix * translation_matrix
    }
}
