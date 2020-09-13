use crate::quad;
use crate::shader;
use crate::control_settings;
use crate::level;
use crate::sound;
use cgmath;
use std::ffi;
use std::collections::HashMap;

const tile_masks: [[u32; 16]; 5] = [[16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16], [2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9], [9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16], [16, 16, 15, 15, 14, 14, 13, 13, 12, 12, 11, 11, 10, 10, 9, 9], [9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2]];

const acceleration: f32 = 0.2;
const deceleration: f32 = 0.1;
const top_speed: f32 = 3.0;
const gravity: f32 = 0.3;
const friction: f32 = 0.05;
const small_speed: f32 = 0.15;
const step_up_size: f32 = 4.0;
const step_down_size: f32 = 5.0;

pub struct CollisionPacket {

    pub collision: bool,
    pub offset: f32,
}

pub struct Player {

    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    quad: quad::Quad,
    right: bool,
    left: bool,
    jump: bool,
    frame: f32,
    width: u32,
    height: u32,
    grounded: bool,
    facing: bool,
}

impl Player {

    pub fn new() -> Player {

        let quad = quad::Quad::new([0.0, 0.0], [48.0, 48.0], "assets/textures/mario_test.png".to_string());

        let mut right: bool = false;
        let mut left: bool = false;
        let mut jump: bool = false;

        Player { pos_x: 50.0, pos_y: 60.0, vel_x: 0.0, vel_y: 0.0, quad, right, left, jump, frame: 0.0, width: 16, height: 24, grounded: false, facing: true }
    }

    pub fn animation(&mut self) {

        if self.grounded {

            if self.right && !self.left && self.vel_x > 0.0 && self.pos_x - self.vel_x.floor() != self.pos_x {

                self.facing = true;
                self.frame += 0.2;
                if self.frame > 6.0 || self.frame < 3.0 {
                    self.frame = 3.0;
                }
            }
            else if self.left && !self.right && self.vel_x < 0.0 {

                self.facing = false;
                self.frame += 0.2;
                if self.frame > 6.0 || self.frame < 3.0 {
                    self.frame = 3.0;
                }
            }
            else if self.vel_x == 0.0 {

                self.frame = 0.0;

            }else if (self.left || self.right) && !(self.left && self.right) && self.pos_x - self.vel_x.floor() != self.pos_x {
                
                self.facing = self.right;
                self.frame = 13.0;
            }
        }
        else {

            if self.vel_y > 0.0 {

                self.frame = 19.0;
            }else {

                self.frame = 20.0;
            }
        }
    }

    pub fn set_key_state(&mut self, keys: &[u8; 16]) {

        if keys[control_settings::Player_State::RIGHT as usize] == 1 {
            self.right = true;
        }else if keys[control_settings::Player_State::RIGHT as usize] == 2 {
            self.right = false;
        } 
        if keys[control_settings::Player_State::LEFT as usize] == 1 {
            self.left = true;
        }else if keys[control_settings::Player_State::LEFT as usize] == 2 {
            self.left = false;
        } 
        if keys[control_settings::Player_State::JUMP as usize] == 1 {
            self.jump = true;
        }else if keys[control_settings::Player_State::JUMP as usize] == 2 {
            self.jump = false;
        } 
    }

    pub fn update(&mut self, tiles: &[u32; 4096], sound_manager: &sound::Sound_Manager) {

        if self.right && !self.left {
            self.vel_x += acceleration;
        }else if self.left && !self.right {
            self.vel_x -= acceleration;
        }else if !self.right && !self.left {
            self.vel_x -= self.vel_x.signum() * (friction + deceleration);
        }

        if self.vel_x.abs() > top_speed {
            self.vel_x = self.vel_x.signum() * top_speed;
        }
        else if self.vel_x.abs() <= small_speed {
            self.vel_x = 0.0;
        }

        //println!("{}", self.vel_x.abs());

        if self.jump && self.grounded {
            self.vel_y = 5.0;
            self.grounded = false;
            //sound_manager.play_sound("jump", 1);
        }

        self.vel_y -= gravity;

        self.pos_x += self.vel_x;
        
        self.horizontal_detection(tiles);

        self.pos_y += self.vel_y;

        self.vertical_detection(tiles);

        self.animation();
    }

    pub fn horizontal_detection(&mut self, tiles: &[u32; 4096]) {

        if self.vel_x > 0.0 {

            let right = (((self.pos_x.floor() as u32 + 7_u32) as usize) & 0xfff0) / 16;
            let top = (((self.pos_y.floor() as u32) as usize) & 0xfff0) / 16;
            let bottom = (((self.pos_y.floor() as u32 - 8_u32) as usize) & 0xfff0) / 16;

            if tiles[64 * right + bottom] > 0 {

                if self.grounded {
                    let try_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * right + bottom] - 1) as usize][((self.pos_x.floor() as u32 + 7) % 16) as usize] as f32 - 1.0;

                    if self.pos_y + step_up_size > try_y {
                        self.pos_y = try_y;
                    }
                    else {
                        self.pos_x = ((right * 16) - 8) as f32;
                        self.vel_x = 0.0;
                    }
                }
                else {
                    self.pos_x = ((right * 16) - 8) as f32;
                    self.vel_x = 0.0;
                }
            }
        }
        else if self.vel_x < 0.0 {

            let left = (((self.pos_x.floor() as u32 - 8_u32) as usize) & 0xfff0) / 16;
            let top = (((self.pos_y.floor() as u32) as usize) & 0xfff0) / 16;
            let bottom = (((self.pos_y.floor() as u32 - 8_u32) as usize) & 0xfff0) / 16;

            if tiles[64 * left + bottom] > 0 {

                if self.grounded {
                    let try_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * left + bottom] - 1) as usize][((self.pos_x.floor() as u32 - 8) % 16) as usize] as f32 - 1.0;

                    if self.pos_y + step_up_size > try_y {
                        self.pos_y = try_y;
                    }
                    else {
                        self.pos_x = (((left + 1) * 16) + 8) as f32;
                        self.vel_x = 0.0;
                    }
                }
                else {
                    self.pos_x = (((left + 1) * 16) + 8) as f32;
                    self.vel_x = 0.0;
                }
            }
        }
    }

    pub fn vertical_detection(&mut self, tiles: &[u32; 4096]) {

        //Floor check
        if self.vel_y < 0.0 {

            let right = (((self.pos_x.floor() as u32 + 7_u32) as usize) & 0xfff0) / 16;
            let mut bottom = (((self.pos_y.floor() as u32 - 15_u32) as usize) & 0xfff0) / 16;
            let left = (((self.pos_x.floor() as u32 - 8_u32) as usize) & 0xfff0) / 16;

            let mut hit_right = false;
            let mut hit_left = false;

            if tiles[64 * right + bottom] > 0 {

                self.pos_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * right + bottom] - 1) as usize][((self.pos_x.floor() as u32 + 7) % 16) as usize] as f32 - 1.0;
                self.vel_y = 0.0;
                self.grounded = true;
                hit_right = true;
            }
            if tiles[64 * left + bottom] > 0 {

                let temp_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * left + bottom] - 1) as usize][((self.pos_x.floor() as u32 - 8) % 16) as usize] as f32 - 1.0;
                if hit_right {
                    self.pos_y = temp_y.max(self.pos_y);
                }else {
                    self.pos_y = temp_y;
                }
                hit_left = true;
                self.vel_y = 0.0;
                self.grounded = true;
            }
            if !hit_left && !hit_right && self.grounded {

                if bottom > 0 {

                    bottom = bottom - 1;
                    
                    if tiles[64 * right + bottom] > 0 {

                        let temp_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * right + bottom] - 1) as usize][((self.pos_x.floor() as u32 + 7) % 16) as usize] as f32 - 1.0;
                        if self.pos_y - step_down_size < temp_y {
                            self.pos_y = temp_y;
                            self.vel_y = 0.0;
                            self.grounded = true;
                            hit_right = true;
                        }
                    }
                    if tiles[64 * left + bottom] > 0 {
        
                        let temp_y = (((bottom + 1) * 16) as f32) + tile_masks[(tiles[64 * left + bottom] - 1) as usize][((self.pos_x.floor() as u32 - 8) % 16) as usize] as f32 - 1.0;
                        if self.pos_y - step_down_size < temp_y {
                            if hit_right {
                                self.pos_y = temp_y.max(self.pos_y);
                            }else {
                                self.pos_y = temp_y;
                            }
                            hit_left = true;
                            self.vel_y = 0.0;
                            self.grounded = true;
                        }
                    }
                }
            }
            self.grounded = hit_left || hit_right;
        }
        else if self.vel_y > 0.0 { //Ceil check

            let right = (((self.pos_x.floor() as u32 + 7_u32) as usize) & 0xfff0) / 16;
            let top = (((self.pos_y.floor() as u32 + 8_u32) as usize) & 0xfff0) / 16;
            let left = (((self.pos_x.floor() as u32 - 8_u32) as usize) & 0xfff0) / 16;

            if tiles[64 * right + top] > 0 {

                self.pos_y = ((top) * 16 - 8) as f32;
                self.vel_y = 0.0;
            }
            else if tiles[64 * left + top] > 0 {

                self.pos_y = ((top) * 16 - 8) as f32;
                self.vel_y = 0.0;
            }
        }
    }

    pub fn get_position_x(&self) -> f32 {

        self.pos_x
    }

    pub fn get_position_y(&self) -> f32 {

        self.pos_y
    }

    pub fn render(&self, shader: &shader::Program) {

        shader.set_used();

        shader.set_int(self.frame as i32, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(18, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        //Floor pos to keep pixel perfect
        let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(self.pos_x.floor() - 24.0, self.pos_y.floor() - 24.0, 0.0));
        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        if self.facing {
            shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));
        }else {
            shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));
        }

        self.quad.render();
    }
}