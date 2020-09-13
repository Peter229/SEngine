use gl;
use image::GenericImageView;
use crate::shader;

pub struct Quad {
    min: [f32; 2],
    max: [f32; 2],
    num_indices: i32,
    _posvbo: gl::types::GLuint,
    _uvvbo: gl::types::GLuint,
    _indvbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    texture_id: gl::types::GLuint,
}

impl Quad {

    pub fn new(min: [f32; 2], max: [f32; 2], path: String) -> Quad {

        let mut vertices_position: Vec<[f32; 2]> = Vec::new();
        let mut vertices_tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        vertices_position.push([min[0], min[1]]);
        vertices_position.push([min[0], max[1]]);
        vertices_position.push([max[0], max[1]]);
        vertices_position.push([max[0], min[1]]);
        vertices_tex_coords.push([0.0, 1.0]);
        vertices_tex_coords.push([0.0, 0.0]);
        vertices_tex_coords.push([1.0, 0.0]);
        vertices_tex_coords.push([1.0, 1.0]);
        indices.push(0);
        indices.push(2);
        indices.push(1);
        indices.push(0);
        indices.push(3);
        indices.push(2);

        let mut posvbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut posvbo);
        }
        let mut uvvbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut uvvbo);
        }
        let mut indvbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut indvbo);
        }

        let mut vao: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, posvbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices_position.len() * ::std::mem::size_of::<[f32; 2]>()) as gl::types::GLsizeiptr, vertices_position.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, uvvbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices_tex_coords.len() * ::std::mem::size_of::<[f32; 2]>()) as gl::types::GLsizeiptr, vertices_tex_coords.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indvbo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * ::std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, indices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let img = image::open(path).unwrap();
        let (dx, dy) = img.dimensions();

        let mut texture_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, dx as i32, dy as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.into_rgba().into_raw().as_ptr() as *const std::ffi::c_void);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        Quad { min, max, num_indices: indices.len() as i32, _posvbo: posvbo, _uvvbo: uvvbo, _indvbo: indvbo, vao, texture_id }
    }
    
    pub fn render(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.num_indices, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn clean_up(&mut self) {

        unsafe {

            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self._posvbo);
            gl::DeleteBuffers(1, &mut self._uvvbo);
            gl::DeleteBuffers(1, &mut self._indvbo);

            gl::DeleteTextures(1, &mut self.texture_id);
        }
    }
}

impl Drop for Quad {

    fn drop(&mut self) {

        unsafe {

            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self._posvbo);
            gl::DeleteBuffers(1, &mut self._uvvbo);
            gl::DeleteBuffers(1, &mut self._indvbo);

            gl::DeleteTextures(1, &mut self.texture_id);
        }
    }
}