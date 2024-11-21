use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::color::Color;
use crate::Uniforms;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::fragment::Fragment;

fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;

    Color::new(r, g, b)
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;

    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();

    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };

    let circle_intensity = (circle1 + circle2).min(1.0f32);

    Color::new(
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8
    )
}

pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, uniforms);

    // Usa el color del círculo si no es negro, sino usa el color base
    if !circle_color.is_black() {
        circle_color * fragment.intensity
    } else {
        base_color * fragment.intensity
    }
}

fn purple_shader(_fragment: &Fragment) -> Color {
    Color::new(128, 0, 128) // Color morado
}

fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let distance = (x * x + y * y).sqrt();

    if distance < 0.25 {
        Color::new(255, 255, 0) // Círculo amarillo
    } else {
        Color::new(0, 0, 0) // Fondo negro (transparente)
    }
}

pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color = purple_shader(fragment);
    let circle_color = circle_shader(fragment);

    let combined_color = match blend_mode {
        "normal" => base_color.blend_normal(&circle_color),
        "multiply" => base_color.blend_multiply(&circle_color),
        "add" => base_color.blend_add(&circle_color),
        "subtract" => base_color.blend_subtract(&circle_color),
        _ => base_color
    };

    combined_color * fragment.intensity
}

fn glow_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let glow_size = 0.05;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI / 2.0).sin();

    Color::new(0, (0.6 * glow_intensity * 255.0) as u8, (glow_intensity * 255.0) as u8)
}

fn core_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let core_size = 0.02;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let core_intensity = if distance_to_center < core_size { 1.0 } else { 0.0 };

    Color::new((0.8 * core_intensity * 255.0) as u8, (0.9 * core_intensity * 255.0) as u8, (core_intensity * 255.0) as u8)
}

fn background_shader(_fragment: &Fragment) -> Color {
    Color::new(10, 10, 20) // Fondo azul oscuro
}

pub fn neon_light_shader(fragment: &Fragment) -> Color {
    let background = background_shader(fragment);
    let glow = glow_shader(fragment);
    let core = core_shader(fragment);

    let blended_glow = background.blend_screen(&glow);
    blended_glow.blend_add(&core)
}

pub fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    Color::new(r, g, b) * fragment.intensity
}

pub fn panda_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 50.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let noise_value = uniforms.noise_open_simplex.get_noise_2d(x * zoom, y * zoom);
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255);
    let base_color = Color::new(0, 0, 0);

    (if noise_value < spot_threshold { spot_color } else { base_color }) * fragment.intensity
}

pub fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
    let noise_value = uniforms.noise_open_simplex.get_noise_2d(x * zoom + t, y * zoom);
    let cloud_threshold = 0.5;
    let cloud_color = Color::new(255, 255, 255);
    let sky_color = Color::new(30, 97, 145);

    (if noise_value > cloud_threshold { cloud_color } else { sky_color }) * fragment.intensity
}

pub fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let cell_noise_value = uniforms.noise_cellular.get_noise_2d(x * zoom, y * zoom).abs();
    let cell_color_1 = Color::new(85, 107, 47);
    let cell_color_2 = Color::new(124, 252, 0);
    let cell_color_3 = Color::new(34, 139, 34);
    let cell_color_4 = Color::new(173, 255, 47);

    (if cell_noise_value < 0.15 {
        cell_color_1
    } else if cell_noise_value < 0.7 {
        cell_color_2
    } else if cell_noise_value < 0.75 {
        cell_color_3
    } else {
        cell_color_4
    }) * fragment.intensity
}

pub fn get_experimental_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str) -> Color {
    match shader_type {
        "random_color" => random_color_shader(fragment, uniforms),
        "panda" => panda_shader(fragment, uniforms),
        "cloud" => cloud_shader(fragment, uniforms),
        "cellular" => cellular_shader(fragment, uniforms),
        _ => Color::new(0, 0, 0),
    }
}
