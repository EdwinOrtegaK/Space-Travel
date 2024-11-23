use crate::fragment::{Fragment, fragment_shader};
use crate::Uniforms;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }            
        }
    }

    /*
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }
    */

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn process_fragments(&mut self, fragments: &[Fragment], uniforms: &Uniforms) {
        for fragment in fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;
            if x < self.width && y < self.height {
                let depth = fragment.position.z;

                // Llamada al fragment shader para calcular el color ajustado
                let shaded_color = fragment_shader(&fragment, uniforms, "static_pattern");
                let color = shaded_color.to_hex();
                
                // Configura el color actual del framebuffer y dibuja el punto
                self.set_current_color(color);
                self.point(x, y, depth);
            }
        }
    }
}