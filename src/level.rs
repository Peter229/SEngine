use gl;
use image::GenericImageView;
use crate::shader;
use std::ffi;

pub struct Level_Model {

    num_indices: i32,
    _posvbo: gl::types::GLuint,
    _uvvbo: gl::types::GLuint,
    _indvbo: gl::types::GLuint, 
    vao: gl::types::GLuint,
    texture_id: gl::types::GLuint,
}

impl Level_Model {

    pub fn new(tiles: &[u32; 4096], width: usize, height: usize, tile_size: u32) -> Level_Model {

        let mut vertices_position: Vec<[f32; 2]> = Vec::new();
        let mut vertices_tex_coords: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let min: [f32; 2] = [0.0, 0.0];
        let max: [f32; 2] = [tile_size as f32, tile_size as f32];

        let mut index: u32 = 0;

        let image_size: u32 = 256;

        for x in 0..width {

            for y in 0..height {

                if tiles[width * x + y] != 0 {

                    let u: f32 = ((tiles[width * x + y] % (image_size / tile_size)) as f32) / ((image_size / tile_size) as f32);
                    let uu = u + ((tile_size as f32) / (image_size as f32));
                    
                    let v: f32 = ((tiles[width * x + y] as f32 / (image_size as f32 / tile_size as f32)).floor());
                    let vv = v + ((tile_size as f32) / (image_size as f32));

                    vertices_position.push([min[0] + ((x as f32) * (tile_size as f32)), min[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([min[0] + ((x as f32) * (tile_size as f32)), max[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([max[0] + ((x as f32) * (tile_size as f32)), max[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([max[0] + ((x as f32) * (tile_size as f32)), min[1] + ((y as f32) * (tile_size as f32))]);
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
            }
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

        let img = image::open("assets/textures/level.png".to_string()).unwrap();
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

        Level_Model { num_indices: indices.len() as i32, _posvbo: posvbo, _uvvbo: uvvbo, _indvbo: indvbo, vao, texture_id }
    }

    pub fn update(&mut self, tiles: &[u32; 4096], width: usize, height: usize, tile_size: u32) {

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
        let max: [f32; 2] = [tile_size as f32, tile_size as f32];

        let mut index: u32 = 0;

        let image_size: u32 = 256;

        for x in 0..width {

            for y in 0..height {

                if tiles[width * x + y] != 0 {

                    let u: f32 = ((tiles[width * x + y] % (image_size / tile_size)) as f32) / ((image_size / tile_size) as f32);
                    let uu = u + ((tile_size as f32) / (image_size as f32));
                    
                    let v: f32 = ((tiles[width * x + y] as f32 / (image_size as f32 / tile_size as f32)).floor());
                    let vv = v + ((tile_size as f32) / (image_size as f32));

                    vertices_position.push([min[0] + ((x as f32) * (tile_size as f32)), min[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([min[0] + ((x as f32) * (tile_size as f32)), max[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([max[0] + ((x as f32) * (tile_size as f32)), max[1] + ((y as f32) * (tile_size as f32))]);
                    vertices_position.push([max[0] + ((x as f32) * (tile_size as f32)), min[1] + ((y as f32) * (tile_size as f32))]);
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
            }
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

impl Drop for Level_Model {

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

pub struct Level {

    width: usize,
    height: usize,
    tile_size: u32,
    tiles: [u32; 4096],
    level_model: Level_Model,
}

impl Level {

    pub fn new() -> Level {

        let width = 64;
        let height = 64;
        let tile_size = 16;
        let mut tiles = [0; 4096];
        
        for i in 0..(width - 1) {
            tiles[width * i + 0] = 1;
            tiles[width * i + (width - 1)] = 1;
            tiles[width * 0 + i] = 1;
            tiles[width * (width - 1) + i] = 1;
        }

        let mut level_model = Level_Model::new(&tiles, width, height, tile_size);

        Level { width, height, tile_size, tiles, level_model }
    }

    pub fn get_tiles(&self) -> [u32; 4096] {

        self.tiles
    }

    pub fn render(&self, shader: &shader::Program) {

        self.level_model.render(shader);
    }

    pub fn mouse_edit(&mut self, mouse_x: usize, mouse_y: usize, left_mouse: bool, right_mouse: bool, current_tile: u32) {

        let tile_pos_x = mouse_x;
        let tile_pos_y = mouse_y;
        if self.tiles[self.width * tile_pos_x + tile_pos_y] > 0 {
            if right_mouse {
                self.tiles[self.width * tile_pos_x + tile_pos_y] = 0;
                self.level_model.update(&self.tiles, self.width, self.height, self.tile_size);
            }
        }else {
            if left_mouse {
                self.tiles[self.width * tile_pos_x + tile_pos_y] = current_tile;
                self.level_model.update(&self.tiles, self.width, self.height, self.tile_size);
            }
        }
    }
}