extern crate sdl2;
extern crate rand;
use super::keypad::Keypad;
use super::display::Display;
use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub struct CPU {
    memory: [u8; 4096],
    opcode: u16,
    V: [u8; 16],
    Index: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    pub drawFlag: bool,
    pub display: Display,
    pub keypad: Keypad,
}

impl CPU {
    pub fn new() -> Self {
        let pc = 0x0200;
        let opcode = 0;
        let Index = 0;
        let stack = vec![];
        let mut memory = [0x00; 4096];
        let delay_timer = 0;
        let sound_timer = 0;
        let V = [0x00; 16];
        let sdl_context = sdl2::init().unwrap();
        let display = Display::new(&sdl_context);
        let keypad = Keypad::new(&sdl_context); 
        let fontset = vec![
             0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
             0x20, 0x60, 0x20, 0x20, 0x70, // 1
             0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
             0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
             0x90, 0x90, 0xF0, 0x10, 0x10, // 4
             0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
             0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
             0xF0, 0x10, 0x20, 0x40, 0x40, // 7
             0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
             0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
             0xF0, 0x90, 0xF0, 0x90, 0x90, // A
             0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
             0xF0, 0x80, 0x80, 0x80, 0xF0, // C
             0xE0, 0x90, 0x90, 0x90, 0xE0, // D
             0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
             0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for i in 0x00..0x4F {
            memory[0x50 + i] = fontset[i];
        }
        CPU {
            opcode: opcode,
            memory: memory,
            pc: pc,
            stack: stack,
            Index: Index,
            delay_timer: delay_timer,
            sound_timer: sound_timer,
            V: V,
            drawFlag: false,
            display: display,
            keypad: keypad,
        }
    }
    pub fn load_game(&mut self, game: &String) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut offset = 0x0;
	    let mut f = File::open(game).expect("COULD NOT OPEN GAME");
	    f.read_to_end(&mut buffer).expect("COULD NOT READ GAME");
        for element in buffer {
            self.memory[(0x200+offset) as usize] = element;
            offset += 1;
        }
        println!("LOADED");
    }
    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize+1] as u16);
        let NNN = self.opcode & 0x0FFF;
        let NN = (self.opcode & 0x00FF) as u8;
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        let op1 = (self.opcode & 0xF000) >> 12;
        let op2 = (self.opcode & 0x0F00) >> 8;
        let op3 = (self.opcode & 0x00F0) >> 4;
        let op4 = self.opcode & 0x000F;
        println!("{}: {:X}",self.pc,self.opcode);
        match (op1,op2,op3,op4) {
            (0x0,0x0,0xE,0x0) => {
                self.display.clear_screen();
                self.pc += 2;
            },
            (0x0,0x0,0xE,0xE) => match self.stack.pop() {
                Some(popped) => {self.pc = popped;println!("POPPED")},
                None => println!("NONE"),
            },
            (0x1,_,_,_) => self.pc = NNN,
            (0x2,_,_,_) => {
                println!("PUSH");
                self.stack.push(self.pc); 
                self.pc = NNN;
            },
            (0x3,_,_,_) => {
                 if self.V[x] == NN {
                    self.pc = self.pc + 4;
                 }
                 else  {
                    self.pc = self.pc + 2;
                 }
            },
            (0x4,_,_,_) => {
                if self.V[x] != NN {
                    self.pc = self.pc + 4;
                } 
                else { 
                    self.pc = self.pc + 2; 
                }
            },
            (0x5,_,_,0x0) => {
                if self.V[x] == self.V[y] {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            (0x6,_,_,_) => {
                self.V[x] = NN;
                self.pc += 2;
            },
            (0x7,_,_,_) => {
                self.V[x] += NN;
                self.pc += 2;
            },
            (0x8,_,_,0x0) => {
                self.V[x] = self.V[y];
                self.pc += 2;
            },
            (0x8,_,_,0x1) => { 
                self.V[x] = self.V[x] | self.V[y]; 
                self.pc += 2;
            },
            (0x8,_,_,0x2) => {self.V[x] = self.V[x] & self.V[y]; self.pc += 2;},
            (0x8,_,_,0x3) => {self.V[x] = self.V[x] ^ self.V[y]; self.pc +=2;},
            (0x8,_,_,0x4) =>{
                        if self.V[y] > (0xFF - self.V[x]) { 
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[x] += self.V[y];
                        self.pc += 2;
                    },
            (0x8,_,_,0x5) => {
                        if self.V[x] <  self.V[y] {
                            self.V[0xF] = 0;
                        }
                        else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] -= self.V[y];
                        self.pc += 2;
                    },
            (0x8,_,_,0x6) => {
                self.V[0xF] = self.V[y] & 0x01;
                self.V[x] = self.V[y] >> 1;
                self.pc += 2;
            },
            (0x8,_,_,0x7) => {
                        if self.V[y] < self.V[x] {
                            self.V[0xF] = 0;
                        } 
                        else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                        self.pc += 2;
            },
            (0x8,_,_,0xE) => {
                self.V[0xF] = ((self.V[y] & 0x10) >> 7);
                self.V[x] = self.V[y] << 1;
                self.pc += 2;
            },
            (0x9,_,_,_) => {
                if self.V[x] == self.V[y] {
                    self.pc = self.pc + 2;
                }
                else {
                    self.pc = self.pc + 4;
                }
            },
            (0xA,_,_,_) => {
                self.Index = NNN;
                self.pc += 2;
            },
            (0xB,_,_,_) => self.pc = NNN + self.V[0x0] as u16,
            (0xC,_,_,_) => {
                self.V[x] = NN & (rand::random::<u8>() as u8);
                self.pc += 2;
            },
            (0xD,_,_,_) => {
                let coord_x: u16 = self.V[x] as u16;
                let coord_y: u16 = self.V[y] as u16;
                let height = (self.opcode & 0x000F) as u8;
                for row in 0..height {
                    let pixel = self.memory[(self.Index + row as u16) as usize]; 
                    for column in 0..8 {
                        if (pixel & (0x80 >> column)) != 0x00 {
                            println!("{}",coord_y+row as u16);
                            if self.display.gfx[((coord_x+column as u16) + ((coord_y+row as u16) * 64)) as usize] == true {
                                self.V[0xF] = 0x1;
                            } // casting to u16 because of multiply overflow in u8
                            self.display.gfx[((coord_x+column as u16)+ ((coord_y+row as u16) * 64)) as usize] ^= true; 
                        }
                    }
                }
                self.drawFlag = true;
                self.pc += 2;
            },
            (0xE,_,0x9,0xE) => {
                //skips the next instruction if the key stored in VX is pressed.
                match self.keypad.key[(self.V[x] >> 4) as usize] {
                    true => self.pc += 4,
                    false => self.pc += 2,
                }
            },
            (0xE,_,0xA,0x1) => {
                //Skips the next instruction if the key stored in VX isn't pressed
                match self.keypad.key[(self.V[x] >> 4) as usize] {
                    true => self.pc += 2,
                    false => self.pc += 4,
                }
            },
            (0xF,_,0x0,0x7) => {
                self.V[x] = self.delay_timer;
                self.pc += 2;
            },
            (0xF,_,0x0,0xA) => {
                self.V[x] = self.keypad.wait_for_input();
                self.pc += 2;
            },
            (0xF,_,0x1,0x8) => {
                self.sound_timer = self.V[x];
                self.pc += 2;
            },
            (0xF,_,0x1,0x5) => {
                self.delay_timer = self.V[x];
                self.pc += 2;
            },
            (0xF,_,0x1,0xE) => {
                self.Index += self.V[x] as u16;
                self.pc += 2;
            },
            (0xF,_,0x2,0x9) => {
                self.Index = self.V[x] as u16;
                self.pc += 2;
            },
            (0xF,_,0x3,0x3) => {
                self.memory[(self.Index) as usize] = self.V[x] / 100;
                self.memory[(self.Index + 0x1) as usize] = (self.V[x] / 10) % 10;
                self.memory[(self.Index + 0x2) as usize] = self.V[x] % 10; 
                self.pc += 2;
            },
            (0xF,_,0x5,0x5) => {
                let mut offset = self.Index as usize;
                for i in 0x0..x {
                    if offset > 4096 {
                        panic!("MEMORY CORRUPTION... EXITING");
                    }
                    self.memory[offset] = self.V[i];
                    offset += 1;
                }
                self.pc += 2;
            },
            (0xF,_,0x6,0x5) => {
                let mut offset = self.Index as usize;
                for i in 0x0..x {
                    if offset > 4096 {
                        panic!("MEMORY CORRUPTION... EXITING");
                    }
                    self.V[i] = self.memory[offset]; 
                    offset += 1;
                }
                self.pc += 2;
            },
            _ => println!("INVALID OPCODE: {:X}",self.opcode),
        }
        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }
        match self.sound_timer {
            1 => {
                println!("BEEP!");
                self.sound_timer = self.sound_timer - 1
            },
            _ => (),
        } 
    }
}


