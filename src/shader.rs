use gl;
use crate::resources::{self,Resources};
use std;
use std::ffi::{CString, CStr};
use cgmath::Matrix4;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}

pub struct Program {

    id: gl::types::GLuint,
}

impl Program {

    pub fn from_res(res: &Resources, name: &str) -> Result<Program, Error> {

        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let resource_names = POSSIBLE_EXT.iter().map(|file_extension| format!("{}{}", name, file_extension)).collect::<Vec<String>>();

        let shaders = resource_names.iter().map(|resource_name| Shader::from_res(res, resource_name)).collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..]).map_err(|message| Error::LinkError { name: name.into(), message, })
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {

        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {

            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }
        
        let mut success: gl::types::GLint = 1;
        unsafe {

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {

            let mut len: gl::types::GLint = 0;

            unsafe {

                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {

                gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar,);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {

            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id})
    }

    pub fn id(&self) -> gl::types::GLuint {

        self.id
    }

    pub fn set_used(&self) {

        unsafe {

            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4_array_cg(&self, mats: &mut [Matrix4<f32>; 56], name: &CStr) {

        let y: *mut f32 = &mut mats[0].x.x;

        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 56_i32, gl::FALSE, y);
        }
    }

    pub fn set_mat4_cg(&self, mat: &Matrix4<f32>, name: &CStr) {

        let y = &mat.x.x;

        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, y);
        }
    }

    pub fn set_int(&self, num: i32, name: &CStr) {

        unsafe {

            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), num);
        }
    }

    pub fn set_vec2(&self, vec: &[f32; 2], name: &CStr) {

        unsafe {

            gl::Uniform2fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, vec.as_ptr());
        }
    }

    pub fn set_vec3(&self, vec: &[f32; 3], name: &CStr) {

        unsafe {

            gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, vec.as_ptr());
        }
    }
}

impl Drop for Program {

    fn drop(&mut self) {

        unsafe {

            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader{

    id: gl::types::GLuint,
}

impl Shader {
    
    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, Error> {

        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_kind = POSSIBLE_EXT.iter().find(|&&(file_extension, _)| name.ends_with(file_extension)).map(|&(_, kind)| kind).ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;
    
        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {name: name.into(), inner: e,})?;

        Shader::from_source(&source, shader_kind).map_err(|message| Error::CompileError { name: name.into(), message, })
    }

    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {

        let id = shader_from_source(source, kind)?;

        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {

        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {

        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {

        self.id
    }
}

impl Drop for Shader {

    fn drop(&mut self) {

        unsafe {

            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {

    let id = unsafe { gl::CreateShader(kind)};

    unsafe {

        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {

        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {

        let mut len: gl::types::GLint = 0;
        
        unsafe {

            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {

            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {

    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { CString::from_vec_unchecked(buffer) }
}