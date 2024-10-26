use nalgebra_glm::{dot, mat4_to_mat3, Mat3, Vec2, Vec3, Vec4};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // fragment.color * fragment.intensity
    combined_shader(fragment, uniforms)
    // combined_blend_shader(fragment, "normal")
    // combined_blend_shader(fragment, "multiply")
    // combined_blend_shader(fragment, "add")
    // combined_blend_shader(fragment, "subtract")
}

fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();
  
    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;
  
    Color::new(r, g, b)
}

fn purple_shader(_fragment: &Fragment) -> Color {
    Color::new(128, 0, 128) // Purple color
}

fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let distance = (x * x + y * y).sqrt();
  
    if distance < 0.25 { // Circle radius
      Color::new(255, 255, 0) // Yellow circle
    } else {
      Color::new(0, 0, 0) // Black (transparent) background
    }
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;
  
    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();
  
    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };
  
    let circle_intensity = (circle1 + circle2).min(1.0f32);
  
    Color::new(
      (circle_intensity * 255.0) as u8,
      (circle_intensity * 255.0) as u8,
      (circle_intensity * 255.0) as u8
    )
}

pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, uniforms);
  
    // Combine shaders: use circle color if it's not black, otherwise use base color
    if !circle_color.is_black() {
      circle_color * fragment.intensity
    } else {
      base_color * fragment.intensity
    }
}

pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
  

    let base_color = purple_shader(fragment);
    let circle_color = circle_shader(fragment);
  
    let combined_color = match blend_mode {
      "normal" => base_color.blend_normal(&circle_color),
      "multiply" => base_color.blend_multiply(&circle_color),
      "add" => base_color.blend_add(&circle_color),
      "subtract" => base_color.blend_subtract(&circle_color),
      _ => base_color
    };
  
    combined_color * fragment.intensity
}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let center = Vec2::new(0.0, 0.0);

  let scale_factor = 0.5; 


  let time = uniforms.time as f32 * 0.02; 

  let wave_x = (x + time).sin() * 0.1;
  let wave_y = (y + time).cos() * 0.1;
  let distance = ((x + wave_x - center.x).powi(2) + (y + wave_y - center.y).powi(2)).sqrt() * scale_factor;


  let color_yellow = Color::new(255, 204, 0); 
  let color_orange = Color::new(255, 140, 0); 


  let gradient_color = color_yellow.lerp(&color_orange, distance.clamp(0.0, 1.0));

  let noise_intensity = ((x * 10.0 + time).sin() * (y * 10.0 + time).cos()).abs();
  let noise_color = Color::new(
      (gradient_color.get_r() as f32 * noise_intensity) as u8,
      (gradient_color.get_g() as f32 * noise_intensity) as u8,
      (gradient_color.get_b() as f32 * noise_intensity) as u8,
  );


  let emission_intensity = 1.5; 
  noise_color * emission_intensity
}

pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let color_dark_gray = Color::new(105, 105, 105); 
  let color_dark_brown = Color::new(80, 65, 55);   


  let pattern = ((x * 10.0).sin() * (y * 10.0).cos()).abs();
  let base_color = Color::new(
      (color_dark_brown.get_r() as f32 * pattern) as u8,
      (color_dark_brown.get_g() as f32 * pattern) as u8,
      (color_dark_brown.get_b() as f32 * pattern) as u8,
  );


  let noise_intensity = (5.0 * (x * 5.0).sin() * (y * 5.0).cos()).abs();
  let noise_color = Color::new(
      (base_color.get_r() as f32 * noise_intensity) as u8,
      (base_color.get_g() as f32 * noise_intensity) as u8,
      (base_color.get_b() as f32 * noise_intensity) as u8,
  );

  color_dark_gray.blend_multiply(&noise_color)
}






