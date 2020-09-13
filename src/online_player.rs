use crate::quad;
use crate::shader;
use crate::control_settings;
use cgmath;
use std::ffi;
use std::collections::HashMap;

pub struct Player {

    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    quad: quad::Quad,
    right: bool,
    left: bool,
    jump: bool,
}

impl Player {

    pub fn new() -> Player {

        let quad = quad::Quad::new([0.0, 0.0], [38.0, 38.0], "assets/textures/sonic.png".to_string());

        let mut right: bool = false;
        let mut left: bool = false;
        let mut jump: bool = false;

        Player { pos_x: 1.0, pos_y: 16.0, vel_x: 0.0, vel_y: 0.0, quad, right, left, jump }
    }

    pub fn update(&mut self, keys: &[u8; 16]) {

        if keys[control_settings::Player_State::RIGHT as usize] == 1 && keys[control_settings::Player_State::LEFT as usize] == 0 {
            self.vel_x = 0.5;
        }else if keys[control_settings::Player_State::LEFT as usize] == 1 && keys[control_settings::Player_State::RIGHT as usize] == 0 {
            self.vel_x = -0.5;
        }else if keys[control_settings::Player_State::RIGHT as usize] == 0 && keys[control_settings::Player_State::LEFT as usize] == 0 {
            self.vel_x = 0.0;
        }

        if keys[control_settings::Player_State::JUMP as usize] == 1 {
            self.vel_y = 0.5;
        }else {
            self.vel_y = 0.0;
        }

        if keys[control_settings::Player_State::JUMP as usize] == 1 && keys[control_settings::Player_State::DUCK as usize] == 0 {
            self.vel_y = 0.5;
        }else if keys[control_settings::Player_State::DUCK as usize] == 1 && keys[control_settings::Player_State::JUMP as usize] == 0  {
            self.vel_y = -0.5;
        }else if keys[control_settings::Player_State::JUMP as usize] == 0 && keys[control_settings::Player_State::DUCK as usize] == 0 {
            self.vel_y = 0.0;
        }

        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
    }

    pub fn render(&self, shader: &shader::Program) {

        shader.set_used();
        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.pos_x, self.pos_y, 0.0));
        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        
        self.quad.render();
    }
}