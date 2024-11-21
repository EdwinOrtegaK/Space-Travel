use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod experimental_shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use fastnoise_lite::{FastNoiseLite, NoiseType, CellularDistanceFunction};
use crate::fragment::{fragment_shader, Fragment, ring_shader};
use crate::color::Color;
use crate::camera::Camera;
use crate::line::draw_circle;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise_open_simplex: FastNoiseLite,
    noise_cellular: FastNoiseLite, 
}

pub struct Moon {
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
}

struct Planet {
    pub name: &'static str,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub shader: &'static str,
}

fn create_uniforms() -> Uniforms {
    let mut noise_open_simplex = FastNoiseLite::with_seed(1337);
    noise_open_simplex.set_noise_type(Some(NoiseType::OpenSimplex2));
    
    let mut noise_cellular = FastNoiseLite::with_seed(1337);
    noise_cellular.set_noise_type(Some(NoiseType::Cellular));
    noise_cellular.set_cellular_distance_function(Some(CellularDistanceFunction::Manhattan));

    Uniforms {
        model_matrix: Mat4::identity(),
        view_matrix: Mat4::identity(),
        projection_matrix: Mat4::identity(),
        viewport_matrix: Mat4::identity(),
        time: 0,
        noise_open_simplex,
        noise_cellular,
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn define_planets() -> Vec<Planet> {
    vec![
        Planet {
            name: "ROCKY_PLANET",
            scale: 5.4,
            orbit_radius: 100.0,
            orbit_speed: 0.02,
            rotation_speed: 0.01,
            shader: "rocky_planet_shader",
        },
        Planet {
            name: "PLANET_COLORFUL",
            scale: 6.8,
            orbit_radius: 180.0,
            orbit_speed: 0.015,
            rotation_speed: 0.008,
            shader: "colorful",
        },
        Planet {
            name: "ROCKY_PLANET_WITH_MOON",
            scale: 6.0,
            orbit_radius: 260.0,
            orbit_speed: 0.01,
            rotation_speed: 0.006,
            shader: "rocky_planet_with_moon_shader",
        },
        Planet {
            name: "DARK_RED",
            scale: 7.4,
            orbit_radius: 340.0,
            orbit_speed: 0.005,
            rotation_speed: 0.004,
            shader: "dark_red",
        },
        Planet {
            name: "GAS_GIANT",
            scale: 12.0,
            orbit_radius: 440.0,
            orbit_speed: 0.002,
            rotation_speed: 0.0008,
            shader: "gas_giant_shader",
        },
        Planet {
            name: "PLANET_EXOTIC",
            scale: 8.0,
            orbit_radius: 530.0,
            orbit_speed: 0.003,
            rotation_speed: 0.002,
            shader: "exotic",
        },
        Planet {
            name: "GAS_GIANT_WITH_RINGS",
            scale: 9.4,
            orbit_radius: 650.0,
            orbit_speed: 0.001,
            rotation_speed: 0.0008,
            shader: "gas_giant_with_rings",
        }
    ]
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], shader_type: &str) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
    
}

fn create_open_simplex_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_cellular_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Planetary System",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let planets = define_planets();
    let mut camera = Camera::new();

    let planet_obj = Obj::load("assets/spheresmooth.obj").expect("Failed to load obj");
    let planet_vertex_array = planet_obj.get_vertex_array();

    let ring_obj = Obj::load("assets/rings.obj").expect("Failed to load rings.obj");
    let ring_vertex_array = ring_obj.get_vertex_array(); 

    let mut time = 0;

    let mut camera_translation = Vec3::new(0.0, 0.0, 0.0);
    let mut camera_rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut camera_scale = 1.0f32;

    let mut moon = Moon {
        position: Vec3::new(0.0, 0.0, 0.0),
        scale: 8.0,
        rotation: Vec3::new(0.0, 0.0, 0.0),
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_camera_input(&mut camera, &window);
        time += 1;

        let view_matrix = camera.view_matrix();

        framebuffer.clear();

        // Renderizar el Sol
        let sun_translation = Vec3::new(window_width as f32 / 2.0, window_height as f32 / 2.0, 0.0);
        let sun_scale = 40.0;
        let sun_rotation = Vec3::new(0.0, 0.0, time as f32 * 0.5);
        let sun_model_matrix = create_model_matrix(sun_translation, sun_scale, sun_rotation);

        let mut sun_uniforms = create_uniforms();
        sun_uniforms.model_matrix = sun_model_matrix;
        sun_uniforms.view_matrix = view_matrix;
        sun_uniforms.time = time;

        render(
            &mut framebuffer,
            &sun_uniforms,
            &planet_vertex_array,
            "solar_surface",
        );

        // Renderizar proyecto
        for planet in &planets {
            let angle = time as f32 * planet.orbit_speed;

            let planet_translation = Vec3::new(
                (window_width as f32 / 2.0) + planet.orbit_radius * angle.cos(),
                (window_height as f32 / 2.0) + planet.orbit_radius * angle.sin(),
                0.0,
            );

            let planet_rotation = Vec3::new(0.0, time as f32 * planet.rotation_speed, 0.0);
            let planet_model_matrix = create_model_matrix(
                planet_translation,
                planet.scale * 2.0,
                planet_rotation,
            );

            let mut planet_uniforms = create_uniforms();
            planet_uniforms.model_matrix = planet_model_matrix;
            planet_uniforms.view_matrix = view_matrix;
            planet_uniforms.time = time;

            // Renderizar órbita
            const ORBIT_COLOR: Color = Color::new(200, 200, 200);

            draw_circle(
                &mut framebuffer,
                Vec3::new(
                    (window_width as f32 / 2.0),
                    (window_height as f32 / 2.0),
                    0.0,
                ),
                planet.orbit_radius,
                ORBIT_COLOR,
                view_matrix,
            );

            // Renderizar planeta
            render(
                &mut framebuffer,
                &planet_uniforms,
                &planet_vertex_array,
                planet.shader,
            );

            // Si el planeta tiene anillos
            if planet.name == "GAS_GIANT_WITH_RINGS" {
                // Configurar los anillos
                let ring_scale = planet.scale * 2.5;
                let ring_model_matrix = create_model_matrix(
                    planet_translation,
                    ring_scale,
                    planet_rotation, 
                );
            
                let mut ring_uniforms = create_uniforms();
                ring_uniforms.model_matrix = ring_model_matrix;
                ring_uniforms.view_matrix = view_matrix;
                ring_uniforms.time = time;
            
                // Renderizar los anillos
                render(
                    &mut framebuffer,
                    &ring_uniforms,
                    &ring_vertex_array, 
                    "ring",      
                );
            }

            // Si el planeta tiene luna
            if planet.name == "ROCKY_PLANET_WITH_MOON" {
                let moon_orbit_radius = 30.0;
                let moon_scale = planet.scale * 0.8;
                let moon_orbit_speed = 0.01;
                let moon_angle = time as f32 * moon_orbit_speed;

                let moon_translation = Vec3::new(
                    planet_translation.x + moon_orbit_radius * moon_angle.cos(),
                    planet_translation.y + moon_orbit_radius * moon_angle.sin(),
                    0.0,
                );

                // Rotación de la luna
                let moon_rotation_speed = planet.rotation_speed * 0.3;
                let moon_rotation = Vec3::new(0.0, time as f32 * moon_rotation_speed, 0.0);

                let moon_model_matrix = create_model_matrix(
                    moon_translation,
                    moon_scale,
                    moon_rotation,
                );

                let mut moon_uniforms = create_uniforms();
                moon_uniforms.model_matrix = moon_model_matrix;
                moon_uniforms.view_matrix = view_matrix;
                moon_uniforms.time = time;

                render(
                    &mut framebuffer,
                    &moon_uniforms,
                    &planet_vertex_array,
                    "moon_shader",
                );
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn render_rings(framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
    let ring_inner_radius = 1.2;
    let ring_outer_radius = 1.8;

    for x in -400..400 {
        for y in -400..400 {
            let xf = x as f32 / 100.0;
            let yf = y as f32 / 100.0;
            let distance = (xf.powi(2) + yf.powi(2)).sqrt();

            if distance > ring_inner_radius && distance < ring_outer_radius {
                let fragment = Fragment::new(
                    Vec2::new(xf, yf),
                    Color::new(0, 0, 0),
                    1.0,
                    Vec3::new(0.0, 0.0, 1.0),
                    1.0,
                    Vec3::new(xf, yf, 0.0),
                );

                let ring_color = ring_shader(&fragment, uniforms);
                
                let x_screen = (xf * 100.0 + framebuffer.width as f32 / 2.0) as usize;
                let y_screen = (yf * 100.0 + framebuffer.height as f32 / 2.0) as usize;
                
                if x_screen < framebuffer.width && y_screen < framebuffer.height {
                    framebuffer.set_current_color(ring_color.to_hex());
                    framebuffer.point(x_screen, y_screen, 1.0);
                }
            }
        }
    }
}

fn handle_camera_input(camera: &mut Camera, window: &Window) {
    let move_speed = 5.0; 
    let rotation_speed = 0.02; 
    let zoom_speed = 0.02; 

    // Movimiento de cámara
    if window.is_key_down(Key::Left) {
        camera.translation.x -= move_speed; 
    }
    if window.is_key_down(Key::Right) {
        camera.translation.x += move_speed; 
    }
    if window.is_key_down(Key::Up) {
        camera.translation.y -= move_speed; 
    }
    if window.is_key_down(Key::Down) {
        camera.translation.y += move_speed; 
    }

    // Control de rotación 
    if window.is_key_down(Key::A) {
        camera.rotation.y += rotation_speed; 
    }
    if window.is_key_down(Key::D) {
        camera.rotation.y -= rotation_speed; 
    }
    if window.is_key_down(Key::W) {
        camera.rotation.x += rotation_speed; 
    }
    if window.is_key_down(Key::S) {
        camera.rotation.x -= rotation_speed; 
    }

    // Zoom
    if window.is_key_down(Key::Q) {
        camera.scale += zoom_speed;  
    }
    if window.is_key_down(Key::E) {
        camera.scale -= zoom_speed;  
        if camera.scale < 0.1 {
            camera.scale = 0.1;
        }
    }
}
