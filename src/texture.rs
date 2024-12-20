use gl33::{*, global_loader::*};
use image::ImageReader;
use super::unpack_enum as unpack_enum;

pub struct Texture(pub u32);
impl Texture {
    pub fn new() -> Option<Self> {
        let mut tex = 0;
        unsafe { glGenTextures(1, &mut tex)};
        if tex != 0 {
            Some(Self(tex))
        } else {
            None
        }
    }

    pub fn bind(&self, tex_unit:GLenum) {
        unsafe{
        glActiveTexture(tex_unit);
        glBindTexture(GL_TEXTURE_2D, self.0);
        }
    }

    pub fn setParams(&self) {
        unsafe{
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, unpack_enum(GL_REPEAT));
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, unpack_enum(GL_REPEAT));
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, unpack_enum(GL_LINEAR));
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, unpack_enum(GL_LINEAR));
        }
    }

    pub fn loadTexFile(&self, img_path: &str) {
        let img = ImageReader::open(img_path).expect("Bad Texture Image Path").decode().expect("Image corrupt?").flipv();
        let data_format = match img.color() {
            image::ColorType::Rgb8 => GL_RGB,
            image::ColorType::Rgba8 => GL_RGBA,
            _ => {panic!("Not supported Image format")},
        };
        unsafe{
        glTexImage2D(GL_TEXTURE_2D, 0, unpack_enum(data_format), 
                        img.width() as i32, img.height() as i32, 0, data_format,
                         GL_UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const _);
        }
    }

}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe{
        glDeleteTextures(1,&self.0);
        }
    }
}