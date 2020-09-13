use sdl2;

pub static mut SENSITIVITY: f32 = 1.0;

//https://wiki.libsdl.org/SDLKeycodeLookup - 1073741753

pub static mut ENTER: usize = sdl2::keyboard::Keycode::Return as usize;
pub static mut BACKSPACE: usize = sdl2::keyboard::Keycode::Backspace as usize;
//pub static mut SPACE: usize = sdl2::keyboard::Keycode::Space as usize;

pub static mut UP: usize = sdl2::keyboard::Keycode::W as usize;
pub static mut DOWN: usize = sdl2::keyboard::Keycode::S as usize;
pub static mut LEFT: usize = sdl2::keyboard::Keycode::A as usize;
pub static mut RIGHT: usize = sdl2::keyboard::Keycode::D as usize;
pub static mut JUMP: usize = sdl2::keyboard::Keycode::Space as usize;
pub static mut DUCK: usize = sdl2::keyboard::Keycode::Kp2 as usize;

pub enum Player_State {
    RIGHT,
    LEFT,
    JUMP,
    DUCK,
}

pub fn change_sensitivity(new_sensitivity: f32) {

    unsafe {
        SENSITIVITY = new_sensitivity;
    }
}