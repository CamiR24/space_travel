use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod planet;
mod gaseous_shader;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use planet::Planet;
use color::Color;
use gaseous_shader::{gaseous_shader, rocky_shader, sun_shader};

pub struct Uniforms {
    model_matrix: Mat4,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlanetType {
    Sun,
    Gaseous,
    Rocky,
    Normal,
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

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], color: u32, planet_type: PlanetType, time: f32, sun_position: Vec3) {
    // Vertex Shader Stage con color aplicado
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let mut transformed = vertex_shader(vertex, uniforms);
        transformed.color = Color::from_hex(color);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
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

    // Rasterization Stage
    let mut fragments = Vec::new();

    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], &sun_position));
    }

    // Fragment Processing Stage con shaders procedurales
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar shader según el tipo de planeta
            let final_color = match planet_type {
                PlanetType::Sun => sun_shader(&fragment, fragment.color, time),
                PlanetType::Gaseous => gaseous_shader(&fragment, fragment.color, time),
                PlanetType::Rocky => rocky_shader(&fragment, fragment.color),
                PlanetType::Normal => fragment.color,
            };
            
            let color = final_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Sistema Solar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let sun_screen_pos = Vec3::new(400.0, 300.0, -200.0);

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000011);

    // Cargar el modelo de la esfera una sola vez
    let obj = Obj::load("assets/models/sphere_smooth.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    // Crear 3 planetas con diferentes características
    let mut planets = vec![
        Planet::new(200.0, 30.0, 0.02, 0.05, 0.0),     // planeta rocoso — radio 200
        Planet::new(320.0, 60.0, 0.01, 0.03, PI / 3.0),// gaseoso — radio 320
        Planet::new(460.0, 45.0, 0.005, 0.04, 2.0 * PI / 3.0), // normal — radio 460
    ];

    for (i, planet) in planets.iter_mut().enumerate() {
        planet.translation.z = -200.0 - (i as f32) * 40.0; // por ejemplo: -200, -240, -280
    }

    let mut sun = Planet {
        translation: Vec3::new(400.0, 300.0, -200.0),
        rotation: Vec3::new(0.0, 0.0, 0.0),
        scale: 90.0,
        orbit_speed: 0.0,
        rotation_speed: 0.0,
        orbit_radius: 0.0,
        orbit_angle: 0.0,
        center_x: 400.0,
        center_y: 300.0,
    };
    

    let start_time = Instant::now();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        let time = start_time.elapsed().as_secs_f32();

        framebuffer.clear();

        // Renderizar el sol (emisivo, sin recibir luz)
        //sun.rotation.y += sun.rotation_speed;
        //let sun_matrix = create_model_matrix(sun.translation, sun.scale, sun.rotation);
        //let sun_uniforms = Uniforms { model_matrix: sun_matrix };
        //render(&mut framebuffer, &sun_uniforms, &vertex_arrays, 0xFFDD00, PlanetType::Sun, time);

        sun.rotation.y += sun.rotation_speed;
        let sun_matrix = create_model_matrix(sun.translation, sun.scale, sun.rotation);
        let sun_uniforms = Uniforms { model_matrix: sun_matrix };
        render(&mut framebuffer, &sun_uniforms, &vertex_arrays, 0xFFDD00, PlanetType::Sun, time, sun_screen_pos);

        // Renderizar cada planeta con su shader específico
        let planet_types = [PlanetType::Rocky, PlanetType::Gaseous, PlanetType::Normal];
        let colors = [
            0xCD5C5C,  // Rojo oscuro para rocoso (tipo Marte)
            0xFFA500,  // Naranja para gaseoso (tipo Júpiter)
            0x4169E1,  // Azul para normal (tipo Tierra)
        ];

        for (i, planet) in planets.iter_mut().enumerate() {
            planet.update();
            let model_matrix = create_model_matrix(planet.translation, planet.scale, planet.rotation);
            let uniforms = Uniforms { model_matrix };
            render(&mut framebuffer, &uniforms, &vertex_arrays, colors[i], planet_types[i], time, sun_screen_pos);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}