extern crate chip8;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = &args[1];
    let mut system = chip8::cpu::CPU::new();
    system.load_game(game);
    loop {
        system.emulate_cycle(); 
        if system.drawFlag {
            system.display.draw();
            system.drawFlag = false;
        }
        if !system.keypad.get_input() {
            break;
        }
    }
}
