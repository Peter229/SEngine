use crate::mario;
use crate::level;
use crate::shader;
use crate::camera;
use crate::sound;
use std::collections::HashMap;
use cgmath;
use std::ffi;

pub struct Game {

    player: mario::Player,
    level: level::Level,
    camera: camera::Camera,
}

impl Game {

    pub fn new() -> Game {

        let mut player = mario::Player::new();
        let mut level = level::Level::new();
        let mut camera = camera::Camera::new();

        Game { player, level, camera }
    }

    pub fn start(&mut self) {

    }

    pub fn update(&mut self, keys: &Vec<HashMap<usize, bool>>, sound_manager: &sound::Sound_Manager) {

        self.player.inputs(keys);
        self.player.update(&self.level.get_tiles(), sound_manager);
        self.camera.pos_x = self.player.get_position_x() - (256.0 / 2.0);
        self.camera.pos_y = self.player.get_position_y() - (192.0 / 3.0);
        self.camera.update();
    }

    pub fn render(&self, quad_shader: &shader::Program, level_shader: &shader::Program, ortho: cgmath::Matrix4<f32>) {

        let eye = cgmath::Point3::new(self.camera.pos_x, self.camera.pos_y, 1.0);
        let dir = cgmath::Vector3::new(0.0, 0.0, -1.0);
        let up = cgmath::Vector3::new(0.0, 1.0, 0.0);
        let view = cgmath::Matrix4::look_at_dir(eye, dir, up);
        let view_projection = ortho * view;

        quad_shader.set_used();
        quad_shader.set_mat4_cg(&view_projection, ffi::CStr::from_bytes_with_nul(b"view_projection\0").expect("CStr::from_bytes_with_nul failed"));

        level_shader.set_used();
        level_shader.set_mat4_cg(&view_projection, ffi::CStr::from_bytes_with_nul(b"view_projection\0").expect("CStr::from_bytes_with_nul failed"));

        self.player.render(quad_shader);
        self.level.render(level_shader);
    }

    pub fn mouse_update(&mut self, mouse_x: i32, mouse_y: i32, left_mouse: bool, right_mouse: bool, current_tile: u32) {

        let tile_pos_x = (((mouse_x as usize) + (self.camera.pos_x as usize)) & 0xfff0) / 16;
        let tile_pos_y = (((mouse_y as usize) + (self.camera.pos_y as usize)) & 0xfff0) / 16;
        //Stop the squad from destroying the floor :(
        if tile_pos_x != 0 && tile_pos_x != 63 && tile_pos_y != 0 && tile_pos_y != 63 {
            self.level.mouse_edit(tile_pos_x, tile_pos_y, left_mouse, right_mouse, current_tile);
        }
    }
}