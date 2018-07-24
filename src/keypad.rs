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
    pub fn mapping(keycode: Keycode) -> u8 {
        let mut hex_key = match keycode {
            Keycode::Kp1 => 0x0,
            Keycode::Kp2 => 0x1,
            Keycode::Kp3 => 0x2,
            Keycode::Kp4 => 0x3,
            Keycode::Q => 0x4,
            Keycode::W => 0x5,
            Keycode::E => 0x6,
            Keycode::R => 0x7,
            Keycode::A => 0x8,
            Keycode::S => 0x9,
            Keycode::D => 0xA,
            Keycode::F => 0xB,
            Keycode::Z => 0xC,
            Keycode::X => 0xD,
            Keycode::C => 0xE,
            Keycode::V => 0xF,
	    _ => panic!("INVALID INPUT"),
        }; 
        hex_key
    }
    pub fn set_keys(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(x), .. } => self.key[Keypad::mapping(x) as usize] = true,
                Event::KeyUp { keycode: Some(x), .. } => self.key[Keypad::mapping(x) as usize] = false,
                _ => (),
            }
        }
    }
    pub fn wait_for_input(&mut self) -> u8 {
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode:Some(x), ..  } => {
                        return Keypad::mapping(x);
                    },
                    _ => (),
                }
            }
        }
    }
}

