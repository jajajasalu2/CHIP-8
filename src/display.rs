extern crate sdl2;
use sdl2::Sdl;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

pub struct Display {
    pub gfx: [[bool; 64];32],
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8",64,32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        Display {
            gfx: [[false;64];32],
            canvas: canvas,
        }
    } 
    pub fn clear_screen(&mut self) {
        self.gfx = [[false;64];32];
        self.draw();
    }
    pub fn draw(&mut self) {
        for row in 0..31 {
            for column in 0..63 {
                let point = Point::new(row,column);
                if self.gfx[row as usize][column as usize] {
                    self.canvas.set_draw_color(Color::RGB(255,255,255));
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(0,0,0));
                }
                self.canvas.draw_point(point);
            }
        }
        self.canvas.clear();
        self.canvas.present();
    }
}
