use nalgebra_glm::{Vec3, Vec4};
use crate::vertex::Vertex;
use crate::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position = Vec4::new(
      vertex.position.x,
      vertex.position.y,
      vertex.position.z,
      1.0,
  );
  let transformed = uniforms.model_matrix * position;

  // Transforma al espacio de vista
  let view_transformed = uniforms.view_matrix * transformed;

  // Transforma al espacio de clip
  let clip_transformed = uniforms.projection_matrix * view_transformed;
  let screen_position = uniforms.viewport_matrix * clip_transformed;

  // Divide por `w` para normalizar en el espacio NDC
  let ndc_position = Vec3::new(
      clip_transformed.x / clip_transformed.w,
      clip_transformed.y / clip_transformed.w,
      clip_transformed.z / clip_transformed.w,
  );

  // Devuelve el vértice transformado
  Vertex {
      position: vertex.position,               
      normal: vertex.normal,                   
      tex_coords: vertex.tex_coords,           
      color: vertex.color,                    
      transformed_position: ndc_position,      
      transformed_normal: vertex.normal,       
  }
}
