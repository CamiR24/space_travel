use crate::color::Color; 
use crate::fragment::Fragment; 
use crate::vertex::Vertex; 
use nalgebra_glm::Vec3; // Función de ruido simple (Perlin-like simplificado) 
fn noise(x: f32, y: f32) -> f32 { 
    let xi = x.floor() as i32; 
    let yi = y.floor() as i32; 
    let hash = ((xi.wrapping_mul(374761393) + yi.wrapping_mul(668265263)) ^ (xi.wrapping_mul(668265263))) as f32; 
    (hash.sin() * 43758.5453).fract() 
} 

fn fbm(x: f32, y: f32, octaves: i32) -> f32 { 
    let mut value = 0.0; let mut amplitude = 1.0; 
    let mut frequency = 1.0; 
    for _ in 0..octaves { 
        value += amplitude * noise(x * frequency, y * frequency); 
        frequency *= 2.0; amplitude *= 0.5; } 
    value 
} 

// Shader para planeta gaseoso (como Júpiter)         
pub fn gaseous_shader(fragment: &Fragment, base_color: Color, time: f32) -> Color { 
    let x = fragment.position.x; 
    let y = fragment.position.y; 
    // Crear bandas horizontales con movimiento 
    let bands = ((y * 0.03 + time * 0.5).sin() * 0.5 + 0.5); 
    // Agregar turbulencia 
    let turbulence = fbm(x * 0.02 + time, y * 0.02, 3); 
    // Combinar efectos 
    let pattern = (bands + turbulence * 0.3).clamp(0.0, 1.0); 
    // Variación de color 
    let color_variation = 0.7 + pattern * 0.6; 

    Color::new( (base_color.r as f32 * color_variation) as u8, (base_color.g as f32 * color_variation) as u8, (base_color.b as f32 * color_variation) as u8, ) 
} 

// Shader para planeta rocoso (como Marte) 
pub fn rocky_shader(fragment: &Fragment, base_color: Color) -> Color { 
    let x = fragment.position.x; 
    let y = fragment.position.y; 
    // Crear textura rocosa con múltiples octavas de ruido 
    let rock_noise = fbm(x * 0.05, y * 0.05, 4); 
    // Crear cráteres (manchas oscuras) 
    let crater_noise = noise(x * 0.03, y * 0.03); 
    let craters = if crater_noise > 0.85 { 
        0.5 // Área de cráter más oscura 
    } else { 1.0 }; 
    // Combinar efectos 
    let pattern = (rock_noise * 0.4 + 0.6) * craters; 
    
    Color::new( (base_color.r as f32 * pattern) as u8, (base_color.g as f32 * pattern) as u8, (base_color.b as f32 * pattern) as u8, ) 
} 

// Shader para el sol (emisivo, sin iluminación) 
pub fn sun_shader(fragment: &Fragment, base_color: Color, time: f32) -> Color { 
    let x = fragment.position.x; 
    let y = fragment.position.y; 
    // Agregar "actividad solar" con ruido animado 
    let activity = fbm(x * 0.03 + time * 2.0, y * 0.03 + time * 1.5, 2); 
    let intensity = 0.9 + activity * 0.2; 
    
    Color::new( ((base_color.r as f32 * intensity).min(255.0)) as u8, ((base_color.g as f32 * intensity).min(255.0)) as u8, ((base_color.b as f32 * intensity).min(255.0)) as u8, ) 
}