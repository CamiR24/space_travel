# space_travel

Un simulador interactivo del sistema solar renderizado desde cero en Rust, sin usar motores gráficos. Implementa un pipeline de renderizado 3D completo con shaders personalizados, iluminación dinámica y controles de cámara avanzados.

# Controles Interactivos
Movimiento de Cámara 2D

A / D o ← / →: Orbitar alrededor del sistema
W / S: Zoom in/out (50-800 unidades)
Q / E: Subir/bajar altura sobre el plano eclíptico

Movimiento de Cámara 3D

M: Activar/desactivar modo 3D
A / D o ← / →: Rotación horizontal (yaw)
↑ / ↓: Rotación vertical (pitch)

Sistema de Warp

1-5: Warp animado a cada planeta
0: Warp al Sol
Transición suave con interpolación

General

ESC: Salir de la aplicación

# Características Adicionales

Órbitas visibles: Anillos orbitales para cada planeta
Nave espacial: Modelo 3D que sigue a la cámara
Física orbital: Movimiento planetario realista con diferentes velocidades
60 FPS: Renderizado suave con control de framerate

# Instalación
Requisitos Previos

Rust 1.70 o superior
Cargo (incluido con Rust)

# Dependencias Principales
```bash nalgebra-glm = "0.18"  # Álgebra lineal y matemáticas 3D
minifb = "0.25"        # Ventana y manejo de eventos
tobj = "3.2"           # Cargador de modelos OBJ
```
# Clonar repo
```bash git clone https://github.com/tuusuario/solar-system-3d.git
cd solar-system-3d 
```
# Compilar y ejecutar
```bash cargo build --release
cargo run --release 
```
# Video
https://drive.google.com/file/d/1yYMsKPswcVO1NCrra6vKhQCrLR7p7Qcd/view?usp=sharing