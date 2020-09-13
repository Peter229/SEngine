use crate::quad;
use crate::shader;
use crate::control_settings;
use std::ffi;
use std::collections::HashMap;

pub struct Menu {

    pos: u32,
    select: bool,
    quad: quad::Quad,
    cursor: quad::Quad,
}

impl Menu {

    pub fn new() -> Menu {

        let quad = quad::Quad::new([0.0, 0.0], [256.0, 192.0], "assets/textures/menu_net.png".to_string());
        let cursor = quad::Quad::new([0.0, 0.0], [10.0, 10.0], "assets/textures/select.png".to_string());

        Menu { pos: 0, select: false, quad, cursor }
    }

    pub fn update(&mut self, keys: &HashMap<usize, bool>) -> bool {

        self.select = false;

        for (key, val) in keys.iter() {
            //On Press
            if *val {
                unsafe {
                    if *key == control_settings::UP {
                        if self.pos > 0 {
                            self.pos -= 1;
                        }
                    }
                    else if *key == control_settings::DOWN {
                        if self.pos < 3 {
                            self.pos += 1;
                        }
                    }
                    else if *key == control_settings::ENTER {
                        self.select = true;
                    }
                }
            }
        }

        self.select
    }

    pub fn get_state(&self) -> u32 {

        self.pos
    }

    pub fn render(&self, shader: &shader::Program) {

        shader.set_used();

        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        let mut model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(12.0, 119.0 - ((self.pos * 20) as f32), 0.0));

        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));

        self.cursor.render();

        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
        shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));

        self.quad.render();
    }
}