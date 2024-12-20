#![allow(non_snake_case)]
#![allow(temporary_cstring_as_ptr)]

pub mod shader;
pub mod texture;

use gl33::{*, global_loader::*};

use beryllium::Sdl;
use beryllium::init::InitFlags;
use beryllium::video::{CreateWinArgs, GlContextFlags, GlProfile, GlWindow};

use std::fs;

pub fn load_shader_file(file: &str) -> String {
    fs::read_to_string(file).expect("Bad shader file")
}

pub fn load_gl(win: &GlWindow){
    unsafe {
        load_global_gl(&|context| win.get_proc_address(context) as *const _);
        glViewport_load_with(&|context| win.get_proc_address(context) as *const _);
    }
}


pub fn create_context() -> Sdl {
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    let mut flags = GlContextFlags::default();
  
    if cfg!(debug_assertions) {
      flags |= GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();    
    sdl
}

pub fn create_window(sdl: &Sdl, name: &str, width: i32, height: i32) -> GlWindow {

    let win = sdl
    .create_gl_window(CreateWinArgs {
      title: name,
      width: width,
      height: height,
      resizable: true,
      allow_high_dpi: true,
      ..Default::default()
    })
    .expect("couldn't make a window and context");
    // win.set_swap_interval(beryllium::video::GlSwapInterval::Immediate).expect("Can't set interval");
    win
}

pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32){
    unsafe { glClearColor(r, g, b, a);}
}

pub fn clear(){
    unsafe{
    glEnable(GL_DEPTH_TEST);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }
}

pub fn unpack_enum(gl_enum: GLenum) -> i32{
    gl_enum.0 as i32
}



pub struct VertexArray(pub u32);
impl VertexArray {
    ///Create new VAO
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe { glGenVertexArrays(1, &mut vao)};
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    ///Bind this VAO to GL State
    pub fn bind(&self){
        glBindVertexArray(self.0);
    }

    ///Clear VAO binding to GL
    pub fn clear_bind() {
        glBindVertexArray(0);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe{
        glDeleteVertexArrays(1,&self.0);
        }
    }
}

/// The types of buffer object that you can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BufferType {
    /// Array Buffers holds arrays of vertex data for drawing.
    Array = GL_ARRAY_BUFFER.0,
    /// Element Array Buffers hold indexes of what vertexes to use for drawing.
    ElementArray = GL_ELEMENT_ARRAY_BUFFER.0,
}

pub struct BufferObject(pub u32);
impl BufferObject {
    ///Makes new VBO
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe { glGenBuffers(1, &mut vbo)};
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    ///Bind this buffer object
    pub fn bind(&self, ty: BufferType) {
        unsafe{ glBindBuffer(GLenum(ty as u32), self.0);};
    }

    ///Clear binding for buffer object of type
    pub fn clear_binding(ty: BufferType) {
        unsafe { glBindBuffer(GLenum(ty as u32), 0);}
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe{
        glDeleteBuffers(1,&self.0);
        }
    }
}


///Places slice of data into previous bound buffer
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum){
    unsafe{ glBufferData(   GLenum(ty as u32), 
                            data.len().try_into().unwrap(), 
                            data.as_ptr().cast(), usage);};
}


/// The polygon display modes you can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PolygonMode {
  /// Just show the points.
  Point = GL_POINT.0,
  /// Just show the lines.
  Line = GL_LINE.0,
  /// Fill in the polygons.
  Fill = GL_FILL.0,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
  unsafe { glPolygonMode(GL_FRONT_AND_BACK, GLenum(mode as u32)) };
}



