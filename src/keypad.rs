extern crate sdl2;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::event::EventType;

pub struct Keypad {
    pub event_pump: sdl2::EventPump,
    pub key: [bool; 16],
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        let key = [false; 16];
        Keypad {
            event_pump: event_pump,
            key: key,
        }
    }
    pub fn mapping(keycode: Keycode) ->Option<u8> {
        let hex_key = match keycode {
            Keycode::Up => Some(0x0),
            Keycode::Down => Some(0x1),
            Keycode::Left => Some(0x2),
            Keycode::Right => Some(0x3),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0x7),
            Keycode::A => Some(0x8),
            Keycode::S => Some(0x9),
            Keycode::D => Some(0xA),
            Keycode::F => Some(0xB),
            Keycode::Z => Some(0xC),
            Keycode::X => Some(0xD),
            Keycode::C => Some(0xE),
            Keycode::V => Some(0xF),
            Keycode::Escape => Some(0xFF),
	        _ => None,
        }; 
        hex_key
    }
    pub fn get_input(&mut self) -> bool {
        let mut up_or_down: bool;
        let mut keycode: Keycode;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(x), .. } => {
                    up_or_down = true;
                    keycode = x;
                },
                Event::KeyUp { keycode: Some(x), .. } => {
                    up_or_down = false;
                    keycode = x;
                },
                _ => continue,
            }
            match Keypad::mapping(keycode) {
                Some(hex) => 
                    match hex {
                        0xFF => return false,
                        _ => {
                            self.key[hex as usize] = up_or_down;
                        },
                },
                None => (),
            }   
        }
        true
    }
    pub fn wait_for_input(&mut self) -> u8 {
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode:Some(x), ..  } => {
                        match Keypad::mapping(x) {
                           Some(x) => return x,
                           None => (),
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}

