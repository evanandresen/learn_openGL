use std::ffi::CString;
use gl33::{*, global_loader::*};
use nalgebra_glm as glm;

use super::unpack_enum as unpack_enum;

/// The types of shader object.
#[repr(u32)]
pub enum ShaderType {
    /// Vertex shaders determine the position of geometry within the screen.
    Vertex = GL_VERTEX_SHADER.0,
    /// Fragment shaders determine the color output of geometry.
    ///
    /// Also other values, but mostly color.
    Fragment = GL_FRAGMENT_SHADER.0,
  }

/// A handle to a [Shader
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects)
pub struct Shader(pub u32);
impl Shader {
    /// Makes a new shader.
    ///
    /// Prefer the [`Shader::from_source`](Shader::from_source) method.
    ///
    /// Possibly skip the direct creation of the shader object and use
    /// [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag).
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader =  glCreateShader(GLenum(ty as u32));
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    /// Assigns a source string to the shader
    /// 
    /// Replaces any previously assigned shaders
    pub fn set_source(&self, src: String) {
        unsafe{
            glShaderSource(self.0, 1, 
                                &(src.as_bytes().as_ptr().cast()), 
                                &(src.len().try_into().unwrap()))
        };
    }

    /// Compiles the shader based on the current source
    pub fn compile(&self) {
        glCompileShader(self.0);
    }

    /// Checks if the last compile was successful or not.
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == unpack_enum(GL_TRUE)
    }

    /// Gets the info log for the shader.
    ///
    /// Usually you use this to get the compilation log when a compile failed.
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetShaderInfoLog(
            self.0,
            v.capacity().try_into().unwrap(),
            &mut len_written,
            v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Marks a shader for deletion.
    ///
    /// Note: This _does not_ immediately delete the shader. It only marks it for
    /// deletion. If the shader has been previously attached to a program then the
    /// shader will stay allocated until it's unattached from that program.
    pub fn mark_delete(self) {
        glDeleteShader(self.0);
    }

    /// Takes a shader type and source string and produces either the compiled
    /// shader or an error message.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn from_source(ty: ShaderType, source: String) -> Result<Self, String> {
        let id = Self::new(ty)
                .ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.mark_delete();
            Err(out)
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        glDeleteShader(self.0);
    }
}

/// A handle to a [Program
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects)
pub struct ShaderProgram(pub u32);
impl ShaderProgram {
    /// Allocates a new program object.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn new() -> Option<Self> {
        let id = glCreateProgram();
        if id != 0 {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Attaches a shader object to this program object.
    pub fn attach_shader(&self, shader: &Shader) {
        glAttachShader(self.0, shader.0);
    }

    /// Links the various attached, compiled shader objects into a usable program.
    pub fn link_program(&self) {
        glLinkProgram(self.0);
    }

    /// Checks if the last linking operation was successful.
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
        success == unpack_enum(GL_TRUE)
    }

    /// Gets the log data for this program.
    ///
    /// This is usually used to check the message when a program failed to link.
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetProgramInfoLog(
            self.0,
            v.capacity().try_into().unwrap(),
            &mut len_written,
            v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Sets the program as the program to use when drawing.
    pub fn use_program(&self) {
        glUseProgram(self.0);
    }

    /// Marks the program for deletion.
    ///
    /// Note: This _does not_ immediately delete the program. If the program is
    /// currently in use it won't be deleted until it's not the active program.
    /// When a program is finally deleted and attached shaders are unattached.
    pub fn delete(self) {
        glDeleteProgram(self.0);
    }

    /// Takes a vertex shader source string and a fragment shader source string
    /// and either gets you a working program object or gets you an error message.
    ///
    /// This is the preferred way to create a simple shader program in the common
    /// case. It's just less error prone than doing all the steps yourself.
    pub fn from_vert_frag(vert: String, frag: String) -> Result<Self, String> {
        let p =
            Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.mark_delete();
        f.mark_delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }


    ///Create functions for uniforms
    pub fn setBool(&self, uniform_name: &str, vals: Vec<bool>) -> bool {
        unsafe {
        let id = glGetUniformLocation(self.0, CString::new(uniform_name).expect("").as_ptr() as *const u8);
        assert_ne!(id, -1, "Uniform not found: {}", uniform_name);
    
        Self::uniformi_helper(id, vals.iter().map(|&x|x as i32).collect())
        }
    }

    pub fn setInt(&self, uniform_name: &str, vals: Vec<i32>) -> bool {
        unsafe {
        let id = glGetUniformLocation(self.0, CString::new(uniform_name).expect("").as_ptr() as *const u8);
        assert_ne!(id, -1, "Uniform not found: {}", uniform_name);
       
        Self::uniformi_helper(id, vals)
        }
    }


    pub fn setFloat(&self, uniform_name: &str, vals: Vec<f32>) -> bool {
        unsafe {
        let id = glGetUniformLocation(self.0, CString::new(uniform_name).expect("").as_ptr() as *const u8);
        assert_ne!(id, -1, "Uniform not found: {}", uniform_name);
   
        let mut iter = vals.clone().into_iter();
        match vals.len() {
            1 => {glUniform1f(id, iter.next().expect("Bad input")); true},
            2 => {glUniform2f(id, iter.next().expect("Bad input"),
                                             iter.next().expect("Bad input"),); true},
            3 => {glUniform3f(id, iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),); true},
            4 => {glUniform4f(id,iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),); true},
            _ => false
        }
        }
    }

    pub fn uniformi_helper(id: i32, vals: Vec<i32>) -> bool {
        unsafe{
        let mut iter = vals.clone().into_iter();
        match vals.len() {
            1 => {glUniform1i(id, iter.next().expect("Bad input")); true},
            2 => {glUniform2i(id, iter.next().expect("Bad input"),
                                             iter.next().expect("Bad input"),); true},
            3 => {glUniform3i(id, iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),); true},
            4 => {glUniform4i(id,iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),
                                            iter.next().expect("Bad input"),); true},
            _ => false
        }
        }
    }

    pub fn setMat4(&self, uniform_name: &str, vec_matrix: Vec<glm::Mat4>)  { //add bool
        let vals: Vec<f32> = vec_matrix.iter().flatten().copied().collect();
        unsafe{
        let id = glGetUniformLocation(self.0, CString::new(uniform_name).expect("").as_ptr() as *const u8);
        assert_ne!(id, -1, "Uniform not found: {}", uniform_name);
        glUniformMatrix4fv(id, vec_matrix.len() as i32, unpack_enum(GL_FALSE) as u8, vals.as_ptr());
        }
    }
}

    
impl Drop for ShaderProgram {
    fn drop(&mut self) {
                glDeleteProgram(self.0);
    }
}
