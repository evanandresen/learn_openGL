#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(temporary_cstring_as_ptr)]


use gl33::{*, global_loader::*};

use beryllium::events::*;

use learn_openGL as learn;
use learn::*;
use learn::shader::ShaderProgram;
use learn::texture::Texture;

use std::collections::HashSet;
use std::time;

use nalgebra_glm as glm;



///TODO: 
/// hash movements
/// enable vsync
/// fix texture stutter
fn main() {

  let mut win_width = 1920;
  let mut win_height = 1080;

  let sdl = create_context();

  let win = create_window(&sdl, "Bev Window", win_width, win_height);

  let cube_pos: [glm::Vec3; 10] = 
        [glm::vec3( 0.0, 0.0, 0.0),
        glm::vec3( 2.0, 5.0,-15.0),
        glm::vec3(-1.5,-2.2,-2.5),
        glm::vec3(-3.8,-2.0,-12.3),
        glm::vec3( 2.4,-0.4,-3.5),
        glm::vec3(-1.7, 3.0,-7.5),
        glm::vec3( 1.3,-2.0,-2.5),
        glm::vec3( 1.5, 2.0,-2.5),
        glm::vec3( 1.5, 0.2,-1.5),
        glm::vec3(-1.3, 1.0,-1.5)];

  type Vertex = [f32; 5];
  const VERTICES: [f32; 36 * 5] = [
    -0.5,-0.5,-0.5, 0.0, 0.0,
    0.5,-0.5,-0.5, 1.0, 0.0,
    0.5, 0.5,-0.5, 1.0, 1.0,
    0.5, 0.5,-0.5, 1.0, 1.0,
    -0.5, 0.5,-0.5, 0.0,1.0,
    -0.5,-0.5,-0.5, 0.0,0.0,

    -0.5,-0.5, 0.5, 0.0,0.0,
    0.5,-0.5, 0.5, 1.0,0.0,
    0.5, 0.5, 0.5, 1.0,1.0,
    0.5, 0.5, 0.5, 1.0,1.0,
    -0.5, 0.5, 0.5, 0.0,1.0,
    -0.5,-0.5, 0.5, 0.0,0.0,

    -0.5, 0.5, 0.5, 1.0,0.0,
    -0.5, 0.5,-0.5, 1.0,1.0,
    -0.5,-0.5,-0.5, 0.0,1.0,
    -0.5,-0.5,-0.5, 0.0,1.0,
    -0.5,-0.5, 0.5, 0.0,0.0,
    -0.5, 0.5, 0.5, 1.0,0.0,

    0.5, 0.5, 0.5, 1.0,0.0,
    0.5, 0.5,-0.5, 1.0,1.0,
    0.5,-0.5,-0.5, 0.0,1.0,
    0.5,-0.5,-0.5, 0.0,1.0,
    0.5,-0.5, 0.5, 0.0,0.0,
    0.5, 0.5, 0.5, 1.0,0.0,

    -0.5,-0.5,-0.5, 0.0,1.0,
    0.5,-0.5,-0.5, 1.0,1.0,
    0.5,-0.5, 0.5, 1.0,0.0,
    0.5,-0.5, 0.5, 1.0,0.0,
    -0.5,-0.5, 0.5, 0.0,0.0,
    -0.5,-0.5,-0.5, 0.0,1.0,

    -0.5, 0.5,-0.5, 0.0,1.0,
    0.5, 0.5,-0.5, 1.0,1.0,
    0.5, 0.5, 0.5, 1.0,0.0,
    0.5, 0.5, 0.5, 1.0,0.0,
    -0.5, 0.5, 0.5, 0.0,0.0,
    -0.5, 0.5,-0.5, 0.0, 1.0];


  let vert_shader = load_shader_file("shaders/vertex.vert");
  let frag_shader= load_shader_file("shaders/frag.frag");

  load_gl(&win);

  set_clear_color(0.0, 0.5, 0.5, 1.0);

  let vao = VertexArray::new().expect("Can't make new VAO");
  vao.bind();

  let vbo = BufferObject::new().expect("Can't make VBO");
  vbo.bind(BufferType::Array);
  buffer_data(BufferType::Array, bytemuck::cast_slice(&VERTICES), GL_STATIC_DRAW);


  let tex = Texture::new().expect("Can't make Texture Object");
  tex.bind(GL_TEXTURE0);
  tex.setParams();
  tex.loadTexFile("textures/brick.jpg");

  let tex2 = Texture::new().expect("Can't make Texture Object");
  tex2.bind(GL_TEXTURE1);
  tex2.setParams();
  tex2.loadTexFile("textures/face.png");


unsafe {

  glVertexAttribPointer(
    0,
    3,
    GL_FLOAT,
    GL_FALSE.0 as u8,
    size_of::<Vertex>().try_into().unwrap(),
    0 as *const _,
  );
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(
    1,
    2,
    GL_FLOAT,
    GL_FALSE.0 as u8,
    size_of::<Vertex>().try_into().unwrap(),
    (3 * size_of::<f32>()) as *const _,
  );
  glEnableVertexAttribArray(1);
}


  let shader_program = ShaderProgram::from_vert_frag(vert_shader, frag_shader).unwrap();
  shader_program.use_program();

  shader_program.setInt("brick", vec![0]);
  shader_program.setInt("face", vec![1]);




  let mut model = glm::rotate(&glm::Mat4::identity(), -55.0_f32.to_radians(), &glm::vec3(1.0,0.0,0.0));
  let mut view: glm::Mat4;
  let mut projection: glm::Mat4;

  let mut angle = 0.0;
  let mut input: f32 = 0.0;
  let mut prev_time = time::Instant::now();

  let mut cam_pos = glm::vec3(0.0,0.0,3.0);
  let mut cam_front = glm::vec3(0.0,0.0,-1.0);
  let cam_up = glm::vec3(0.0,1.0,0.0);
  
  let mut pressed_keys = HashSet::<beryllium::events::SDL_Keycode>::new();
  let mut yaw = -90.0;
  let mut pitch = 0.0;
  let mut fov: f32 = 45.0;
  'main_loop: loop {

    let delta_t = time::Instant::now().duration_since(prev_time).as_millis() as f32;
    prev_time = time::Instant::now();

    // let fps = 1.0 / (delta_t / 1000.0);
    // println!("{}", fps);

    let cam_speed = 0.01 * delta_t;

    sdl.set_relative_mouse_mode(true).expect("Can't capture mouse");

    // handle events this frame
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'main_loop,
        Event::WindowCloseRequest { .. } => break 'main_loop,
        Event::WindowSizeChanged {width, height, .. } => {
                  win_width = width;
                  win_height = height;
                  unsafe { glViewport(0, 0, width, height); }
        },
        // Event::WindowGainedKeyboardFocus { .. } => sdl.set_relative_mouse_mode(true).expect("Can't capture mouse"),
        // Event::WindowLostKeyboardFocus { .. } => sdl.set_relative_mouse_mode(false).expect("Can't release mouse"),
        Event::MouseMotion {x_delta, y_delta , ..} => {
          println!("x: {}, y: {}", x_delta, y_delta);
          cam_front = rotate_camera(&mut yaw, &mut pitch, x_delta, y_delta);
        },
        Event::MouseWheel {y, .. } => {
          fov -= y as f32;
          fov = fov.clamp(1.0, 45.0);
        },
        Event::Key{pressed, keycode, ..} if pressed => { pressed_keys.insert(keycode); },
        Event::Key{pressed, keycode, ..} if !pressed => { pressed_keys.remove(&keycode); },
        _ => (),
      }
    }
    //manage key presses
    let mut keys_iter = pressed_keys.iter();
    while let Some(keycode) = keys_iter.next().copied() {
      #[allow(non_upper_case_globals)]
      match keycode {
        SDLK_UP => input += 0.005 * delta_t,
        SDLK_DOWN => input -= 0.005 * delta_t,
        SDLK_w => cam_pos += cam_speed * cam_front,
        SDLK_s => cam_pos -= cam_speed * cam_front,
        SDLK_a => cam_pos -= cam_speed * glm::normalize(&glm::cross(&cam_front, &cam_up)),
        SDLK_d => cam_pos += cam_speed * glm::normalize(&glm::cross(&cam_front, &cam_up)),
        SDLK_SPACE => cam_pos += cam_speed * cam_up,
        SDLK_LSHIFT => cam_pos -= cam_speed * cam_up,
        _ => (),
      }
    };
    // now the events are clear.
    input = input.clamp(0.0, 1.0);

    angle += 0.0008 * delta_t as f32;
    view = glm::look_at(&cam_pos, &(cam_pos+cam_front), &cam_up);
    projection = glm::perspective(win_width as f32 / win_height as f32, fov.to_radians(), 0.1, 100.0);



    shader_program.setFloat("mix_lvl", vec![input]);
    shader_program.setMat4("model", vec![model]);
    shader_program.setMat4("view", vec![view]);
    shader_program.setMat4("projection", vec![projection]);


    clear();
    for i in 0..10{
      model = glm::Mat4::identity();
      model = glm::translate(&model, &cube_pos[i]);
      model = glm::rotate(&model, angle + i as f32, &glm::vec3(1.0,0.3,0.5));
      shader_program.setMat4("model", vec![model]);
      unsafe {
        glDrawArrays(GL_TRIANGLES, 0, 36);
      }
    }
    win.swap_window();
  }
}

fn rotate_camera(yaw: &mut f32, pitch: &mut f32, x_delta: i32, y_delta: i32) -> glm::Vec3 {
    let sensitivity = 0.1;
    let x_delta = x_delta as f32 * sensitivity;
    let y_delta = -y_delta as f32 * sensitivity;
    *yaw += x_delta;
    *pitch += y_delta;
    *pitch = pitch.clamp(-89.9, 89.9);
    let mut front = glm::vec3(0.0,0.0,0.0);
    front.x = yaw.to_radians().cos() * pitch.to_radians().cos();
    front.y = pitch.to_radians().sin();
    front.z = yaw.to_radians().sin() * pitch.to_radians().cos();
    glm::normalize(&front)
}

