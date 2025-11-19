use nalgebra_glm::Vec3;
use crate::camera::Camera;

pub struct Spaceship {
    pub offset: Vec3,  // Offset relativo a la cámara
    pub rotation: Vec3,
    pub scale: f32,
}

impl Spaceship {
    pub fn new() -> Self {
        Spaceship {
            // Posición relativa a la cámara (adelante y a la derecha)
            offset: Vec3::new(40.0, -30.0, -100.0),  // x: derecha, y: abajo, z: adelante
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 10.0,  // Escala inicial de la nave
        }
    }
    
    // Calcular la posición mundial de la nave basada en la cámara
    pub fn get_world_position(&self, camera: &Camera) -> Vec3 {
        // Vector desde la cámara hacia donde mira
        let forward = (camera.center - camera.eye).normalize();
        
        // Vector "derecha" (perpendicular a forward y up)
        let right = forward.cross(&camera.up).normalize();
        
        // Vector "arriba" real (perpendicular a forward y right)
        let up = right.cross(&forward).normalize();
        
        // Aplicar el offset en el sistema de coordenadas de la cámara
        camera.eye 
            + right * self.offset.x 
            + up * self.offset.y 
            + forward * (-self.offset.z)  // Negativo porque queremos adelante
    }
    
    // Calcular la rotación de la nave para que apunte en la dirección de la cámara
    pub fn get_world_rotation(&self, camera: &Camera) -> Vec3 {
        let forward = (camera.center - camera.eye).normalize();
        
        // Calcular yaw (rotación en Y)
        let yaw = forward.z.atan2(forward.x);
        
        // Calcular pitch (rotación en X)
        let pitch = (-forward.y).asin();
        
        Vec3::new(
            pitch + self.rotation.x,
            yaw + self.rotation.y + std::f32::consts::PI / 2.0,  // +90° para orientar correctamente
            self.rotation.z
        )
    }
    
    // Ajustar el offset (útil para controles)
    pub fn adjust_offset(&mut self, delta_x: f32, delta_y: f32, delta_z: f32) {
        self.offset.x += delta_x;
        self.offset.y += delta_y;
        self.offset.z += delta_z;
        
        // Debug para ver dónde está la nave
        println!("Nave offset: x={:.1}, y={:.1}, z={:.1}", 
                 self.offset.x, self.offset.y, self.offset.z);
    }
}