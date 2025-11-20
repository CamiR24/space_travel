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
mod camera;
mod spaceship;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use planet::Planet;
use color::Color;
use gaseous_shader::{gaseous_shader, rocky_shader, sun_shader};
use camera::Camera;
use spaceship::Spaceship;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlanetType {
    Sun,
    Gaseous,
    Rocky,
    Normal,
    Spaceship,
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

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    color: u32,
    planet_type: PlanetType,
    time: f32,
    sun_world_position: Vec3
) {
    let is_sun = planet_type == PlanetType::Sun;

    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let mut transformed = vertex_shader(vertex, uniforms);
        transformed.color = Color::from_hex(color);
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
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], &sun_world_position, is_sun));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let final_color = match planet_type {
                PlanetType::Sun => sun_shader(&fragment, fragment.color, time),
                PlanetType::Gaseous => gaseous_shader(&fragment, fragment.color, time),
                PlanetType::Rocky => rocky_shader(&fragment, fragment.color),
                PlanetType::Normal => fragment.color,
                PlanetType::Spaceship => fragment.color,
            };
            
            let color = final_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 1200;
    let window_height = 800;
    let framebuffer_width = 1200;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar 3D - Rust Graphics",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(100, 100);
    framebuffer.set_background_color(0x000011);

    // Cargar modelos
    let sphere_obj = Obj::load("assets/models/sphere.obj").expect("Failed to load sphere");
    let vertex_arrays = sphere_obj.get_vertex_array();
    
    // Cargar nave espacial
    let spaceship_obj = Obj::load("assets/models/spaceship.obj").expect("Failed to load spaceship");
    let spaceship_vertices = spaceship_obj.get_vertex_array();
    
    println!("Nave cargada con {} vértices", spaceship_vertices.len());

    // Crear cámara - mirando desde atrás hacia el origen
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 900.0),  // eye - MÁS LEJOS para ver todos los planetas
        Vec3::new(0.0, 0.0, 0.0),    // center - origen
        Vec3::new(0.0, 1.0, 0.0),    // up
    );
    
    // Crear sistema de nave espacial
    let mut spaceship = Spaceship::new();

    // Configuración del sistema solar - centrado en el origen
    let center = Vec3::new(0.0, 0.0, 0.0);
    
    // SOL
    let mut sun = Planet {
        translation: center,
        rotation: Vec3::new(0.0, 0.0, 0.0),
        scale: 80.0,
        orbit_speed: 0.0,
        rotation_speed: 0.0,
        orbit_radius: 0.0,
        orbit_angle: 0.0,
        center_x: center.x,
        center_y: center.y,
    };

    // PLANETAS (5 planetas para conseguir 50 puntos)
    let mut planets = vec![
        // Mercurio - Rocoso pequeño
        Planet::new(120.0, 15.0, 0.04, 0.03, 0.0),
        // Venus - Rocoso
        Planet::new(180.0, 25.0, 0.03, 0.025, PI / 4.0),
        // Tierra - Normal con agua
        Planet::new(250.0, 28.0, 0.025, 0.02, PI / 2.0),
        // Marte - Rocoso rojo
        Planet::new(320.0, 22.0, 0.02, 0.018, 3.0 * PI / 4.0),
        // Júpiter - Gaseoso grande
        Planet::new(450.0, 55.0, 0.01, 0.04, PI),
    ];

    // Configurar profundidad Z para cada planeta
    for (i, planet) in planets.iter_mut().enumerate() {
        planet.translation.z = (i as f32 - 2.0) * 50.0;
    }

    let planet_types = [
        PlanetType::Rocky,   // Mercurio
        PlanetType::Rocky,   // Venus
        PlanetType::Normal,  // Tierra
        PlanetType::Rocky,   // Marte
        PlanetType::Gaseous, // Júpiter
    ];

    let colors = [
        0x8C7853,  // Mercurio - gris marrón
        0xFFC649,  // Venus - amarillo
        0x4169E1,  // Tierra - azul
        0xCD5C5C,  // Marte - rojo
        0xDAA520,  // Júpiter - dorado/naranja
    ];

    let start_time = Instant::now();
    
    // Variables para warp animado
    let mut warp_target_index: Option<usize> = None;
    let mut warp_in_progress = false;
    
    // Modo 3D
    let mut mode_3d = false;

    println!("=== CONTROLES ===");
    println!("Movimiento 2D:");
    println!("  A/D o ←/→: Rotar cámara");
    println!("  W/S: Zoom in/out");
    println!("  Q/E: Subir/Bajar altura");
    println!("\nMovimiento 3D:");
    println!("  M: Activar/Desactivar modo 3D");
    println!("  A/D o ←/→: Rotar horizontalmente");
    println!("  ↑/↓: Rotar verticalmente");
    println!("\nNave Espacial:");
    println!("  I/K: Mover nave arriba/abajo");
    println!("  J/L: Mover nave izquierda/derecha");
    println!("  U/O: Mover nave cerca/lejos");
    println!("\nWarp:");
    println!("  1-5: Warp a planetas");
    println!("  0: Warp al sol");
    println!("\nESC: Salir");

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        let time = start_time.elapsed().as_secs_f32();

        // ===== CONTROLES DE CÁMARA =====
        
        // Movimiento orbital (2D en el plano eclíptico)
        if !mode_3d {
            if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
                camera.orbit(0.05);
            }
            if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
                camera.orbit(-0.05);
            }
        } else {
            // Movimiento 3D completo
            if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
                camera.rotate_3d(0.05, 0.0);
            }
            if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
                camera.rotate_3d(-0.05, 0.0);
            }
            if window.is_key_down(Key::Up) {
                camera.rotate_3d(0.0, 0.05);
            }
            if window.is_key_down(Key::Down) {
                camera.rotate_3d(0.0, -0.05);
            }
        }
        
        // Zoom
        if window.is_key_down(Key::W) {
            camera.zoom(-5.0);
        }
        if window.is_key_down(Key::S) {
            camera.zoom(5.0);
        }
        
        // Altura (para movimiento 3D)
        if window.is_key_down(Key::Q) {
            camera.change_height(5.0);
        }
        if window.is_key_down(Key::E) {
            camera.change_height(-5.0);
        }
        
        // Controles de la nave espacial
        if window.is_key_down(Key::I) {
            spaceship.adjust_offset(0.0, 1.0, 0.0);  // Arriba
        }
        if window.is_key_down(Key::K) {
            spaceship.adjust_offset(0.0, -1.0, 0.0);  // Abajo
        }
        if window.is_key_down(Key::J) {
            spaceship.adjust_offset(-1.0, 0.0, 0.0);  // Izquierda
        }
        if window.is_key_down(Key::L) {
            spaceship.adjust_offset(1.0, 0.0, 0.0);  // Derecha
        }
        if window.is_key_down(Key::U) {
            spaceship.adjust_offset(0.0, 0.0, -1.0);  // Más cerca
        }
        if window.is_key_down(Key::O) {
            spaceship.adjust_offset(0.0, 0.0, 1.0);  // Más lejos
        }
        
        // Toggle modo 3D
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            mode_3d = !mode_3d;
            println!("Modo 3D: {}", if mode_3d { "ACTIVADO" } else { "DESACTIVADO" });
        }
        
        // Instant warp a planetas (teclas 1-5)
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            warp_target_index = Some(0);
            warp_in_progress = true;
            println!("Warping a Mercurio...");
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            warp_target_index = Some(1);
            warp_in_progress = true;
            println!("Warping a Venus...");
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            warp_target_index = Some(2);
            warp_in_progress = true;
            println!("Warping a Tierra...");
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            warp_target_index = Some(3);
            warp_in_progress = true;
            println!("Warping a Marte...");
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            warp_target_index = Some(4);
            warp_in_progress = true;
            println!("Warping a Júpiter...");
        }
        // Warp al sol
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            camera.warp_to(sun.translation, 200.0);
            println!("Warping al Sol...");
        }
        
        // Procesar warp animado
        if warp_in_progress {
            if let Some(idx) = warp_target_index {
                if idx < planets.len() {
                    let target = planets[idx].translation;
                    let distance = 150.0;
                    let completed = camera.animated_warp_to(target, distance, 0.05);
                    if completed {
                        warp_in_progress = false;
                        warp_target_index = None;
                        println!("Warp completado!");
                    }
                }
            }
        }

        // ===== ACTUALIZAR PLANETAS =====
        for planet in &mut planets {
            planet.update();
        }
        sun.rotation.y += sun.rotation_speed;

        // ===== RENDERIZADO =====
        framebuffer.clear();

        // Crear matrices de transformación
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix(
            window_width as f32,
            window_height as f32
        );
        let viewport_matrix = create_viewport_matrix(
            framebuffer_width as f32,
            framebuffer_height as f32
        );

        // Renderizar el sol
        let sun_model_matrix = create_model_matrix(sun.translation, sun.scale, sun.rotation);
        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };
        
        render(
            &mut framebuffer,
            &sun_uniforms,
            &vertex_arrays,
            0xFFDD00,
            PlanetType::Sun,
            time,
            sun.translation
        );

        // Renderizar planetas
        for (i, planet) in planets.iter().enumerate() {
            let model_matrix = create_model_matrix(
                planet.translation,
                planet.scale,
                planet.rotation
            );
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
            };
            render(
                &mut framebuffer,
                &uniforms,
                &vertex_arrays,
                colors[i],
                planet_types[i],
                time,
                sun.translation
            );
        }
        
        // Renderizar nave espacial (al final para que se vea encima)
        let spaceship_position = spaceship.get_world_position(&camera);
        let spaceship_rotation = spaceship.get_world_rotation(&camera);
        let spaceship_model_matrix = create_model_matrix(
            spaceship_position,
            spaceship.scale,
            spaceship_rotation
        );
        let spaceship_uniforms = Uniforms {
            model_matrix: spaceship_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };
        render(
            &mut framebuffer,
            &spaceship_uniforms,
            &spaceship_vertices,
            0xCCCCCC,  // Gris metálico
            PlanetType::Spaceship,
            time,
            sun.translation
        );

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}