use nalgebra_glm::Vec3;

pub struct Planet {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub orbit_radius: f32,
    pub orbit_angle: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl Planet {
    pub fn new(orbit_radius: f32, scale: f32, orbit_speed: f32, rotation_speed: f32, initial_angle: f32) -> Self {
        let center_x = 400.0;
        let center_y = 300.0;
        
        let translation = Vec3::new(
            center_x + orbit_radius * initial_angle.cos(),
            center_y + orbit_radius * initial_angle.sin(),
            0.0
        );
        
        Planet {
            translation,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale,
            orbit_speed,
            rotation_speed,
            orbit_radius,
            orbit_angle: initial_angle,
            center_x,
            center_y,
        }
    }
    
    pub fn update(&mut self) {
        // Actualizar ángulo de órbita
        self.orbit_angle += self.orbit_speed;
        
        // Actualizar posición basada en órbita circular perfecta
        self.translation.x = self.center_x + self.orbit_radius * self.orbit_angle.cos();
        self.translation.y = self.center_y + self.orbit_radius * self.orbit_angle.sin();
        
        // Actualizar rotación del planeta
        self.rotation.y += self.rotation_speed;
    }
}