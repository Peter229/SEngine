use crate::quad;
use crate::shader;
use crate::control_settings;
use crate::level;
use cgmath;
use std::ffi;
use std::collections::HashMap;

const tile_masks: [[u32; 16]; 5] = [[16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16], [2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9], [9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16], [16, 16, 15, 15, 14, 14, 13, 13, 12, 12, 11, 11, 10, 10, 9, 9], [9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2]];

//Ground Speed constants
const acceleration: f32 = 0.046875;
const deceleration: f32 = 0.5;
const friction: f32 = 0.046875;
const top_horizontal_speed: f32 = 6.0;
const slope_factors: f32 = 0.125;
const slope_roll_up: f32 = 0.078125;
const slope_roll_down: f32 = 0.3125;
const fall: f32 = 2.5;

//Jumping Constants
const air: f32 = 0.09375;
const jump_force: f32 = 6.5;
const gravity: f32 = 0.21875;

//real_angle = (256-hex_angle)*1.40625 angle conversion

pub struct Player {

    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    vel_ground: f32,
    slope: f32,
    angle: f32,
    width_radius: u32,
    height_radius: u32,
    quad: quad::Quad,
    right: bool,
    left: bool,
    jump: bool,
    duck: bool,
    debug_quad: quad::Quad,
}

impl Player {

    pub fn new() -> Player {

        let quad = quad::Quad::new([0.0, 0.0], [38.0, 38.0], "assets/textures/sonic.png".to_string());
        let debug_quad = quad::Quad::new([0.0, 0.0], [39.0, 39.0], "assets/textures/bounds.png".to_string());

        let mut right: bool = false;
        let mut left: bool = false;
        let mut jump: bool = false;
        let mut duck: bool = false;

        Player { pos_x: 50.0, pos_y: 60.0, vel_x: 0.0, vel_y: 0.0, vel_ground: 0.0, slope: 0.0, angle: 0.0, width_radius: 8, height_radius: 19, quad, right, left, jump, duck, debug_quad }
    }

    pub fn inputs(&mut self, keys: &Vec<HashMap<usize, bool>>) {
        
        for i in 0..keys.len() {
            for (key, val) in keys[i].iter() {
                //On Press
                if *val {
                    unsafe {
                        if *key == control_settings::RIGHT {
                            self.right = true;
                        }
                        else if *key == control_settings::LEFT {
                            self.left = true;
                        }
                        else if *key == control_settings::JUMP {
                            self.jump = true;
                        }
                        else if *key == control_settings::DUCK {
                            self.duck = true;
                        }
                    }
                }
                else { 
                //On Release
                    unsafe {
                        if *key == control_settings::RIGHT {
                            self.right = false;
                        }
                        else if *key == control_settings::LEFT {
                            self.left = false;
                        }
                        else if *key == control_settings::JUMP {
                            self.jump = false;
                        }
                        else if *key == control_settings::DUCK {
                            self.duck = false;
                        }
                    }
                }
            }
        }
    }

    pub fn online_state_buffer(&self) -> [u8; 16] {

        let mut state_buffer: [u8; 16] = [0; 16];

        if self.right {
            state_buffer[control_settings::Player_State::RIGHT as usize] = 1;
        }
        if self.left {
            state_buffer[control_settings::Player_State::LEFT as usize] = 1;
        }
        if self.jump {
            state_buffer[control_settings::Player_State::JUMP as usize] = 1;
        }
        if self.duck {
            state_buffer[control_settings::Player_State::DUCK as usize] = 1;
        }

        state_buffer
    }

    pub fn update(&mut self) {

        if self.right && !self.left {
            self.vel_ground = 0.5;
        }else if self.left && !self.right {
            self.vel_ground = -0.5;
        }else if !self.left && !self.right {
            self.vel_ground = 0.0;
        }

        if self.jump && !self.duck {
            self.vel_y = 0.5;
        }else if self.duck && !self.jump  {
            self.vel_y = -0.5;
        }else if !self.jump && !self.duck {
            self.vel_y = 0.0;
        }

        self.vel_x = self.vel_ground * self.angle.cos();
        self.vel_y = self.vel_ground * -(self.angle.sin());

        self.pos_x += self.vel_x;        
        self.pos_y += self.vel_y;
    }

    pub fn floor_sensor(&mut self, x: usize, y: usize, inner_x: i32, tiles: &[u32; 4096]) -> f32 {
        
        let mut final_y = self.pos_y;

        if x >= 0 && x < 64 && y >= 0 && y < 64 {

            //If the tile is solid
            if tiles[64 * x + y] > 0 {

                let tile_height = tile_masks[(tiles[64 * x + y] - 1) as usize][inner_x as usize];

                if tile_height < 16 && tile_height > 0 {

                    final_y = ((y * 16) as f32) + tile_height as f32;
                    final_y += self.height_radius as f32;
                } //If at the top of the tile check the one above
                else if tile_height >= 16 {
                    //If the tile on top is solid set the height to it
                    if tiles[64 * x + (y + 1)] > 0 {

                        let tile_height_temp = tile_masks[(tiles[64 * x + (y + 1)] - 1) as usize][inner_x as usize];

                        if tile_height_temp < 16 {

                            final_y = (((y + 1) * 16) as f32) + tile_height_temp as f32;
                            final_y += self.height_radius as f32;
                        }
                    } //Else use the previous height
                    else {

                        final_y = ((y * 16) as f32) + tile_height as f32;
                        final_y += self.height_radius as f32;
                    }
                } //If at the bottom of the tile
                else {

                    if tiles[64 * x + (y - 1)] > 0 {

                        let tile_height_temp = tile_masks[(tiles[64 * x + (y - 1)] - 1) as usize][inner_x as usize];

                        if tile_height_temp < 16 {

                            final_y = (((y - 1) * 16) as f32) + tile_height_temp as f32;
                            final_y += self.height_radius as f32;
                        }
                    }
                    else {
                        
                        final_y = ((y * 16) as f32) + tile_height as f32;
                        final_y += self.height_radius as f32;
                    }
                }
            } //If on a non solid tile but the tile below is solid
            else {

                if tiles[64 * x + (y - 1)] > 0 {

                    let tile_height_temp = tile_masks[(tiles[64 * x + (y - 1)] - 1) as usize][inner_x as usize];

                    if tile_height_temp > 0{

                        final_y = (((y - 1) * 16) as f32) + tile_height_temp as f32;
                        final_y += self.height_radius as f32;
                    }
                }
            }
        }

        final_y
    }

    pub fn collision_detection(&mut self, tiles: &[u32; 4096]) {

        //Check floor
        if self.vel_y <= 0.0 {
            let left = (((self.pos_x.floor() as u32 - self.width_radius) as usize) & 0xfff0) / 16;
            let bottom = (((self.pos_y.floor() as u32 - self.height_radius) as usize) & 0xfff0) / 16;
            let mut inner_pos = (self.pos_x.floor() - self.width_radius as f32) as i32 % 16;

            let y1 = self.floor_sensor(left, bottom, inner_pos, tiles);

            let right = (((self.pos_x.floor() as u32 + self.width_radius) as usize) & 0xfff0) / 16;
            inner_pos = (self.pos_x.floor() + self.width_radius as f32) as i32 % 16;

            let y2 = self.floor_sensor(right, bottom, inner_pos, tiles);

            self.pos_y = y1.max(y2);
        }




    }

    pub fn render(&self, shader: &shader::Program) {

        shader.set_used();
        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        //Floor pos to keep pixel perfect
        let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.pos_x.floor() - self.height_radius as f32, self.pos_y.floor() - self.height_radius as f32, 0.0));
        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        
        self.debug_quad.render();
        //self.quad.render();
    }
}