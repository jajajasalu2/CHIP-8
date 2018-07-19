extern crate rand;
use rand::Rng;
use chip8::Keypad;

pub struct CPU {
    memory: [u8; 4096],
    opcode: u16,
    V: [u8; 16],
    Index: u16,
    pc: u16,
    gfx: [[bool; 32];64],
    delay_timer: char,
    sound_timer: char,
    stack: Vec<u16>,
    keypad: chip8::Keypad,
}

impl CPU {
    fn new() -> Self {
        let pc = 0x0200;
        let opcode = 0;
        let Index = 0;
        let stack = vec![];
        let memory = [0x00; 4096];
        let gfx = [[false;32];64];
        let delay_timer = 0;
        let sound_timer = 0;
        let V = [0x00; 16];
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
            memory: memory,
            pc: pc,
            stack: stack,
            Index: Index,
            gfx: gfx,
            delay_timer: delay_timer,
            sound_timer: sound_timer,
            V: V,
            keypad: chip8::Keypad::new(),
        }
    }
    fn initialize(&mut self) {
        self.pc = 0x200;
        self.opcode = 0;
        self.Index = 0;
        for i in 0..79 {
            self.memory[i] = self.fontset[i];
        }
    }
    fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize+1] as u16);
        let NNN = self.opcode & 0x0FFF;
        let NN = (self.opcode & 0x00FF) as u8;
        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
        let (op1,op2,op3,op4) = ((self.opcode & 0xF000)>>12,self.opcode & 0x0F00 >> 8,self.opcode & 0x00F0 >> 4 , self.opcode & 0x000F);
        match (op1,op2,op3,op4) {
            (0x0,0x0,0xE,0x0) => {
                //Clear screen
            },
            (0x0,0x0,0xE,0xE) => match self.stack.pop() {
                Some(popped) => self.pc = popped,
                None => (),
            },
            (0x0,_,_,_) => {
                //Calls RCA 1802 program (?) at address NNN
            },
            (0x1,_,_,_) => self.pc = NNN,
            (0x2,_,_,_) => {
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
                    self.pc = self.pc + 4; } 
                else { self.pc = self.pc + 2; }
            },
            (0x5,_,_,_) => {
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
            (0x8,_,_,0x1) => self.V[x] = self.V[x] | self.V[y],
            (0x8,_,_,0x2) => self.V[x] = self.V[x] & self.V[y],
            (0x8,_,_,0x3) => self.V[x] = self.V[x] ^ self.V[y],
            (0x8,_,_,0x4) =>{
                        if self.V[y] > (0xFF - self.V[x]) { 
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[x] = self.V[x] + self.V[y];
                    },
            (0x8,_,_,0x5) => {
                        if self.V[x] <  self.V[y] {
                            self.V[0xF] = 0;
                        }
                        else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                    },
            (0x8,_,_,0x6) => {
                self.V[0xF] = self.V[y] & 0x01;
                self.V[x] = self.V[y] >> 1;
            },
            (0x8,_,_,0x7) => {
                        if self.V[y] < self.V[x] {
                            self.V[0xF] = 0;
                        } 
                        else {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
            },
            (0x8,_,_,0xE) => {
                self.V[0xF] = self.V[y] & 0x10;
                self.V[x] = self.V[y] << 1;
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
                self.pc = self.pc + 2;
            },
            (0xB,_,_,_) => self.pc = NNN + self.V[0x0] as u16,
            (0xC,_,_,_) => {
                self.V[x] = NN & (rand::thread_rng().gen_range(0,255));
                self.pc = self.pc + 2;
            },
            (0xD,_,_,_) => {
                let coord_x = self.V[x];
                let coord_y = self.V[y];
                let height = self.opcode & 0x000F;
                for row in 0..height-1 {
                    let pixel = self.memory[self.Index + row];
                    for column in 0..7 {
                        if pixel & (0x80 >> column) != 0x00 {
                            if self.gfx[coord_x+column][coord_y+row] == 1 {
                                self.V[0xF] = 0x1;
                            }
                            self.gfx[coord_x+column][coord_y+row] ^= 1; 
                        }
                    }
                }
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
                self.V[x] = self.keypad.await_input();
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
                /*Sets I to the location of the sprite for the character in VX. 
                 *Characters 0-F (in hexadecimal) are represented by a 4x5 font. */
                self.Index = self.V[x] as u16;
                self.pc += 2;
            },
            (0xF,_,0x3,0x3) => {
                self.memory[self.Index] = self.V[x] / 100;
                self.memory[self.Index + 0x1] = (self.V[x] / 10) % 10;
                self.memory[self.Index + 0x2] = self.V[x] % 10; 
                self.pc += 2;
            },
            (0xF,_,0x5,0x5) => {
                let mut offset = self.Index;
                for i in (0x0..x) {
                    if offset > 4096 {
                        panic!("MEMORY CORRUPTION... EXITING");
                    }
                    self.memory[offset] = self.V[i];
                    offset += 1;
                }
                self.pc += 2;
            },
            (0xF,_,0x6,0x5) => {
                let mut offset = &self.Index;
                for i in (0x0..x) {
                    if offset > 4096 {
                        panic!("MEMORY CORRUPTION... EXITING");
                    }
                    self.V[i] = self.memory[offset]; 
                    offset += 1;
                }
                self.pc += 2;
            },
            _ => panic!(format!("INVALID OPCODE: {}",self.opcode)),
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


