use crate::mario;
use crate::online_mario;
use crate::level;
use crate::shader;
use crate::camera;
use crate::sound;
use crate::network;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{self, Read, Error, stdin, stdout, Write, ErrorKind};
use std::collections::HashMap;
use cgmath;
use std::ffi;

pub struct Game {

    player: mario::Player,
    players: HashMap<std::net::SocketAddr, online_mario::Player>,
    level: level::Level,
    camera: camera::Camera,
    network: network::Network,
}

impl Game {

    pub fn new() -> Game {

        let mut player = mario::Player::new();
        let mut players: HashMap<std::net::SocketAddr, online_mario::Player> = HashMap::new();
        let mut level = level::Level::new();
        let mut camera = camera::Camera::new();
        let mut network = network::Network::quick_new();

        Game { player, players, level, camera, network, }
    }

    pub fn start(&mut self) {

        //self.sound_manager.play_sound("track0", 0);
        //self.sound_manager.set_volume(0.5, 0);
    }

    pub fn init_peer(&mut self) {

        self.network = network::Network::new();
    }

    pub fn add_player(&mut self) {

        let mut s=String::new();
        print!("Please enter the ip of the person you want to connect to: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        let socket: SocketAddr = (s.trim_right()).parse().unwrap();

        self.players.insert(socket, online_mario::Player::new());
    }

    pub fn add_player_fast(&mut self, socket_addr: SocketAddr) {

        self.players.insert(socket_addr, online_mario::Player::new());
    }

    pub fn request_sync(&mut self, socket_addr: SocketAddr) {

        let mut buffer: [u8; 16] = [0; 16];
        buffer[0] = 245;
        self.network.send_inputs(&buffer, socket_addr.clone());
        println!("Request to sync sent");
    }

    pub fn check_updates(&mut self, sound_manager: &sound::Sound_Manager) {

        let mut come_through = self.network.recieve();
        if come_through.recieve {
        
            //Update world
            if come_through.buffer[0] == 244 {
                self.online_mouse_update(&come_through.buffer);
            }
            else if come_through.buffer[0] == 245 {

                println!("HOST: Recieved sync request");

                //Send sync info
                let mut buffer: [u8; 16] = come_through.buffer;
                buffer[0] = 246;
                let mut new_socket = come_through.from_socket.ip().to_string();
                
                let new_socket_parts: Vec<&str> = new_socket.split(".").collect();
                //let new_socket_length = b.len();
                //buffer[1] = new_socket_length as u8;
                let mut pos = 1;
                for part in new_socket_parts {
                    buffer[pos] = part.parse::<u8>().unwrap();
                    pos += 1;
                }

                println!("{:?}", buffer);

                for socket in self.players.keys() {
                    //Dont send it init sync back to person who wants sync
                    if socket.ip() != come_through.from_socket.ip() {
                        self.network.send_inputs(&buffer, socket.clone());
                    }
                }

                let mut buffer_two: [u8; 16] = [0; 16];
                buffer_two[0] = 247;
                buffer_two[1] = (((self.player.get_position_x().floor() as usize) & 0xfff0) / 16) as u8;
                buffer_two[2] = ((self.player.get_position_x().floor() as usize) % 16) as u8;
                buffer_two[3] = (((self.player.get_position_y().floor() as usize) & 0xfff0) / 16) as u8;
                buffer_two[4] = ((self.player.get_position_y().floor() as usize) % 16) as u8;
                self.network.send_inputs(&buffer_two, come_through.from_socket.clone());

                if !self.players.contains_key(&come_through.from_socket) {

                    self.players.insert(come_through.from_socket, online_mario::Player::new());
                }
            }
            else if come_through.buffer[0] == 246 {

                println!("Resyncing with other players");
                let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(come_through.buffer[1], come_through.buffer[2], come_through.buffer[3], come_through.buffer[4])), 3456);

                println!("Socket that needs syncing with {}", socket_addr);

                let mut buffer_two: [u8; 16] = [0; 16];
                buffer_two[0] = 247;
                buffer_two[1] = (((self.player.get_position_x().floor() as usize) & 0xfff0) / 16) as u8;
                buffer_two[2] = ((self.player.get_position_x().floor() as usize) % 16) as u8;
                buffer_two[3] = (((self.player.get_position_y().floor() as usize) & 0xfff0) / 16) as u8;
                buffer_two[4] = ((self.player.get_position_y().floor() as usize) % 16) as u8;
                self.network.send_inputs(&buffer_two, socket_addr.clone());

                if !self.players.contains_key(&socket_addr) {

                    self.players.insert(socket_addr, online_mario::Player::new());
                }
            }
            else if come_through.buffer[0] == 247 {

                println!("Sync confirmed");
                let mut pos_x: u32 = (((come_through.buffer[1] as u32) * 16_u32) + (come_through.buffer[2] as u32));
                let mut pos_y: u32 = (((come_through.buffer[3] as u32) * 16_u32) + (come_through.buffer[4] as u32));

                if self.players.contains_key(&come_through.from_socket) {

                    self.players.get_mut(&come_through.from_socket).unwrap().update_pos(pos_x, pos_y);
                }
                else {
                    self.players.insert(come_through.from_socket, online_mario::Player::new_pos(pos_x, pos_y));
                }
            }
            else {
                //Game Update
                if self.players.contains_key(&come_through.from_socket) {
                    self.players.get_mut(&come_through.from_socket).unwrap().set_key_state(&come_through.buffer);
                    self.players.get_mut(&come_through.from_socket).unwrap().update(&self.level.get_tiles(), sound_manager);
                }
                else {
                    self.players.insert(come_through.from_socket, online_mario::Player::new());
                }
            }
        }
    }

    pub fn update(&mut self, keys: &Vec<HashMap<usize, bool>>, sound_manager: &sound::Sound_Manager) {

        self.player.inputs(keys);
        self.player.update(&self.level.get_tiles(), sound_manager);
        for socket in self.players.keys() {
            self.network.send_inputs(&self.player.online_state_buffer(), socket.clone());
        }
        self.camera.pos_x = self.player.get_position_x() - (256.0 / 2.0);
        self.camera.pos_y = self.player.get_position_y() - (192.0 / 3.0);
        self.camera.update();
    }

    pub fn render(&mut self, quad_shader: &shader::Program, level_shader: &shader::Program, ortho: cgmath::Matrix4<f32>) {

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
        for player in self.players.values_mut() {
            player.render(quad_shader);
        }
        self.level.render(level_shader);
    }

    pub fn online_mouse_update(&mut self, keys: &[u8; 16]) {

        let left_mouse = keys[1] == 1;
        let right_mouse = keys[2] == 1;
        let tile_pos_x = keys[3] as usize;
        let tile_pos_y = keys[4] as usize;
        let current_tile = keys[5] as u32;
        self.level.mouse_edit(tile_pos_x, tile_pos_y, left_mouse, right_mouse, current_tile);
    }

    pub fn mouse_update(&mut self, mouse_x: i32, mouse_y: i32, left_mouse: bool, right_mouse: bool, current_tile: u32) {

        let tile_pos_x = (((mouse_x as usize) + (self.camera.pos_x as usize)) & 0xfff0) / 16;
        let tile_pos_y = (((mouse_y as usize) + (self.camera.pos_y as usize)) & 0xfff0) / 16;
        //Stop the squad from destroying the floor :(
        if tile_pos_x != 0 && tile_pos_x != 63 && tile_pos_y != 0 && tile_pos_y != 63 {
            self.level.mouse_edit(tile_pos_x, tile_pos_y, left_mouse, right_mouse, current_tile);
            if left_mouse || right_mouse {
                let mut buffer: [u8; 16] = [0; 16];
                buffer[0] = 244;
                if left_mouse {
                    buffer[1] = 1;
                }else {
                    buffer[1] = 0;
                }
                if right_mouse {
                    buffer[2] = 1;
                }else {
                    buffer[2] = 0;
                }
                buffer[3] = tile_pos_x as u8;
                buffer[4] = tile_pos_y as u8;
                buffer[5] = current_tile as u8;
                for socket in self.players.keys() {
                    self.network.send_inputs(&buffer, socket.clone());
                }
            }
        }
    }
}