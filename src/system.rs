extern crate chip8;
struct System {
    cpu: chip8::CPU,
    keypad: chip8::Keypad,
}
impl System {
    fn new() -> Self {
        System {
            cpu: chip8::CPU::new(),
            keypad: chip8::Keypad::new(),
        }
    }

}


