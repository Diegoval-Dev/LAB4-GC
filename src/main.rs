use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader, sun_shader, rocky_planet_shader,venus_shader, earth_shader};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    // Transformación para rotación y escala
    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    // Crear matriz de rotación
    let rotation_x = Mat4::new_rotation(Vec3::new(rotation.x, 0.0, 0.0));
    let rotation_y = Mat4::new_rotation(Vec3::new(0.0, rotation.y, 0.0));
    let rotation_z = Mat4::new_rotation(Vec3::new(0.0, 0.0, rotation.z));
    let rotation_matrix = rotation_z * rotation_y * rotation_x;

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
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
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = sun_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_rocky_planet(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
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

    // Rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = rocky_planet_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn render_venus(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
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

    // Rasterización y procesamiento de fragmentos
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Shader específico de Venus
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader específico para Venus
            let shaded_color = venus_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_earth(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Ensamblado de Triángulos
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

    // Rasterización y procesamiento de fragmentos
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Aplicar `earth_shader` en cada fragmento
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = earth_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
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
        "Sistema Solar Shader Lab - Sol, Mercurio y Venus",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000000);

    // Configuración del Sol
    let translation_sun = Vec3::new(0.0, 0.0, 0.0);
    let scale_sun = 1.5f32;
    let rotation_sun = Vec3::new(0.0, 0.0, 0.0);
    let obj_sun = Obj::load("assets/models/sun.obj").expect("Failed to load sun");
    let vertex_array_sun = obj_sun.get_vertex_array();

    // Configuración de Mercurio
    let translation_mercury = Vec3::new(4.0, 0.0, 0.0); // Mercurio cerca del sol
    let scale_mercury = 0.5f32;
    let rotation_mercury = Vec3::new(0.0, 0.0, 0.0);
    let obj_mercury = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
    let vertex_array_mercury = obj_mercury.get_vertex_array();

    // Configuración de Venus
    let translation_venus = Vec3::new(6.0, 0.0, 0.0); // Venus más lejos que Mercurio
    let scale_venus = 0.6f32;
    let rotation_venus = Vec3::new(0.0, 0.0, 0.0);
    let obj_venus = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
    let vertex_array_venus = obj_venus.get_vertex_array();

    // Configuración de la Tierra
    let translation_earth = Vec3::new(8.0, 0.0, 0.0); // Más lejos que Venus
    let scale_earth = 0.6f32;
    let rotation_earth = Vec3::new(0.0, 0.0, 0.0);
    let obj_earth = Obj::load("assets/models/planet.obj").expect("Failed to load planet");
    let vertex_array_earth = obj_earth.get_vertex_array();

    // Cámara inicial
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        // Procesar entrada de la cámara
        handle_input(&window, &mut camera);

        // Actualizar la matriz de vista de la cámara
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);

        framebuffer.clear();

        // Renderizar el Sol
        let model_matrix_sun = create_model_matrix(translation_sun, scale_sun, rotation_sun);
        let uniforms_sun = Uniforms { 
            model_matrix: model_matrix_sun, 
            view_matrix: view_matrix.clone(), 
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
            time
        };
        render(&mut framebuffer, &uniforms_sun, &vertex_array_sun);

        // Renderizar Mercurio con el shader de planeta rocoso
        let model_matrix_mercury = create_model_matrix(translation_mercury, scale_mercury, rotation_mercury);
        let uniforms_mercury = Uniforms {
            model_matrix: model_matrix_mercury,
            view_matrix: view_matrix.clone(),
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
            time,
        };
        render_rocky_planet(&mut framebuffer, &uniforms_mercury, &vertex_array_mercury);

        // Renderizar Venus con el shader atmosférico
        let model_matrix_venus = create_model_matrix(translation_venus, scale_venus, rotation_venus);
        let uniforms_venus = Uniforms {
            model_matrix: model_matrix_venus,
            view_matrix,
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
            time,
        };
        render_venus(&mut framebuffer, &uniforms_venus, &vertex_array_venus);

        // Dentro del bucle principal
    let model_matrix_earth = create_model_matrix(translation_earth, scale_earth, rotation_earth);
    let uniforms_earth = Uniforms {
        model_matrix: model_matrix_earth,
        view_matrix: view_matrix.clone(),
        projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32),
        viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32),
        time,
    };
    render_earth(&mut framebuffer, &uniforms_earth, &vertex_array_earth);


        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}


fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    //  camera orbit controls
    if window.is_key_down(Key::Left) {
      camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
      camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
      camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
      camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
      movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
      movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
      movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
      movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
      camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
      camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
      camera.zoom(-zoom_speed);
    }
}
