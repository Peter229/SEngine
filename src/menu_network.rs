use crate::quad;
use crate::shader;
use crate::control_settings;
use crate::text;
use std::ffi;
use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr, SocketAddrV4};

pub struct Menu {

    pos: u32,
    select: bool,
    quad: quad::Quad,
    cursor: quad::Quad,
    text: text::Text,
}

impl Menu {

    pub fn new() -> Menu {

        let quad = quad::Quad::new([0.0, 0.0], [256.0, 192.0], "assets/textures/menu_net.png".to_string());
        let cursor = quad::Quad::new([0.0, 0.0], [10.0, 10.0], "assets/textures/select.png".to_string());
        let text = text::Text::new();

        Menu { pos: 0, select: false, quad, cursor, text }
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
                    else if self.pos == 0 {
                        self.text.edit(*key as u32);
                    }
                }
            }
        }

        self.select
    }

    pub fn get_ip(&self) -> SocketAddr {

        let ip4: Ipv4Addr = self.text.get_text_as_string().parse().unwrap();
        let socket_addr: SocketAddr = SocketAddr::new(IpAddr::V4(ip4), 3456);
        socket_addr
    }

    pub fn get_state(&self) -> u32 {

        self.pos
    }

    pub fn render(&self, quad_shader: &shader::Program, level_program: &shader::Program) {

        level_program.set_used();
        self.text.render(level_program);

        quad_shader.set_used();

        quad_shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        quad_shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        let mut model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(12.0, 119.0 - ((self.pos * 20) as f32), 0.0));

        quad_shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        quad_shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));

        self.cursor.render();

        quad_shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"posInImage\0").expect("CStr::from_bytes_with_nul failed"));
        quad_shader.set_int(1, ffi::CStr::from_bytes_with_nul(b"sizeOfImage\0").expect("CStr::from_bytes_with_nul failed"));
        model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
        quad_shader.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        quad_shader.set_int(0, ffi::CStr::from_bytes_with_nul(b"flip\0").expect("CStr::from_bytes_with_nul failed"));

        self.quad.render();
    }
}