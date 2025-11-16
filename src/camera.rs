use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
    
    // Para movimiento en el plano eclíptico
    pub angle: f32,
    pub radius: f32,
    pub height: f32,  // altura sobre el plano eclíptico
    
    // Para movimiento 3D completo
    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        // Calcular ángulo y radio inicial
        let dx = eye.x - center.x;
        let dy = eye.y - center.y;
        let radius = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);
        
        Camera {
            eye,
            center,
            up,
            has_changed: true,
            angle,
            radius,
            height: eye.z,
            pitch: 0.0,
            yaw: angle,
        }
    }
    
    // Movimiento orbital en el plano eclíptico
    pub fn orbit(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
        self.eye.x = self.center.x + self.radius * self.angle.cos();
        self.eye.y = self.center.y + self.radius * self.angle.sin();
        self.has_changed = true;
    }
    
    // Zoom in/out
    pub fn zoom(&mut self, delta: f32) {
        self.radius = (self.radius + delta).max(50.0).min(800.0);
        self.eye.x = self.center.x + self.radius * self.angle.cos();
        self.eye.y = self.center.y + self.radius * self.angle.sin();
        self.has_changed = true;
    }
    
    // Mover altura sobre el plano (para movimiento 3D)
    pub fn change_height(&mut self, delta: f32) {
        self.height += delta;
        self.eye.z = self.height;
        self.has_changed = true;
    }
    
    // Movimiento 3D completo usando pitch y yaw
    pub fn rotate_3d(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        
        // Calcular nueva posición de la cámara
        self.eye.x = self.center.x + self.radius * self.yaw.cos() * self.pitch.cos();
        self.eye.y = self.center.y + self.radius * self.yaw.sin() * self.pitch.cos();
        self.eye.z = self.center.z + self.radius * self.pitch.sin();
        
        self.has_changed = true;
    }
    
    // Instant warp a una posición específica
    pub fn warp_to(&mut self, target: Vec3, distance: f32) {
        self.center = target;
        // Mantener el ángulo actual pero ajustar posición
        self.radius = distance;
        self.eye.x = self.center.x + self.radius * self.angle.cos();
        self.eye.y = self.center.y + self.radius * self.angle.sin();
        self.eye.z = self.center.z + self.height;
        self.has_changed = true;
    }
    
    // Warp animado (interpolar hacia objetivo)
    pub fn animated_warp_to(&mut self, target: Vec3, distance: f32, speed: f32) -> bool {
        let target_eye = Vec3::new(
            target.x + distance * self.angle.cos(),
            target.y + distance * self.angle.sin(),
            target.z + self.height,
        );
        
        // Interpolación lineal
        let diff_center = target - self.center;
        let diff_eye = target_eye - self.eye;
        
        if diff_center.magnitude() < 1.0 && diff_eye.magnitude() < 1.0 {
            self.center = target;
            self.eye = target_eye;
            self.radius = distance;
            return true; // Warp completado
        }
        
        self.center += diff_center * speed;
        self.eye += diff_eye * speed;
        self.has_changed = true;
        false // Warp en progreso
    }
    
    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }
    
    pub fn get_projection_matrix(&self, window_width: f32, window_height: f32) -> Mat4 {
        perspective(
            window_width / window_height,
            45.0 * PI / 180.0,
            0.1,
            1000.0
        )
    }
}



