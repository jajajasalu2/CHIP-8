extern crate sdl2;
use sdl2::Sdl;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

pub struct Display {
    pub gfx: [bool; 2048],
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8",640,320)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas()
            .build()
            .unwrap();
        canvas.set_scale(10.0,10.0);
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();
        Display {
            gfx: [false;2048],
            canvas: canvas,
        }
    } 
    pub fn clear_screen(&mut self) {
        self.gfx = [false; 2048];
        self.draw();
    }
    pub fn draw(&mut self) {
       self.canvas.clear();
       for row in 0..32 {
            let offset = 32 * row; 
            for column in 0..64 {
                if self.gfx[offset + column] {
                    self.canvas.set_draw_color(Color::RGB(255,255,255));
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(0,0,0));
                }
                self.canvas.draw_point(Point::new(column as i32, row as i32));
            }
        }
        self.canvas.present();
    }
}
