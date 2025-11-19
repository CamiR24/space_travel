use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    
    // Calcular posición mundial (solo modelo, sin vista ni proyección)
    let world_position_4 = uniforms.model_matrix * position;
    let world_position = Vec3::new(
        world_position_4.x,
        world_position_4.y,
        world_position_4.z
    );
    
    // Model -> World -> View -> Clip (para renderizado)
    let clip_space = uniforms.projection_matrix 
        * uniforms.view_matrix 
        * uniforms.model_matrix 
        * position;

    // Perspective division: Clip -> NDC
    let w = clip_space.w;
    let ndc = Vec3::new(
        clip_space.x / w,
        clip_space.y / w,
        clip_space.z / w
    );
    
    // NDC -> Screen space
    let screen = uniforms.viewport_matrix * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);
    
    let transformed_position = Vec3::new(
        screen.x,
        screen.y,
        screen.z
    );

    // Transform normal (en espacio mundial)
    let model_mat3 = Mat3::new(
        uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
        uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
        uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
    );
    
    let transformed_normal = (model_mat3 * vertex.normal).normalize();

    Vertex {
        position: world_position,  // IMPORTANTE: Guardamos la posición MUNDIAL, no la original
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal,
    }
}