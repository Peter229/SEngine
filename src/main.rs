extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate failure;
extern crate image;
extern crate cgmath;
extern crate rodio;

mod debug;
pub mod shader;
pub mod resources;
pub mod control_settings;
mod quad;
mod player;
mod level;
mod game;
mod online_player;
mod network;
mod network_game;
mod mario;
mod camera;
mod sound;
mod online_mario;
mod menu;
mod menu_network;

use failure::err_msg;
use crate::resources::Resources;
use std::path::Path;
use std::ffi;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::collections::HashMap;

enum ProgramState {
    MENU,
    MENU_NETWORK,
    OPTIONS,
    GAME,
    NETWORK_GAME,
}

fn main() {

    if let Err(e) = run() {

        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {

    let device = rodio::default_output_device().unwrap();
    let mut sound_manager = sound::Sound_Manager::new(device);
    //sound_manager.play_sound("track0", 0);
    //sound_manager.set_volume(0.3, 0);

    let mut width: i32 = 1024;
    let mut height: i32 = 768;
    let render_width: i32 = 256;
    let render_height: i32 = 192;

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let mut timer = sdl.timer().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("S_Engine", width as u32, height as u32).opengl().resizable().build().unwrap();

    sdl.mouse().show_cursor(true);

    let _gl_context = window.gl_create_context().map_err(err_msg)?;

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void );

    video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate).map_err(err_msg)?;

    //Resolution to render at
    let ortho = cgmath::ortho(0.0, render_width as f32, 0.0, render_height as f32, 0.0, 10.0);

    let eye = cgmath::Point3::new(0.0, 0.0, 1.0);
    let dir = cgmath::Vector3::new(0.0, 0.0, -1.0);
    let up = cgmath::Vector3::new(0.0, 1.0, 0.0);
    let view = cgmath::Matrix4::look_at_dir(eye, dir, up);
    let model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
    let view_projection = ortho * view;

    let quad_program = shader::Program::from_res(&res, "shaders/quad")?;
    let level_program = shader::Program::from_res(&res, "shaders/level")?;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    }

    //GAME STATE INIT
    let mut program_state = ProgramState::MENU;

    let mut old_time_ms = timer.ticks();

    let mut inputs_per_frame: Vec<HashMap<usize, bool>> = Vec::new();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut game = game::Game::new();
    let mut net_game = network_game::Game::new();
    let mut menu = menu::Menu::new();
    let mut net_menu = menu_network::Menu::new();

    let mut mouse_x = 0;
    let mut mouse_y = 0;
    let mut left_mouse = false;
    let mut right_mouse = false;

    let mut current_tile: u32 = 1;

    'main: loop {

        quad_program.set_used();
        quad_program.set_mat4_cg(&view_projection, ffi::CStr::from_bytes_with_nul(b"view_projection\0").expect("CStr::from_bytes_with_nul failed"));
        quad_program.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));
        level_program.set_used();
        level_program.set_mat4_cg(&view_projection, ffi::CStr::from_bytes_with_nul(b"view_projection\0").expect("CStr::from_bytes_with_nul failed"));
        level_program.set_mat4_cg(&model, ffi::CStr::from_bytes_with_nul(b"model\0").expect("CStr::from_bytes_with_nul failed"));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mut inputs_per_cycle = HashMap::new();

        for event in event_pump.poll_iter() {

            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), .. } => {
                    match program_state {
                        ProgramState::MENU => {
                            net_game.init_peer();
                            program_state = ProgramState::NETWORK_GAME;
                        },
                        ProgramState::NETWORK_GAME => {
                            net_game.add_player();
                        }
                        _ => {},
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::E), .. } => {
                    if current_tile < 5 {
                        current_tile += 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => {
                    if current_tile > 1 {
                        current_tile -= 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    inputs_per_cycle.insert(keycode as usize, true);
                },
                sdl2::event::Event::KeyUp { keycode: Some(keycode), .. } => {
                    inputs_per_cycle.insert(keycode as usize, false);
                },
                sdl2::event::Event::MouseMotion { x, y, .. } => { 
                    mouse_x = ((x as f32 / width as f32) * (render_width as f32)) as i32;
                    mouse_y = (render_height - 1) - ((y as f32 / height as f32) * (render_height as f32)) as i32;   
                },
                sdl2::event::Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => { 
                    left_mouse = true;
                },
                sdl2::event::Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => { 
                    left_mouse = false;
                },
                sdl2::event::Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Right, .. } => { 
                    right_mouse = true;
                },
                sdl2::event::Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Right, .. } => { 
                    right_mouse = false;
                },
                sdl2::event::Event::Window { 
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                    width = w;
                    height = h;
                },
                _ => {},
            }
        }

        match program_state {
            ProgramState::MENU => {
                if menu.update(&inputs_per_cycle) {
                    let state = menu.get_state();
                    if state == 0 {
                        program_state = ProgramState::GAME;
                    }
                    else if state == 1 {
                        program_state = ProgramState::MENU_NETWORK;
                    }
                    else if state == 3 {
                        break 'main;
                    }
                }else {
                    menu.render(&quad_program);
                }
            },
            ProgramState::MENU_NETWORK => {
                if net_menu.update(&inputs_per_cycle) {
                    let state = net_menu.get_state();
                    if state == 0 {
                        //program_state = ProgramState::GAME;
                    }
                    else if state == 1 {
                        program_state = ProgramState::NETWORK_GAME;
                    }
                    else if state == 2 {
                        program_state = ProgramState::NETWORK_GAME;
                    }
                    else if state == 3 {
                        program_state = ProgramState::MENU;
                    }
                }else {
                    net_menu.render(&quad_program);
                }
            },
            ProgramState::OPTIONS => {

            },
            ProgramState::GAME => {

                inputs_per_frame.push(inputs_per_cycle);

                game.mouse_update(mouse_x, mouse_y, left_mouse, right_mouse, current_tile);

                let now = timer.ticks();

                if now - old_time_ms > 16 {
                    //Called 60 times a second
                    old_time_ms = now;
                    game.update(&inputs_per_frame, &sound_manager);
                    inputs_per_frame.clear();
                }
                game.render(&quad_program, &level_program, ortho);
            },
            ProgramState::NETWORK_GAME => {

                inputs_per_frame.push(inputs_per_cycle);

                let now = timer.ticks();

                net_game.mouse_update(mouse_x, mouse_y, left_mouse, right_mouse, current_tile);

                net_game.check_updates(&sound_manager);

                if now - old_time_ms > 16 {
                    //Called 60 times a second
                    old_time_ms = now;
                    net_game.update(&inputs_per_frame, &sound_manager);
                    inputs_per_frame.clear();
                }
                net_game.render(&quad_program, &level_program, ortho);
            },
        }

        window.gl_swap_window();
    }

    Ok(())
}