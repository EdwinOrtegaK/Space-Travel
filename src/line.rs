use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4};

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    let normal_vector = Vec3::new(0.0, 0.0, 1.0);
    let intensity_value = 1.0;

    loop {
        let z = start.z + (end.z - start.z) * (x0 - start.x as i32) as f32 / (end.x - start.x) as f32;
        
        fragments.push(Fragment::new(
            Vec3::new(x0 as f32, y0 as f32, 0.0),
            Color::new(255, 255, 255),
            z,
            normal_vector,
            intensity_value ,
            start
        ));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}

pub fn draw_circle(
    framebuffer: &mut Framebuffer,
    orbit_center: Vec3,
    orbit_radius: f32,
    orbit_color: Color,
    view_matrix: Mat4,
) {
    for angle in (0..360).step_by(1) {
        let radians = (angle as f32).to_radians();

        let local_x = orbit_center.x + orbit_radius * radians.cos();
        let local_y = orbit_center.y + orbit_radius * radians.sin();
        let local_z = orbit_center.z;

        let transformed_point = view_matrix * Vec4::new(local_x, local_y, local_z, 1.0);

        let screen_x = (transformed_point.x / transformed_point.w) as usize;
        let screen_y = (transformed_point.y / transformed_point.w) as usize;

        if screen_x < framebuffer.width && screen_y < framebuffer.height {
            framebuffer.set_current_color(orbit_color.to_hex());
            framebuffer.point(screen_x, screen_y, local_z);
        }
    }
}


