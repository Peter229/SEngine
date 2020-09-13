use gl;
use image::GenericImageView;
use crate::shader;
use std::ffi;

pub struct Text_Model {

    num_indices: i32,
    _posvbo: gl::types::GLuint,
    _uvvbo: gl::types::GLuint,
    _indvbo: gl::types::GLuint, 
    vao: gl::types::GLuint,
    texture_id: gl::types::GLuint,
}

impl Text_Model {

    pub fn new(text: &Vec<u32>, pos_x: f32, pos_y: f32, tile_size: u32) -> Text_Model {

        let mut vertices_position: Vec<[f32; 2]> = Vec::new();
        let mut vertices_tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let min: [f32; 2] = [0.0, 0.0];
        let render_width = 10.0;
        let max: [f32; 2] = [tile_size as f32, tile_size as f32];

        let mut index: u32 = 0;

        let image_size: u32 = 128;

        for x in 0..text.len() {

            let mut pos: u32 = 0;

            //Convert sdl to sprite sheet location
            if text[x] > 47 && text[x] <= 59 {

                pos = text[x] - 48;
            }
            else if text[x]  == 46 {

                pos = 11;
            }

            let u: f32 = ((pos % (image_size / tile_size)) as f32) / ((image_size / tile_size) as f32);
            let uu = u + ((tile_size as f32) / (image_size as f32));

            let v: f32 = (((pos as f32) / ((image_size as f32) / (tile_size as f32))).floor()) * ((tile_size as f32) / (image_size as f32));
            let vv = v + ((tile_size as f32) / (image_size as f32));

            vertices_position.push([min[0] + (pos_x + ((x as f32) * render_width)), min[1] + (pos_y as f32)]);
            vertices_position.push([min[0] + (pos_x + ((x as f32) * render_width)), max[1] + (pos_y as f32)]);
            vertices_position.push([max[0] + (pos_x + ((x as f32) * render_width)), max[1] + (pos_y as f32)]);
            vertices_position.push([max[0] + (pos_x + ((x as f32) * render_width)), min[1] + (pos_y as f32)]);
            vertices_tex_coords.push([u, vv]);
            vertices_tex_coords.push([u, v]);
            vertices_tex_coords.push([uu, v]);
            vertices_tex_coords.push([uu, vv]);
            indices.push(index + 0);
            indices.push(index + 2);
            indices.push(index + 1);
            indices.push(index + 0);
            indices.push(index + 3);
            indices.push(index + 2);

            index = index + 4;
        }

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

        let img = image::open("assets/textures/font.png".to_string()).unwrap();
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

        Text_Model { num_indices: indices.len() as i32, _posvbo: posvbo, _uvvbo: uvvbo, _indvbo: indvbo, vao, texture_id }
    }

    pub fn update(&mut self, text: &Vec<u32>, pos_x: f32, pos_y: f32, tile_size: u32) {

        unsafe {
    
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self._posvbo);
            gl::DeleteBuffers(1, &mut self._uvvbo);
            gl::DeleteBuffers(1, &mut self._indvbo);
        }

        let mut vertices_position: Vec<[f32; 2]> = Vec::new();
        let mut vertices_tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let min: [f32; 2] = [0.0, 0.0];
        let render_width = 10.0;
        let max: [f32; 2] = [tile_size as f32, tile_size as f32];

        let mut index: u32 = 0;

        let image_size: u32 = 128;

        for x in 0..text.len() {

            let mut pos: u32 = 0;

            //Convert sdl to sprite sheet location
            if text[x] > 47 && text[x] < 59 {

                pos = text[x] - 48;
            }
            else if text[x] == 46 {

                pos = 11;
            }
            else if text[x] == 59 {
                
                pos = 10;
            }

            let u: f32 = ((pos % (image_size / tile_size)) as f32) / ((image_size / tile_size) as f32);
            let uu = u + ((tile_size as f32) / (image_size as f32));

            let v: f32 = (((pos as f32) / ((image_size as f32) / (tile_size as f32))).floor()) * ((tile_size as f32) / (image_size as f32));
            let vv = v + ((tile_size as f32) / (image_size as f32));

            vertices_position.push([min[0] + (pos_x + ((x as f32) * render_width)), min[1] + (pos_y as f32)]);
            vertices_position.push([min[0] + (pos_x + ((x as f32) * render_width)), max[1] + (pos_y as f32)]);
            vertices_position.push([max[0] + (pos_x + ((x as f32) * render_width)), max[1] + (pos_y as f32)]);
            vertices_position.push([max[0] + (pos_x + ((x as f32) * render_width)), min[1] + (pos_y as f32)]);
            vertices_tex_coords.push([u, vv]);
            vertices_tex_coords.push([u, v]);
            vertices_tex_coords.push([uu, v]);
            vertices_tex_coords.push([uu, vv]);
            indices.push(index + 0);
            indices.push(index + 2);
            indices.push(index + 1);
            indices.push(index + 0);
            indices.push(index + 3);
            indices.push(index + 2);

            index = index + 4;
        }

        self.num_indices = indices.len() as i32;

        unsafe {
            gl::GenBuffers(1, &mut self._posvbo);
            gl::GenBuffers(1, &mut self._uvvbo);
            gl::GenBuffers(1, &mut self._indvbo);

            gl::GenVertexArrays(1, &mut self.vao);
            
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self._posvbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices_position.len() * ::std::mem::size_of::<[f32; 2]>()) as gl::types::GLsizeiptr, vertices_position.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, self._uvvbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices_tex_coords.len() * ::std::mem::size_of::<[f32; 2]>()) as gl::types::GLsizeiptr, vertices_tex_coords.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self._indvbo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * ::std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, indices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW,);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&self, shader: &shader::Program) {

        shader.set_used();
        let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));

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

impl Drop for Text_Model {

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

pub struct Text {

    pos_x: f32,
    pos_y: f32,
    tile_size: u32,
    text: Vec<u32>,
    text_model: Text_Model,
}

impl Text {

    pub fn new() -> Text {

        let pos_x = 45.0;
        let pos_y = 115.0;
        let tile_size = 16;
        let text: Vec<u32> = Vec::new();
        let mut text_model = Text_Model::new(&text, pos_x, pos_y, tile_size);

        Text { pos_x, pos_y, tile_size, text, text_model }
    }

    pub fn render(&self, shader: &shader::Program) {

        self.text_model.render(shader);
    }

    pub fn get_text_as_string(&self) -> String {

        let mut text = String::new();

        for i in 0..self.text.len() {

            if self.text[i] == 59 {
                text.push(':');
            }
            else {
                text.push((self.text[i] as u8) as char);
            }
        }

        text
    }

    pub fn edit(&mut self, key: u32) {

        if (key > 47 && key <= 59) || key == 46 {
            self.text.push(key);
            self.text_model.update(&self.text, self.pos_x, self.pos_y, self.tile_size);
        }else if key == 8 {
            self.text.pop();
            self.text_model.update(&self.text, self.pos_x, self.pos_y, self.tile_size);
        }
    }
}